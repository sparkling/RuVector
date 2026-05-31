//! ADR-0167 Phase 1 — cross-process stress-test worker.
//!
//! Spawned by `tests/adr0167_n8_stress.rs` and `tests/adr0095_coldstart_race.rs`
//! via `env!("CARGO_BIN_EXE_rvf_test_writer")`. One worker per child process.
//!
//! Usage:
//!   rvf_test_writer <rvf_path> <vid> <payload> [<dim>]
//!
//! Two modes:
//!
//!   * **open-only** (default): the parent MUST pre-create the store. The
//!     worker `open()`s it and ingests — isolates the ADR-0167 boot-time race
//!     from the create-vs-open race. Does NOT retry.
//!
//!   * **cold-start** (env `RVF_TEST_WRITER_COLDSTART=1`): the parent does NOT
//!     pre-create; all N workers race a cold create of the same path. The
//!     worker mirrors the real CLI (`rvf-backend.ts`) create-or-open-with-retry
//!     dance: try `create()`, and on `LockHeld` (a peer created first) or a
//!     transient cold-start race shape, retry as `open()` within a budget.
//!     This is the exact t3-2 (`adr0079-tier3`) regression scenario.
//!
//! Behaviour in both modes:
//!   1. Acquire the store (open / create-or-open).
//!   2. Ingest a single vector with id=<vid> and a one-field metadata record
//!      (field_id=0, String=<payload>).
//!   3. Drop the store cleanly (releases the writer flock).
//!   4. Exit 0 on success, non-zero on any error — the parent asserts on both
//!      the exit code and the post-run readback count.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use rvf_runtime::options::{DistanceMetric, MetadataEntry, MetadataValue, RvfOptions};
use rvf_runtime::store::RvfStore;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "usage: {} <rvf_path> <vid> <payload> [<dim>]",
            args.first().map(String::as_str).unwrap_or("rvf_test_writer")
        );
        return ExitCode::from(2);
    }

    let path = PathBuf::from(&args[1]);
    let vid: u64 = match args[2].parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("invalid vid: {}", e);
            return ExitCode::from(2);
        }
    };
    let payload = args[3].clone();
    let dim: u16 = if args.len() >= 5 {
        match args[4].parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("invalid dim: {}", e);
                return ExitCode::from(2);
            }
        }
    } else {
        4
    };

    let coldstart = std::env::var("RVF_TEST_WRITER_COLDSTART")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let mut store = if coldstart {
        match acquire_coldstart(&path, dim) {
            Ok(s) => s,
            Err(code) => return code,
        }
    } else {
        if !path.exists() {
            eprintln!("expected pre-created store at {:?}; parent must seed it", path);
            return ExitCode::from(3);
        }
        match RvfStore::open(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("open failed: {:?}", e);
                return ExitCode::from(3);
            }
        }
    };

    // Deterministic vector content keyed off vid so duplicate-vid bugs
    // would be visible (different payload per writer).
    let vec_data: Vec<f32> = (0..dim).map(|i| (vid as f32) + 0.01 * (i as f32)).collect();
    let meta = vec![MetadataEntry {
        field_id: 0,
        value: MetadataValue::String(payload),
    }];

    match store.ingest_batch(&[&vec_data], &[vid], Some(&meta)) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ingest failed: {:?}", e);
            return ExitCode::from(5);
        }
    }

    if let Err(e) = store.close() {
        eprintln!("close failed: {:?}", e);
        return ExitCode::from(6);
    }

    ExitCode::SUCCESS
}

/// Create-or-open-with-retry, mirroring the real CLI cold-start path.
///
/// With the FIFO-blocking `flock` writer lock, the first peer creates the file
/// (holding the flock through `write_manifest`+`sync_all` on close) and every
/// other peer's `create()` re-check sees the file already present → `LockHeld`
/// → we retry as `open()`, which blocks FIFO until the creator releases, then
/// boots a fully-committed file. No write should be lost. The retry budget is
/// generous belt-and-suspenders: with a correct lock the loop converges in one
/// or two iterations.
fn acquire_coldstart(path: &Path, dim: u16) -> Result<RvfStore, ExitCode> {
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut attempt: u32 = 0;
    loop {
        attempt += 1;
        if path.exists() {
            match RvfStore::open(path) {
                Ok(s) => return Ok(s),
                Err(e) => {
                    if Instant::now() < deadline {
                        backoff(attempt);
                        continue;
                    }
                    eprintln!("coldstart open exhausted (attempt {attempt}): {:?}", e);
                    return Err(ExitCode::from(3));
                }
            }
        } else {
            let opts = RvfOptions {
                dimension: dim,
                metric: DistanceMetric::L2,
                ..Default::default()
            };
            match RvfStore::create(path, opts) {
                Ok(s) => return Ok(s),
                // LockHeld here means a peer created the file first (create()
                // drops its lock and returns LockHeld when the post-flock
                // existence check sees the file) — retry, which routes to the
                // open() branch above. Any other transient race shape also
                // retries within the budget.
                Err(e) => {
                    if Instant::now() < deadline {
                        backoff(attempt);
                        continue;
                    }
                    eprintln!("coldstart create exhausted (attempt {attempt}): {:?}", e);
                    return Err(ExitCode::from(4));
                }
            }
        }
    }
}

/// Small bounded backoff between cold-start retries. With a correct FIFO lock
/// the loop rarely iterates, so this is mostly dormant; the cap keeps a
/// pathological spin from busy-looping.
fn backoff(attempt: u32) {
    let ms = 2u64.saturating_mul(1u64 << attempt.min(5)).min(50);
    std::thread::sleep(Duration::from_millis(ms));
}
