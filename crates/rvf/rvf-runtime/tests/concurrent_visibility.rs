//! ADR-0163 — concurrent-writer visibility race in `RvfStore::boot()`.
//!
//! This test reproduces the data-loss shape behind the t3-2-concurrent
//! acceptance regression: N writers each ingest one entry, but a fresh
//! reader observes only N-1 entries.
//!
//! Mechanism (Window α refined): writers running in different processes
//! each open the file fresh and assign per-process numeric vector IDs
//! starting from 1 (the JS-side `nextNativeId` counter). The kernel
//! `flock(LOCK_EX)` on `<file>.lock` serialises the writers, and each
//! writer's `boot()` reads the latest manifest before assigning new
//! IDs — so under faithful sequential semantics the per-process counters
//! advance past prior writers' IDs.
//!
//! BUT: the JS-side `nativeIdMap` / `nextNativeId` is initialised from
//! `_reserveAssignedNativeId` calls during `loadFromNativeSegments`, which
//! reads `iter_metadata_with_vectors` snapshots. If a fresh JS process
//! starts BEFORE writing, `nextNativeId=1`. With the kernel flock taken at
//! `RvfStore::open` time, two writers can each enter assuming their
//! private state is correct; the second-to-acquire DOES read the first's
//! META_SEG via `boot()` but only inside the rust runtime — the JS layer's
//! `nextNativeId` was set BEFORE the native open completed, off the cached
//! state from the previous JS-side call (or initialised to 1 if first
//! call). The writers then call `ingest_batch(_, [numId=1], _)` from two
//! different processes — same vid.
//!
//! Two `(VEC_SEG, META_SEG)` pairs both keyed on vid=1 land in the file.
//! `boot()` populates `vectors` and `metadata_full` `HashMap`s in segment
//! order; later inserts overwrite earlier ones for the same key. A fresh
//! reader observes exactly ONE entry for vid=1 even though two distinct
//! `(stringId, embedding, metadata)` payloads were durably written.
//!
//! ## What this test proves
//!
//! Test 1 (single-process duplicate-vid replay): proves the BOOT
//! mechanism: when the same vid appears in multiple META_SEGs across
//! multiple ingest_batches, `boot()` keeps only the last by HashMap
//! overwrite. Directly reproduces "N entries durable, N-1 visible" without
//! needing concurrency.
//!
//! Test 2 (sequential reopen + same-vid): two `RvfStore::open` lifetimes
//! in the same process, each ingesting with vid=1 and a different
//! stringId in metadata. Confirms boot-side overwrite happens across
//! manifest commits, not just within one ingest_batch call.
//!
//! Test 3 (multi-process subprocess ingest): the realistic cross-process
//! reproduction. Spawns N child processes, each calling
//! `RvfStore::open` + `ingest_batch` with a hard-coded vid. The kernel
//! flock serialises them. We then open the file in the parent and check
//! how many distinct entries are visible. If the JS-side caller naively
//! supplied colliding vids, the count would be < N.
//!
//! Test 3 also gates on the fix: with the proposed fix (RvfStore-side
//! refusal of caller-supplied vids that collide with on-disk metadata),
//! the second writer's call with a colliding vid must FAIL loud rather
//! than silently overwriting.

use rvf_runtime::options::{
    DistanceMetric, MetadataEntry, MetadataValue, RvfOptions,
};
use rvf_runtime::store::RvfStore;
use std::path::Path;
use tempfile::TempDir;

const DIM: u16 = 4;

fn opts() -> RvfOptions {
    RvfOptions {
        dimension: DIM,
        metric: DistanceMetric::L2,
        ..Default::default()
    }
}

fn meta_string(field_id: u16, s: &str) -> MetadataEntry {
    MetadataEntry {
        field_id,
        value: MetadataValue::String(s.to_string()),
    }
}

fn ingest_one(path: &Path, vid: u64, payload: &str) {
    let mut store = if path.exists() {
        RvfStore::open(path).expect("open existing")
    } else {
        RvfStore::create(path, opts()).expect("create new")
    };
    let vec = vec![1.0f32, 2.0, 3.0, 4.0];
    let meta = vec![meta_string(0, payload)];
    store
        .ingest_batch(&[&vec], &[vid], Some(&meta))
        .expect("ingest");
    drop(store);
}

fn count_distinct_payloads(path: &Path) -> Vec<(u64, String)> {
    let store = RvfStore::open_readonly(path).expect("open ro");
    let mut out: Vec<(u64, String)> = Vec::new();
    for (id, _vec, entries) in store.iter_metadata_with_vectors() {
        for e in entries {
            if let MetadataValue::String(s) = &e.value {
                out.push((id, s.clone()));
            }
        }
    }
    out.sort_by_key(|(id, _)| *id);
    out
}

/// Test 1 — Single-process duplicate-vid (sequential, same-process).
///
/// Two ingest_batch calls on the same store with the same vid produce two
/// META_SEGs for that vid. `boot()` (on a fresh reader) sees both entries
/// in the manifest's segment_dir. Both get loaded into `metadata_full`,
/// the second overwriting the first. Reader observes ONE entry — the
/// later one.
#[test]
fn duplicate_vid_within_single_store_overwrites_in_boot() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("dup_vid_single.rvf");

    {
        let mut store = RvfStore::create(&path, opts()).unwrap();
        let vec = vec![1.0f32, 2.0, 3.0, 4.0];
        // First ingest with vid=1, payload="alice"
        store
            .ingest_batch(&[&vec], &[1u64], Some(&[meta_string(0, "alice")]))
            .unwrap();
        // Second ingest, SAME vid=1, different payload. This is what
        // happens when two cross-process writers both naively assign
        // numId=1 starting from a per-process counter at 1.
        store
            .ingest_batch(&[&vec], &[1u64], Some(&[meta_string(0, "bob")]))
            .unwrap();
    }

    let observed = count_distinct_payloads(&path);
    eprintln!("[ADR-0163 test 1] observed entries: {observed:?}");

    // The race condition: two writes happened, but only one is observable.
    // The reader sees vid=1 with payload "bob" (last write wins on
    // HashMap insert in boot()). This is the silent data-loss shape.
    assert_eq!(observed.len(), 1, "duplicate vid should silently lose one");
    assert_eq!(observed[0].0, 1);
    assert_eq!(observed[0].1, "bob");
}

/// Test 2 — Sequential cross-lifetime duplicate vid.
///
/// Writer 1: open, ingest vid=1 "alice", drop.
/// Writer 2: open, ingest vid=1 "bob", drop. (DOES read writer 1's META_SEG
///           in boot(), but ingests with the same vid anyway.)
/// Reader: opens, sees only "bob".
///
/// This is closer to what happens when two CLI processes serialise via
/// the kernel flock. After writer 1 exits, writer 2's boot() loads
/// vid=1=alice. But writer 2's caller-supplied vid (from the JS layer's
/// per-process `nextNativeId=1`) is still 1.
#[test]
fn sequential_writers_with_colliding_vid_lose_first_write() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("dup_vid_seq.rvf");

    ingest_one(&path, 1, "alice");
    ingest_one(&path, 1, "bob");

    let observed = count_distinct_payloads(&path);
    eprintln!("[ADR-0163 test 2] observed: {observed:?}");

    // Without the fix: only "bob" survives.
    assert_eq!(observed.len(), 1);
    assert_eq!(observed[0].1, "bob");
}

/// Test 3 — Multi-process concurrent ingest with COLLIDING vids.
///
/// Spawns N child processes that each ingest one entry with a DIFFERENT
/// vid (caller-correct path: {1, 2, 3, 4, 5, 6}). The kernel flock
/// serialises them. After all exit, the parent opens the file and
/// counts distinct vids.
///
/// **Baseline expectation**: N entries visible. This proves the kernel
/// flock + boot() machinery work correctly for the non-colliding case.
///
/// We then run the same harness with COLLIDING vids (all writers use
/// vid=1) — expect 1 entry visible (silent loss of N-1 writes). This
/// pins down that the loss-on-collision is at the rust layer, not
/// somewhere in the JS-side machinery.
#[test]
fn multi_process_unique_vids_all_visible() {
    use std::process::Command;

    let dir = TempDir::new().unwrap();
    let path = dir.path().join("multi_proc.rvf");

    // The helper binary path. We'll build a tiny binary in a subdir
    // for cargo to compile; for now we use the running test binary
    // with an env-var dispatch.
    let helper = std::env::current_exe().unwrap();
    let n: u64 = 6;

    // Pre-create the file so all child processes hit the open path
    // (sidesteps create-vs-open coordination noise that's separately
    // tested elsewhere).
    {
        let store = RvfStore::create(&path, opts()).unwrap();
        drop(store);
    }

    // Spawn N children, each running the test binary in helper mode
    // (env var `RVF_TEST_HELPER=1`). Each child ingests one entry
    // with vid=child_index+1, payload="entry-N".
    let mut children = Vec::new();
    for i in 1..=n {
        let mut cmd = Command::new(&helper);
        cmd.env("RVF_TEST_HELPER", "1");
        cmd.env("RVF_TEST_PATH", &path);
        cmd.env("RVF_TEST_VID", i.to_string());
        cmd.env("RVF_TEST_PAYLOAD", format!("entry-{i}"));
        cmd.arg("--ignored").arg("__rvf_test_helper_main__");
        children.push(cmd.spawn().expect("spawn child"));
    }
    for mut c in children {
        let status = c.wait().expect("wait child");
        assert!(status.success(), "child failed: {status:?}");
    }

    let observed = count_distinct_payloads(&path);
    eprintln!("[ADR-0163 test 3 unique] observed ({}): {observed:?}", observed.len());
    assert_eq!(observed.len(), n as usize, "all entries visible under flock");
}

/// Test 3b — Multi-process concurrent ingest with COLLIDING vids.
///
/// Same harness as Test 3 but every child uses vid=1. Confirms the
/// duplicate-vid loss shape under cross-process kernel-flock
/// serialisation.
#[test]
fn multi_process_colliding_vids_lose_writes() {
    use std::process::Command;

    let dir = TempDir::new().unwrap();
    let path = dir.path().join("multi_proc_collide.rvf");
    let helper = std::env::current_exe().unwrap();
    let n: u64 = 6;

    {
        let store = RvfStore::create(&path, opts()).unwrap();
        drop(store);
    }

    let mut children = Vec::new();
    for i in 1..=n {
        let mut cmd = Command::new(&helper);
        cmd.env("RVF_TEST_HELPER", "1");
        cmd.env("RVF_TEST_PATH", &path);
        cmd.env("RVF_TEST_VID", "1"); // collide!
        cmd.env("RVF_TEST_PAYLOAD", format!("entry-{i}"));
        cmd.arg("--ignored").arg("__rvf_test_helper_main__");
        children.push(cmd.spawn().expect("spawn child"));
    }
    for mut c in children {
        let _ = c.wait().expect("wait child");
    }

    let observed = count_distinct_payloads(&path);
    eprintln!("[ADR-0163 test 3b collide] observed ({}): {observed:?}", observed.len());
    // Without the fix: 1 entry visible — N-1 writes silently lost.
    // The exact survivor is non-deterministic (depends on flock FIFO).
    assert_eq!(
        observed.len(),
        1,
        "colliding vids: only one survivor (silent data loss, pre-fix)"
    );
}

/// Helper-mode entry. When the test binary is run with
/// `RVF_TEST_HELPER=1`, just ingest one entry and exit.
///
/// We pretend it's an `#[ignore]`-marked test so that normal `cargo test`
/// runs do not exercise it; the spawning tests above invoke it
/// explicitly via `--ignored __rvf_test_helper_main__`.
#[test]
#[ignore]
fn __rvf_test_helper_main__() {
    if std::env::var("RVF_TEST_HELPER").is_err() {
        // Normal `cargo test --ignored` invocation: be a no-op.
        return;
    }
    let path = std::env::var("RVF_TEST_PATH").unwrap();
    let vid: u64 = std::env::var("RVF_TEST_VID").unwrap().parse().unwrap();
    let payload = std::env::var("RVF_TEST_PAYLOAD").unwrap();
    let path = Path::new(&path);
    ingest_one(path, vid, &payload);
}
