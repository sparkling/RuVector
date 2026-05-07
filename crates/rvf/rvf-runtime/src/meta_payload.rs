//! META_SEG payload encode/decode for per-vector metadata persistence.
//!
//! ADR-0154 Phase 1c — opaque-record stop-gap format. Designed for the
//! claude-flow memory backend's typical workload (≤ 10K entries with ~11
//! fields each, mostly short strings). Trades upstream's column-oriented
//! spec-08 layout for implementation simplicity. A future ADR migrates to
//! the column layout when the use case justifies it; the on-disk format
//! version field below provides forward-compat headroom.
//!
//! Layout (little-endian throughout):
//!
//!   [u8  format_version]              // 0x01
//!   [u8  reserved (3 bytes)]          // zero-padded for u32 alignment
//!   [u32 record_count]
//!   For each record:
//!     [u64 vector_id]
//!     [u32 entry_count]
//!     For each entry:
//!       [u16 field_id]
//!       [u8  value_type]              // 0=u64, 1=i64, 2=f64, 3=string, 4=bytes
//!       [u8  reserved]                // zero-padded for u32 alignment
//!       [u32 value_len]
//!       [bytes value]                 // raw; numerics in LE
//!
//! Decoder is bound-checked: any out-of-range read returns
//! `MetaPayloadError::Truncated`.

use crate::options::{MetadataEntry, MetadataValue};

const FORMAT_VERSION_V1: u8 = 0x01;

const VT_U64: u8 = 0;
const VT_I64: u8 = 1;
const VT_F64: u8 = 2;
const VT_STRING: u8 = 3;
const VT_BYTES: u8 = 4;

/// Errors returned by the META_SEG payload decoder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaPayloadError {
    /// Payload is shorter than the layout requires at the current offset.
    Truncated { needed: usize, offset: usize, total: usize },
    /// `format_version` byte is not recognized.
    UnsupportedVersion(u8),
    /// `value_type` byte is outside the documented enum.
    UnknownValueType(u8),
    /// `value_len` for a numeric type is wrong (must be 8 for u64/i64/f64).
    InvalidNumericLen { value_type: u8, len: u32 },
    /// String payload is not valid UTF-8.
    InvalidUtf8,
}

impl std::fmt::Display for MetaPayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated { needed, offset, total } => write!(
                f,
                "meta payload truncated: needed {needed} bytes at offset {offset} (total {total})"
            ),
            Self::UnsupportedVersion(v) => write!(f, "unsupported meta payload version 0x{v:02x}"),
            Self::UnknownValueType(t) => write!(f, "unknown value_type 0x{t:02x}"),
            Self::InvalidNumericLen { value_type, len } => write!(
                f,
                "value_type=0x{value_type:02x} requires len=8, got len={len}"
            ),
            Self::InvalidUtf8 => write!(f, "string value is not valid UTF-8"),
        }
    }
}

impl std::error::Error for MetaPayloadError {}

/// Encode a META_SEG payload from per-vector metadata entries.
///
/// Each `(vector_id, entries)` pair becomes one record. Entries within a
/// vector are written in iteration order. Empty `entries` is permitted
/// (record carries `entry_count=0`).
pub fn encode_meta_payload(records: &[(u64, &[MetadataEntry])]) -> Vec<u8> {
    let mut estimated_size = 8; // header
    for (_, entries) in records {
        estimated_size += 12 + entries.iter().map(|e| 8 + value_payload_len(&e.value)).sum::<usize>();
    }
    let mut out = Vec::with_capacity(estimated_size);

    out.push(FORMAT_VERSION_V1);
    out.extend_from_slice(&[0u8; 3]); // reserved
    out.extend_from_slice(&(records.len() as u32).to_le_bytes());

    for (vid, entries) in records {
        out.extend_from_slice(&vid.to_le_bytes());
        out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
        for entry in entries.iter() {
            out.extend_from_slice(&entry.field_id.to_le_bytes());
            match &entry.value {
                MetadataValue::U64(v) => {
                    out.push(VT_U64);
                    out.push(0u8); // reserved
                    out.extend_from_slice(&8u32.to_le_bytes());
                    out.extend_from_slice(&v.to_le_bytes());
                }
                MetadataValue::I64(v) => {
                    out.push(VT_I64);
                    out.push(0u8);
                    out.extend_from_slice(&8u32.to_le_bytes());
                    out.extend_from_slice(&v.to_le_bytes());
                }
                MetadataValue::F64(v) => {
                    out.push(VT_F64);
                    out.push(0u8);
                    out.extend_from_slice(&8u32.to_le_bytes());
                    out.extend_from_slice(&v.to_le_bytes());
                }
                MetadataValue::String(s) => {
                    out.push(VT_STRING);
                    out.push(0u8);
                    out.extend_from_slice(&(s.len() as u32).to_le_bytes());
                    out.extend_from_slice(s.as_bytes());
                }
                MetadataValue::Bytes(b) => {
                    out.push(VT_BYTES);
                    out.push(0u8);
                    out.extend_from_slice(&(b.len() as u32).to_le_bytes());
                    out.extend_from_slice(b);
                }
            }
        }
    }
    out
}

fn value_payload_len(v: &MetadataValue) -> usize {
    match v {
        MetadataValue::U64(_) | MetadataValue::I64(_) | MetadataValue::F64(_) => 8,
        MetadataValue::String(s) => s.len(),
        MetadataValue::Bytes(b) => b.len(),
    }
}

/// Decode a META_SEG payload into per-vector metadata records.
///
/// Returns `(vector_id, entries)` tuples in on-disk order. The decoder is
/// bound-checked: any truncation surfaces as `MetaPayloadError::Truncated`.
pub fn decode_meta_payload(
    payload: &[u8],
) -> Result<Vec<(u64, Vec<MetadataEntry>)>, MetaPayloadError> {
    let mut cur = 0usize;

    // Header: 1 byte version + 3 bytes reserved + 4 bytes record_count.
    need(payload, cur, 8)?;
    let version = payload[cur];
    if version != FORMAT_VERSION_V1 {
        return Err(MetaPayloadError::UnsupportedVersion(version));
    }
    cur += 4; // skip version + reserved
    let record_count = read_u32(payload, &mut cur)?;

    let mut records = Vec::with_capacity(record_count as usize);
    for _ in 0..record_count {
        need(payload, cur, 12)?;
        let vid = read_u64(payload, &mut cur)?;
        let entry_count = read_u32(payload, &mut cur)?;

        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            need(payload, cur, 8)?;
            let field_id = read_u16(payload, &mut cur)?;
            let value_type = payload[cur];
            cur += 2; // value_type + reserved
            let value_len = read_u32(payload, &mut cur)?;
            need(payload, cur, value_len as usize)?;
            let raw = &payload[cur..cur + value_len as usize];
            cur += value_len as usize;

            let value = match value_type {
                VT_U64 => {
                    if value_len != 8 {
                        return Err(MetaPayloadError::InvalidNumericLen { value_type, len: value_len });
                    }
                    MetadataValue::U64(u64::from_le_bytes(raw.try_into().unwrap()))
                }
                VT_I64 => {
                    if value_len != 8 {
                        return Err(MetaPayloadError::InvalidNumericLen { value_type, len: value_len });
                    }
                    MetadataValue::I64(i64::from_le_bytes(raw.try_into().unwrap()))
                }
                VT_F64 => {
                    if value_len != 8 {
                        return Err(MetaPayloadError::InvalidNumericLen { value_type, len: value_len });
                    }
                    MetadataValue::F64(f64::from_le_bytes(raw.try_into().unwrap()))
                }
                VT_STRING => {
                    let s = std::str::from_utf8(raw)
                        .map_err(|_| MetaPayloadError::InvalidUtf8)?;
                    MetadataValue::String(s.to_string())
                }
                VT_BYTES => MetadataValue::Bytes(raw.to_vec()),
                other => return Err(MetaPayloadError::UnknownValueType(other)),
            };
            entries.push(MetadataEntry { field_id, value });
        }
        records.push((vid, entries));
    }

    Ok(records)
}

fn need(payload: &[u8], offset: usize, n: usize) -> Result<(), MetaPayloadError> {
    if offset + n > payload.len() {
        Err(MetaPayloadError::Truncated {
            needed: n,
            offset,
            total: payload.len(),
        })
    } else {
        Ok(())
    }
}

fn read_u16(payload: &[u8], cur: &mut usize) -> Result<u16, MetaPayloadError> {
    need(payload, *cur, 2)?;
    let v = u16::from_le_bytes([payload[*cur], payload[*cur + 1]]);
    *cur += 2;
    Ok(v)
}

fn read_u32(payload: &[u8], cur: &mut usize) -> Result<u32, MetaPayloadError> {
    need(payload, *cur, 4)?;
    let v = u32::from_le_bytes([
        payload[*cur],
        payload[*cur + 1],
        payload[*cur + 2],
        payload[*cur + 3],
    ]);
    *cur += 4;
    Ok(v)
}

fn read_u64(payload: &[u8], cur: &mut usize) -> Result<u64, MetaPayloadError> {
    need(payload, *cur, 8)?;
    let bytes: [u8; 8] = payload[*cur..*cur + 8].try_into().unwrap();
    let v = u64::from_le_bytes(bytes);
    *cur += 8;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries(items: Vec<(u16, MetadataValue)>) -> Vec<MetadataEntry> {
        items
            .into_iter()
            .map(|(field_id, value)| MetadataEntry { field_id, value })
            .collect()
    }

    #[test]
    fn round_trip_mixed_types() {
        let v0 = entries(vec![
            (1, MetadataValue::String("hello".into())),
            (2, MetadataValue::U64(42)),
            (3, MetadataValue::F64(3.14)),
            (4, MetadataValue::Bytes(vec![0x00, 0xFF, 0xAB, 0xCD])),
        ]);
        let v1 = entries(vec![
            (1, MetadataValue::String(String::new())),
            (2, MetadataValue::I64(-7)),
        ]);
        let records: Vec<(u64, &[MetadataEntry])> =
            vec![(100, v0.as_slice()), (200, v1.as_slice())];

        let payload = encode_meta_payload(&records);
        let decoded = decode_meta_payload(&payload).expect("decode");

        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].0, 100);
        assert_eq!(decoded[0].1.len(), 4);
        match &decoded[0].1[0].value {
            MetadataValue::String(s) => assert_eq!(s, "hello"),
            _ => panic!("expected string"),
        }
        match &decoded[0].1[3].value {
            MetadataValue::Bytes(b) => assert_eq!(b, &[0x00, 0xFF, 0xAB, 0xCD]),
            _ => panic!("expected bytes"),
        }

        assert_eq!(decoded[1].0, 200);
        assert_eq!(decoded[1].1.len(), 2);
        match &decoded[1].1[1].value {
            MetadataValue::I64(v) => assert_eq!(*v, -7),
            _ => panic!("expected i64"),
        }
    }

    #[test]
    fn empty_records() {
        let records: Vec<(u64, &[MetadataEntry])> = vec![];
        let payload = encode_meta_payload(&records);
        let decoded = decode_meta_payload(&payload).expect("decode");
        assert!(decoded.is_empty());
    }

    #[test]
    fn record_with_zero_entries() {
        let empty: Vec<MetadataEntry> = vec![];
        let records: Vec<(u64, &[MetadataEntry])> = vec![(42, empty.as_slice())];
        let payload = encode_meta_payload(&records);
        let decoded = decode_meta_payload(&payload).expect("decode");
        assert_eq!(decoded, vec![(42, vec![])]);
    }

    #[test]
    fn truncated_header_returns_error() {
        let result = decode_meta_payload(&[0x01, 0x00, 0x00, 0x00]); // only 4 bytes
        assert!(matches!(result, Err(MetaPayloadError::Truncated { .. })));
    }

    #[test]
    fn unsupported_version_rejected() {
        let bad = [0x99, 0, 0, 0, 0, 0, 0, 0]; // version=0x99
        let result = decode_meta_payload(&bad);
        assert_eq!(result, Err(MetaPayloadError::UnsupportedVersion(0x99)));
    }

    #[test]
    fn unknown_value_type_rejected() {
        // version=1, record_count=1, vid=0, entry_count=1, field_id=0,
        // value_type=0x77 (unknown), reserved, value_len=0.
        let mut bad = vec![0x01, 0, 0, 0, 1, 0, 0, 0]; // header + count
        bad.extend_from_slice(&0u64.to_le_bytes()); // vid
        bad.extend_from_slice(&1u32.to_le_bytes()); // entry_count
        bad.extend_from_slice(&0u16.to_le_bytes()); // field_id
        bad.push(0x77); // unknown value_type
        bad.push(0); // reserved
        bad.extend_from_slice(&0u32.to_le_bytes()); // value_len
        let result = decode_meta_payload(&bad);
        assert_eq!(result, Err(MetaPayloadError::UnknownValueType(0x77)));
    }

    #[test]
    fn invalid_numeric_len_rejected() {
        let mut bad = vec![0x01, 0, 0, 0, 1, 0, 0, 0];
        bad.extend_from_slice(&0u64.to_le_bytes());
        bad.extend_from_slice(&1u32.to_le_bytes());
        bad.extend_from_slice(&0u16.to_le_bytes());
        bad.push(VT_U64);
        bad.push(0);
        bad.extend_from_slice(&4u32.to_le_bytes()); // wrong: u64 needs 8
        bad.extend_from_slice(&[0u8; 4]);
        let result = decode_meta_payload(&bad);
        assert!(matches!(
            result,
            Err(MetaPayloadError::InvalidNumericLen { value_type: VT_U64, len: 4 })
        ));
    }

    #[test]
    fn invalid_utf8_rejected() {
        let mut bad = vec![0x01, 0, 0, 0, 1, 0, 0, 0];
        bad.extend_from_slice(&0u64.to_le_bytes());
        bad.extend_from_slice(&1u32.to_le_bytes());
        bad.extend_from_slice(&0u16.to_le_bytes());
        bad.push(VT_STRING);
        bad.push(0);
        bad.extend_from_slice(&2u32.to_le_bytes());
        bad.extend_from_slice(&[0xff, 0xfe]); // not valid UTF-8
        let result = decode_meta_payload(&bad);
        assert_eq!(result, Err(MetaPayloadError::InvalidUtf8));
    }
}
