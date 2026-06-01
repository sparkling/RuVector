//! ADR-0167 cross-process N=8 / N=16 write-coordination stress tests.
//!
//! These tests gate the Phase 1 acceptance criteria from ADR-0167 (Final
//! acceptance criteria, round-2): AC1 (N=8 deterministic), AC6 (N=8
//! wall-time ~2.8–3.2s on Darwin; <10s budget here for platform headroom),
//! and AC7 (N=16 wall-time <20s).
//!
//! Each test spawns N worker processes via `std::process::Command`, all
//! pointing at a shared `.rvf` path. Workers use unique vids (1..=N) and
//! unique payloads ("writer-1".."writer-N"). All N children are spawned
//! FIRST, THEN waited on — ensuring genuine concurrent execution and
//! therefore covering the exact multi-writer race ADR-0167 is fixing.
//!
//! Tests live in this crate (rvf-runtime) rather than higher-level JS
//! tests because: (a) the correctness fix is in the Rust runtime; (b)
//! `cargo test` gives second-scale iteration vs the minute-scale release
//! pipeline; (c) `env!("CARGO_BIN_EXE_rvf_test_writer")` resolves at
//! compile time to the exact binary produced by this build.
//!
//! Any flakiness in this file is an ADR-0167 regression. Do not mark
//! failures as "pre-existing" — they indicate the cross-process race is
//! not fully resolved.

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use rvf_runtime::options::{DistanceMetric, MetadataValue, RvfOptions};
use rvf_runtime::store::RvfStore;
use tempfile::TempDir;

const DIM: u16 = 4;

/// Pre-create the .rvf store so workers always go through `open()`. This
/// isolates the ADR-0167 boot-time race (the actual target of Phase 1)
/// from the orthogonal `LockHeld`-on-create race that's outside the ADR's
/// scope.
fn pre_create_store(path: &std::path::Path) {
    let opts = RvfOptions {
        dimension: DIM,
        metric: DistanceMetric::L2,
        ..Default::default()
    };
    let store = RvfStore::create(path, opts).expect("pre-create store");
    store.close().expect("close pre-created store");
}

/// Spawn N worker processes against `store_path`, wait for all, panic on any failure.
///
/// All N children are spawned first, then waited on — ensuring they run
/// concurrently rather than sequentially, which is the whole point of the test.
fn run_n_workers(store_path: &std::path::Path, n: usize) {
    let writer_bin = env!("CARGO_BIN_EXE_rvf_test_writer");

    // Phase 1: spawn ALL children before waiting on any of them.
    let children: Vec<(usize, std::process::Child)> = (1..=n)
        .map(|i| {
            let mut cmd = Command::new(writer_bin);
            cmd.arg(store_path)
                .arg(i.to_string())
                .arg(format!("writer-{i}"))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped());
            let child = cmd.spawn().unwrap_or_else(|e| panic!("spawn worker {i}: {e}"));
            (i, child)
        })
        .collect();

    // Phase 2: consume each child (wait_with_output takes ownership).
    for (i, child) in children {
        let output = child
            .wait_with_output()
            .unwrap_or_else(|e| panic!("wait on worker {i}: {e}"));

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!(
                "worker {i} exited with status {:?}\nstderr:\n{stderr}",
                output.status
            );
        }
    }
}

/// Read back all (vid, payload) pairs from the store and return them sorted.
fn readback_payloads(store_path: &std::path::Path) -> Vec<(u64, String)> {
    let store = RvfStore::open_readonly(store_path)
        .expect("open_readonly after workers");
    let mut out: Vec<(u64, String)> = store
        .iter_metadata_with_vectors()
        .flat_map(|(id, _vec, entries)| {
            entries.iter().filter_map(move |e| {
                if let MetadataValue::String(s) = &e.value {
                    Some((id, s.clone()))
                } else {
                    None
                }
            })
        })
        .collect();
    out.sort_by_key(|(id, _)| *id);
    out
}

/// n8_cross_process_deterministic — gates AC1 and AC6.
///
/// Spawns 8 workers concurrently against a single `.rvf` store. All 8 must
/// exit with status 0, and a fresh read-only open must expose exactly 8
/// distinct vector entries with payloads "writer-1".."writer-8". Wall-time
/// must be under 10s (AC6 target is ~2.8–3.2s on Darwin; 10s gives headroom
/// for Linux/Windows CI variation).
#[test]
fn n8_cross_process_deterministic() {
    let dir = TempDir::new().expect("tempdir");
    let store_path = dir.path().join("store.rvf");
    pre_create_store(&store_path);
    let n =8_usize;

    let start = Instant::now();
    run_n_workers(&store_path, n);
    let elapsed = start.elapsed();

    let entries = readback_payloads(&store_path);
    eprintln!("[ADR-0167 n8] wall-time={}ms entries={}", elapsed.as_millis(), entries.len());

    assert_eq!(
        entries.len(),
        n,
        "expected {n} entries, got {} — cross-process write loss detected: {entries:?}",
        entries.len()
    );

    let expected: Vec<(u64, String)> = (1..=n as u64)
        .map(|i| (i, format!("writer-{i}")))
        .collect();
    assert_eq!(entries, expected, "payload mismatch — vid-assignment or overwrite bug");

    assert!(
        elapsed < Duration::from_secs(10),
        "wall-time {}ms exceeded 10s budget (AC6)",
        elapsed.as_millis()
    );
}

/// n16_cross_process_headroom — gates AC7.
///
/// Same shape as n8_cross_process_deterministic but with N=16. Wall-time
/// must be under 20s (AC7). Ignored by default to keep `cargo test` fast;
/// run explicitly via `cargo test --test adr0167_n8_stress -- --ignored
/// n16_cross_process_headroom`.
#[test]
#[ignore]
fn n16_cross_process_headroom() {
    let dir = TempDir::new().expect("tempdir");
    let store_path = dir.path().join("store.rvf");
    pre_create_store(&store_path);
    let n =16_usize;

    let start = Instant::now();
    run_n_workers(&store_path, n);
    let elapsed = start.elapsed();

    let entries = readback_payloads(&store_path);
    eprintln!("[ADR-0167 n16] wall-time={}ms entries={}", elapsed.as_millis(), entries.len());

    assert_eq!(
        entries.len(),
        n,
        "expected {n} entries, got {} — cross-process write loss detected: {entries:?}",
        entries.len()
    );

    let expected: Vec<(u64, String)> = (1..=n as u64)
        .map(|i| (i, format!("writer-{i}")))
        .collect();
    assert_eq!(entries, expected, "payload mismatch under N=16");

    assert!(
        elapsed < Duration::from_secs(20),
        "wall-time {}ms exceeded 20s budget (AC7)",
        elapsed.as_millis()
    );
}

/// n8_repeat_5x_no_flake — stability canary for AC1.
///
/// Runs the N=8 scenario 5 times in independent tempdirs. A single N=8 pass
/// could be lucky; 5 consecutive passes with a fresh store each time provides
/// statistical confidence that the fix is deterministic, not just survivable.
/// Ignored by default — it's a multi-minute regression canary.
#[test]
#[ignore]
fn n8_repeat_5x_no_flake() {
    let n = 8_usize;
    let rounds = 5;

    for round in 1..=rounds {
        let dir = TempDir::new().expect("tempdir");
        let store_path = dir.path().join("store.rvf");
        pre_create_store(&store_path);

        let start = Instant::now();
        run_n_workers(&store_path, n);
        let elapsed = start.elapsed();

        let entries = readback_payloads(&store_path);
        eprintln!(
            "[ADR-0167 n8x5 round={round}] wall-time={}ms entries={}",
            elapsed.as_millis(),
            entries.len()
        );

        assert_eq!(
            entries.len(),
            n,
            "round {round}: expected {n} entries, got {} — flakiness detected: {entries:?}",
            entries.len()
        );

        let expected: Vec<(u64, String)> = (1..=n as u64)
            .map(|i| (i, format!("writer-{i}")))
            .collect();
        assert_eq!(entries, expected, "round {round}: payload mismatch");

        assert!(
            elapsed < Duration::from_secs(10),
            "round {round}: wall-time {}ms exceeded 10s budget",
            elapsed.as_millis()
        );
    }
}

// ── ADR-0284 single-flock collapse: park/unpark envelope stress ──────────────
//
// The tests above use open→ingest→close, where the flock is held CONTINUOUSLY
// from open to close — peers fully serialize and never hit unpark_writer. The JS
// `rvf-backend.ts` layer (and the ADR-0284 single-flock collapse) instead RELEASES
// the flock between transactions (park) and reacquires-with-resync (unpark). After
// the `.jslock` was removed, the JS layer still lost writes at N=16 — so the
// residual race is in the park/unpark + resync_for_write path, which the
// open→ingest→close tests cannot see. These tests reproduce it natively.

/// Spawn N park/unpark workers (env `RVF_TEST_WRITER_PARKUNPARK=1`) concurrently.
fn run_n_workers_parkunpark(store_path: &std::path::Path, n: usize) {
    let writer_bin = env!("CARGO_BIN_EXE_rvf_test_writer");
    let children: Vec<(usize, std::process::Child)> = (1..=n)
        .map(|i| {
            let mut cmd = Command::new(writer_bin);
            cmd.arg(store_path)
                .arg(i.to_string())
                .arg(format!("writer-{i}"))
                .env("RVF_TEST_WRITER_PARKUNPARK", "1")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped());
            let child = cmd.spawn().unwrap_or_else(|e| panic!("spawn parkunpark worker {i}: {e}"));
            (i, child)
        })
        .collect();
    for (i, child) in children {
        let output = child
            .wait_with_output()
            .unwrap_or_else(|e| panic!("wait on parkunpark worker {i}: {e}"));
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("parkunpark worker {i} exited {:?}\nstderr:\n{stderr}", output.status);
        }
    }
}

/// ADR-0284 — N=16 cross-process stress using the PARK/UNPARK envelope (the JS
/// pattern: open → park → unpark+resync → ingest → park), the SOLE cross-process
/// serializer the single-flock collapse relies on. Must reach 16/16; a shortfall
/// is the residual `resync_for_write` race the JS layer hit.
#[test]
fn n16_parkunpark_cross_process() {
    let dir = TempDir::new().expect("tempdir");
    let store_path = dir.path().join("store.rvf");
    pre_create_store(&store_path);
    let n = 16_usize;

    let start = Instant::now();
    run_n_workers_parkunpark(&store_path, n);
    let elapsed = start.elapsed();

    let entries = readback_payloads(&store_path);
    eprintln!("[ADR-0284 n16 parkunpark] wall-time={}ms entries={}/{}", elapsed.as_millis(), entries.len(), n);

    assert_eq!(
        entries.len(),
        n,
        "park/unpark cross-process write loss: {}/{} survived — {entries:?}",
        entries.len(),
        n
    );
    let expected: Vec<(u64, String)> = (1..=n as u64).map(|i| (i, format!("writer-{i}"))).collect();
    assert_eq!(entries, expected, "park/unpark payload mismatch (overwrite/clobber)");
}

/// ADR-0284 ROOT-CAUSE confirmation — duplicate native ids OVERWRITE.
///
/// 8 park/unpark writers all using vid=1 with DISTINCT payloads. A store keyed by
/// numeric id collapses them to a single entry. This is the exact mechanism the JS
/// `assignNativeId` hits cross-process: `nextNativeId` is a per-process counter
/// seeded identically from the shared load, so concurrent writers assign the SAME
/// id to different entries. Proves the loss is id-collision, NOT a lock/resync
/// defect (the unique-id `n16_parkunpark_cross_process` passes 16/16 on the same
/// build). The fix therefore belongs in id assignment, not the flock.
#[test]
fn dup_native_id_overwrites_confirms_idcollision() {
    let dir = TempDir::new().expect("tempdir");
    let store_path = dir.path().join("store.rvf");
    pre_create_store(&store_path);
    let n = 8_usize;
    let writer_bin = env!("CARGO_BIN_EXE_rvf_test_writer");
    let children: Vec<(usize, std::process::Child)> = (1..=n)
        .map(|i| {
            let mut cmd = Command::new(writer_bin);
            cmd.arg(&store_path)
                .arg("1") // ALL writers use vid=1 — the collision the JS counter produces
                .arg(format!("writer-{i}"))
                .env("RVF_TEST_WRITER_PARKUNPARK", "1")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped());
            (i, cmd.spawn().unwrap_or_else(|e| panic!("spawn dup-id worker {i}: {e}")))
        })
        .collect();
    for (i, child) in children {
        let output = child.wait_with_output().unwrap_or_else(|e| panic!("wait {i}: {e}"));
        assert!(output.status.success(), "dup-id worker {i}: {}", String::from_utf8_lossy(&output.stderr));
    }
    let entries = readback_payloads(&store_path);
    eprintln!("[ADR-0284 dup-id] {} entries from 8 writers all vid=1 => {entries:?}", entries.len());
    assert_eq!(
        entries.len(),
        1,
        "8 dup-id writers should collapse to 1 entry (id-keyed overwrite); got {entries:?}"
    );
}

/// ADR-0284 — N=8 park/unpark repeated 5× (the race is probabilistic; a single
/// pass can be lucky). Ignored by default — a multi-round regression canary.
#[test]
#[ignore]
fn n8_parkunpark_repeat_5x() {
    let n = 8_usize;
    for round in 1..=5 {
        let dir = TempDir::new().expect("tempdir");
        let store_path = dir.path().join("store.rvf");
        pre_create_store(&store_path);
        run_n_workers_parkunpark(&store_path, n);
        let entries = readback_payloads(&store_path);
        eprintln!("[ADR-0284 n8 parkunpark round={round}] entries={}/{}", entries.len(), n);
        assert_eq!(entries.len(), n, "round {round}: park/unpark loss {}/{} — {entries:?}", entries.len(), n);
    }
}
