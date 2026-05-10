//! ADR-0167 Phase 1 — cross-process stress-test worker.
//!
//! Spawned by `tests/adr0167_n8_stress.rs` via
//! `env!("CARGO_BIN_EXE_rvf_test_writer")`. One worker per child process.
//!
//! Usage:
//!   rvf_test_writer <rvf_path> <vid> <payload> [<dim>]
//!
//! Behaviour:
//!   1. Open the .rvf at <rvf_path> — the test parent MUST pre-create it
//!      (this isolates the ADR-0167 boot-time race from the orthogonal
//!      create-vs-open race).
//!   2. Ingest a single vector with id=<vid> and a one-field metadata
//!      record (field_id=0, String=<payload>).
//!   3. Drop the store cleanly (releases the writer flock).
//!   4. Exit with code 0 on success, non-zero on any error — the parent
//!      asserts on the exit code as well as the post-run readback count.
//!
//! Intentionally does NOT retry. Any boot-time race surfaces as a non-zero
//! exit; the parent then assert!()s. ADR-0167 Phase 1 is supposed to make
//! N=8 deterministic — if any worker fails after the fix lands, that's a
//! regression.

use std::path::PathBuf;
use std::process::ExitCode;

use rvf_runtime::options::{MetadataEntry, MetadataValue};
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

    // Note: <dim> arg is accepted for parity with the parent; the actual
    // dimension comes from the on-disk manifest via `open()`. The parent
    // pre-creates the store with the dimension it wants and seeds the file.

    if !path.exists() {
        eprintln!("expected pre-created store at {:?}; parent must seed it", path);
        return ExitCode::from(3);
    }
    let mut store = match RvfStore::open(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("open failed: {:?}", e);
            return ExitCode::from(3);
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
