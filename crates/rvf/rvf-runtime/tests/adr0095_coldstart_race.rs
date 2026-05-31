//! ADR-0095 cold-start concurrent-write regression (t3-2 / `adr0079-tier3`).
//!
//! The complement of `adr0167_n8_stress.rs`: those tests PRE-CREATE the store
//! so workers only race `open()`. These tests do NOT pre-create — all N workers
//! race a **cold create** of the same fresh `.rvf`, which is the exact scenario
//! the `t3-2-concurrent` acceptance check exercises (it `rm`s the `.rvf` before
//! racing N writers).
//!
//! The 2026-05-04 lock change (blocking `flock(LOCK_EX)` → unfair
//! `LOCK_EX|LOCK_NB` poll) regressed this: under contention writers either
//! timed out with `LockHeld` (loud loss) or — with a longer timeout —
//! serialised non-FIFO and clobbered each other's manifests (silent loss). The
//! 2026-06-01 fix restores FIFO-blocking flock + a per-path in-process mutex.
//!
//! Correctness invariant (both modes, all N): every worker's vector+payload
//! must survive. Any shortfall is cross-process write loss — do NOT mark a
//! failure here "pre-existing"; it means the cold-start race is not resolved.
//!
//! Workers run in cold-start mode via env `RVF_TEST_WRITER_COLDSTART=1`.

use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use rvf_runtime::options::MetadataValue;
use rvf_runtime::store::RvfStore;
use tempfile::TempDir;

const DIM: u16 = 4;

/// Spawn N cold-start worker processes against `store_path` (which must NOT
/// exist yet), wait for all, panic on any non-zero exit.
fn run_n_coldstart_workers(store_path: &std::path::Path, n: usize) {
    let writer_bin = env!("CARGO_BIN_EXE_rvf_test_writer");

    // Spawn ALL children before waiting on any — genuine concurrent cold start.
    let children: Vec<(usize, std::process::Child)> = (1..=n)
        .map(|i| {
            let mut cmd = Command::new(writer_bin);
            cmd.arg(store_path)
                .arg(i.to_string())
                .arg(format!("writer-{i}"))
                .arg(DIM.to_string())
                .env("RVF_TEST_WRITER_COLDSTART", "1")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped());
            let child = cmd.spawn().unwrap_or_else(|e| panic!("spawn worker {i}: {e}"));
            (i, child)
        })
        .collect();

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

/// Read back all (vid, payload) pairs, sorted by vid.
fn readback_payloads(store_path: &std::path::Path) -> Vec<(u64, String)> {
    let store = RvfStore::open_readonly(store_path).expect("open_readonly after workers");
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

fn assert_all_persisted(store_path: &std::path::Path, n: usize, label: &str) {
    let entries = readback_payloads(store_path);
    assert_eq!(
        entries.len(),
        n,
        "[{label}] expected {n} entries, got {} — cold-start cross-process write loss: {entries:?}",
        entries.len()
    );
    let expected: Vec<(u64, String)> = (1..=n as u64).map(|i| (i, format!("writer-{i}"))).collect();
    assert_eq!(entries, expected, "[{label}] payload mismatch — overwrite/clobber bug");
}

/// coldstart_n8_no_loss — fast CI guard.
///
/// 8 workers race a cold create of a fresh `.rvf`; all 8 vectors must persist.
#[test]
fn coldstart_n8_no_loss() {
    let dir = TempDir::new().expect("tempdir");
    let store_path = dir.path().join("coldstart.rvf");
    let n = 8_usize;

    let start = Instant::now();
    run_n_coldstart_workers(&store_path, n);
    let elapsed = start.elapsed();
    eprintln!("[ADR-0095 coldstart n8] wall-time={}ms", elapsed.as_millis());

    assert_all_persisted(&store_path, n, "coldstart-n8");
}

/// coldstart_n16_no_loss — N=16 headroom, no injected load.
///
/// Ignored by default (slower); run with
/// `cargo test --test adr0095_coldstart_race -- --ignored coldstart_n16_no_loss`.
#[test]
#[ignore]
fn coldstart_n16_no_loss() {
    let dir = TempDir::new().expect("tempdir");
    let store_path = dir.path().join("coldstart.rvf");
    let n = 16_usize;

    run_n_coldstart_workers(&store_path, n);
    assert_all_persisted(&store_path, n, "coldstart-n16");
}

/// coldstart_n16_under_load — lock-level stress guard under saturation.
///
/// Saturates the machine with CPU + IO/fsync hogs for the duration of the race
/// and repeats the cold-start race over several rounds; every round must
/// persist all N. NOTE: this exercises the pure-Rust `RvfStore` path only — its
/// generous create-or-open retry budget absorbs the unfair NB-poll lock without
/// data loss even at this scale (the old lock is ~2x slower here but does not
/// lose). The *authoritative* reproduction of the t3-2 regression is the
/// CLI-level harness (full Node + NAPI + JS park/unpark + tight budgets), which
/// is where the unfair lock actually starves writers into loss. This test
/// guards the lock's correctness + reentrancy + that the fix introduces no
/// perf/serialization regression — it is not the regression's bite.
///
/// Ignored by default — it pins every core. Run explicitly:
/// `cargo test --test adr0095_coldstart_race -- --ignored --nocapture coldstart_n16_under_load`.
/// Tune via env: RVF_TEST_CPU_HOGS (default 24), RVF_TEST_IO_HOGS (default 6),
/// RVF_TEST_ROUNDS (default 8), RVF_TEST_N (default 16).
#[test]
#[ignore]
fn coldstart_n16_under_load() {
    let n: usize = env_usize("RVF_TEST_N", 16);
    let rounds: usize = env_usize("RVF_TEST_ROUNDS", 8);
    let cpu_hogs: usize = env_usize("RVF_TEST_CPU_HOGS", 24);
    let io_hogs: usize = env_usize("RVF_TEST_IO_HOGS", 6);

    let stop = Arc::new(AtomicBool::new(false));
    let io_dir = TempDir::new().expect("io tempdir");

    // CPU hogs: busy-loop on every core.
    let mut hog_handles = Vec::new();
    for _ in 0..cpu_hogs {
        let stop = Arc::clone(&stop);
        hog_handles.push(std::thread::spawn(move || {
            let mut x: u64 = 0;
            while !stop.load(Ordering::Relaxed) {
                // Opaque work the optimizer can't elide.
                x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                std::hint::black_box(x);
            }
        }));
    }
    // IO/fsync hogs: write + fsync in a tight loop to contend on the same
    // F_FULLFSYNC path the RVF writer pays.
    for h in 0..io_hogs {
        let stop = Arc::clone(&stop);
        let path = io_dir.path().join(format!("io-hog-{h}.bin"));
        hog_handles.push(std::thread::spawn(move || {
            let buf = vec![0u8; 1 << 20]; // 1 MiB
            while !stop.load(Ordering::Relaxed) {
                if let Ok(f) = std::fs::File::create(&path) {
                    use std::io::Write;
                    let mut f = f;
                    for _ in 0..8 {
                        let _ = f.write_all(&buf);
                    }
                    let _ = f.sync_all();
                }
            }
        }));
    }

    let result = std::panic::catch_unwind(|| {
        for round in 1..=rounds {
            let dir = TempDir::new().expect("tempdir");
            let store_path = dir.path().join("coldstart.rvf");

            let start = Instant::now();
            run_n_coldstart_workers(&store_path, n);
            let elapsed = start.elapsed();

            let entries = readback_payloads(&store_path);
            eprintln!(
                "[ADR-0095 coldstart-under-load round={round}/{rounds}] N={n} cpu={cpu_hogs} io={io_hogs} wall={}ms persisted={}/{n}",
                elapsed.as_millis(),
                entries.len()
            );
            assert_all_persisted(&store_path, n, &format!("coldstart-load-round-{round}"));
        }
    });

    stop.store(true, Ordering::Relaxed);
    for h in hog_handles {
        let _ = h.join();
    }

    if let Err(p) = result {
        std::panic::resume_unwind(p);
    }
}

fn env_usize(key: &str, default: usize) -> usize {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}
