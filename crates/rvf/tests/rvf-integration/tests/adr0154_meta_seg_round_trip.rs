//! ADR-0154 Phase 1h: META_SEG round-trip durability test.
//!
//! Asserts the runtime persists per-vector metadata to disk via META_SEGs and
//! reconstructs it bit-exact on reopen. Specifically exercises the
//! `MetadataValue::Bytes` variant — the variant that the legacy
//! `MetadataStore` lossy-converted to empty string before this ADR.

use rvf_runtime::options::{DistanceMetric, MetadataEntry, MetadataValue, RvfOptions};
use rvf_runtime::RvfStore;
use tempfile::TempDir;

const DIM: u16 = 4;

fn vec_n(n: usize) -> Vec<f32> {
    (0..DIM as usize).map(|i| (n + i) as f32).collect()
}

fn entries_for(n: u64) -> Vec<MetadataEntry> {
    vec![
        MetadataEntry {
            field_id: 1,
            value: MetadataValue::String(format!("key-{n}")),
        },
        MetadataEntry {
            field_id: 2,
            value: MetadataValue::U64(n),
        },
        MetadataEntry {
            field_id: 3,
            value: MetadataValue::I64(-(n as i64)),
        },
        MetadataEntry {
            field_id: 4,
            value: MetadataValue::F64(n as f64 * 3.14),
        },
        // The bytes variant — what ADR-0154 specifically fixes.
        MetadataEntry {
            field_id: 99,
            value: MetadataValue::Bytes(vec![n as u8, (n >> 8) as u8, 0xAB, 0xCD]),
        },
    ]
}

#[test]
fn round_trip_two_hundred_entries_with_bytes() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("adr0154-roundtrip.rvf");

    // ── Phase A: write 200 entries with mixed metadata, drop store ──────
    {
        let opts = RvfOptions {
            dimension: DIM,
            metric: DistanceMetric::L2,
            ..Default::default()
        };
        let mut store = RvfStore::create(&path, opts).unwrap();

        // Batch in chunks of 10 so we exercise multiple META_SEGs in the file.
        for chunk_start in (0..200u64).step_by(10) {
            let mut vec_data: Vec<Vec<f32>> = Vec::with_capacity(10);
            let mut ids: Vec<u64> = Vec::with_capacity(10);
            let mut metadata: Vec<MetadataEntry> = Vec::with_capacity(50);

            for i in 0..10u64 {
                let n = chunk_start + i;
                vec_data.push(vec_n(n as usize));
                ids.push(n);
                metadata.extend(entries_for(n));
            }
            let vectors_ref: Vec<&[f32]> = vec_data.iter().map(|v| v.as_slice()).collect();
            store
                .ingest_batch(&vectors_ref, &ids, Some(&metadata))
                .unwrap();
        }

        store.close().unwrap();
    }

    // ── Phase B: reopen, assert all 200 entries' metadata is bit-exact ──
    {
        let store = RvfStore::open(&path).unwrap();

        for n in 0..200u64 {
            let recalled = store
                .get_metadata(n)
                .unwrap_or_else(|| panic!("get_metadata({n}) returned None after reopen"));
            let expected = entries_for(n);

            assert_eq!(
                recalled.len(),
                expected.len(),
                "vid={n}: entry count mismatch (recalled {} vs expected {})",
                recalled.len(),
                expected.len(),
            );

            for (actual, want) in recalled.iter().zip(expected.iter()) {
                assert_eq!(actual.field_id, want.field_id, "vid={n}: field_id mismatch");
                assert_eq!(
                    actual.value, want.value,
                    "vid={n}: value mismatch for field_id={}",
                    want.field_id
                );
            }

            // Specifically verify the Bytes variant survives — the headline
            // change of ADR-0154 Phase 1.
            let bytes_entry = recalled.iter().find(|e| e.field_id == 99).expect("bytes entry");
            match &bytes_entry.value {
                MetadataValue::Bytes(b) => {
                    assert_eq!(
                        b.as_slice(),
                        &[n as u8, (n >> 8) as u8, 0xAB, 0xCD],
                        "vid={n}: bytes payload corrupted"
                    );
                }
                other => panic!("vid={n}: expected Bytes, got {other:?}"),
            }
        }
    }
}

#[test]
fn round_trip_empty_store_has_no_metadata() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("adr0154-empty.rvf");

    {
        let opts = RvfOptions {
            dimension: DIM,
            ..Default::default()
        };
        let store = RvfStore::create(&path, opts).unwrap();
        store.close().unwrap();
    }

    let store = RvfStore::open(&path).unwrap();
    assert!(store.iter_metadata().next().is_none());
    assert!(store.get_metadata(0).is_none());
    assert!(store.get_metadata(99).is_none());
}

#[test]
fn ingest_without_metadata_writes_no_meta_seg() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("adr0154-no-meta.rvf");

    {
        let opts = RvfOptions {
            dimension: DIM,
            ..Default::default()
        };
        let mut store = RvfStore::create(&path, opts).unwrap();
        let v = vec_n(0);
        let vectors_ref: Vec<&[f32]> = vec![v.as_slice()];
        store.ingest_batch(&vectors_ref, &[0u64], None).unwrap();
        store.close().unwrap();
    }

    let store = RvfStore::open(&path).unwrap();
    // Vector exists, but no metadata was written so get_metadata returns None.
    assert!(store.get_metadata(0).is_none());
}
