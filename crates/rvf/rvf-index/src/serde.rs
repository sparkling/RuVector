//! Faithful (lossless) serialization of a full [`HnswGraph`].
//!
//! The [`codec`](crate::codec) module's `IndexSegData` is a progressive-index
//! wire format: it reconstructs node IDs from their position (assuming
//! contiguous `0..node_count`) and does not carry the entry point or the
//! per-node layer membership of an arbitrary HNSW graph. RVF vector IDs are
//! NOT guaranteed contiguous (deletes, COW branches, externally-assigned
//! IDs), so that format cannot round-trip a runtime graph without silently
//! corrupting it.
//!
//! This module persists the graph exactly: every node's real `u64` ID, its
//! neighbor list per layer, the entry point, and `max_layer`. It is the
//! payload of the witnessed `INDEX_SEG` written by the RVF runtime.
//!
//! Layout (all integers LEB128 varint unless noted):
//!
//! ```text
//! magic:        u32 LE   = 0x52564758  ("RVGX")
//! version:      u8       = 1
//! m:            varint
//! m0:           varint
//! ef_construct: varint
//! layer_count:  varint
//! has_entry:    u8       (0 = None, 1 = Some)
//! entry_point:  varint   (present iff has_entry == 1)
//! max_layer:    varint
//! for each layer (0..layer_count):
//!   node_count: varint
//!   for each node:
//!     node_id:       varint
//!     neighbor_count varint
//!     neighbors:     varint * neighbor_count   (absolute IDs, in stored order)
//! ```

extern crate alloc;

use alloc::vec::Vec;

use crate::codec::{decode_varint, encode_varint};
use crate::hnsw::{HnswConfig, HnswGraph, HnswLayer};

/// Magic prefix identifying a faithful HNSW graph serialization ("RVGX").
pub const HNSW_GRAPH_MAGIC: u32 = 0x5256_4758;

/// Current serialization format version.
pub const HNSW_GRAPH_VERSION: u8 = 1;

/// Errors that can occur while deserializing a serialized [`HnswGraph`].
#[derive(Clone, Debug, PartialEq)]
pub enum GraphCodecError {
    /// Buffer ended before all expected fields were read.
    Truncated,
    /// The leading magic did not match [`HNSW_GRAPH_MAGIC`].
    BadMagic,
    /// The version byte is not understood by this build.
    UnsupportedVersion(u8),
    /// A varint was malformed (overflow / incomplete).
    InvalidVarint,
}

impl core::fmt::Display for GraphCodecError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Truncated => write!(f, "serialized graph truncated"),
            Self::BadMagic => write!(f, "bad HNSW graph magic"),
            Self::UnsupportedVersion(v) => write!(f, "unsupported HNSW graph version: {}", v),
            Self::InvalidVarint => write!(f, "invalid varint in serialized graph"),
        }
    }
}

/// Serialize a full [`HnswGraph`] into a self-describing byte payload.
///
/// The result round-trips through [`deserialize_graph`] exactly: node IDs,
/// per-layer adjacency, entry point, and `max_layer` are all preserved.
pub fn serialize_graph(graph: &HnswGraph) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&HNSW_GRAPH_MAGIC.to_le_bytes());
    buf.push(HNSW_GRAPH_VERSION);

    encode_varint(graph.m as u64, &mut buf);
    encode_varint(graph.m0 as u64, &mut buf);
    encode_varint(graph.ef_construction as u64, &mut buf);
    encode_varint(graph.layers.len() as u64, &mut buf);

    match graph.entry_point {
        Some(ep) => {
            buf.push(1);
            encode_varint(ep, &mut buf);
        }
        None => buf.push(0),
    }
    encode_varint(graph.max_layer as u64, &mut buf);

    for layer in &graph.layers {
        encode_varint(layer.adjacency.len() as u64, &mut buf);
        for (&node_id, neighbors) in &layer.adjacency {
            encode_varint(node_id, &mut buf);
            encode_varint(neighbors.len() as u64, &mut buf);
            for &nid in neighbors {
                encode_varint(nid, &mut buf);
            }
        }
    }

    buf
}

/// Reconstruct a [`HnswGraph`] previously written by [`serialize_graph`].
pub fn deserialize_graph(data: &[u8]) -> Result<HnswGraph, GraphCodecError> {
    if data.len() < 5 {
        return Err(GraphCodecError::Truncated);
    }
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    if magic != HNSW_GRAPH_MAGIC {
        return Err(GraphCodecError::BadMagic);
    }
    let version = data[4];
    if version != HNSW_GRAPH_VERSION {
        return Err(GraphCodecError::UnsupportedVersion(version));
    }
    let mut pos = 5usize;

    let m = read_varint(data, &mut pos)? as usize;
    let m0 = read_varint(data, &mut pos)? as usize;
    let ef_construction = read_varint(data, &mut pos)? as usize;
    let layer_count = read_varint(data, &mut pos)? as usize;

    if pos >= data.len() {
        return Err(GraphCodecError::Truncated);
    }
    let has_entry = data[pos];
    pos += 1;
    let entry_point = if has_entry == 1 {
        Some(read_varint(data, &mut pos)?)
    } else {
        None
    };
    let max_layer = read_varint(data, &mut pos)? as usize;

    let config = HnswConfig {
        m,
        m0,
        ef_construction,
    };
    let mut graph = HnswGraph::new(&config);
    // HnswGraph::new seeds a single empty layer 0; replace with exactly the
    // deserialized layer set so the structure matches byte-for-byte.
    graph.layers = Vec::with_capacity(layer_count);

    for _ in 0..layer_count {
        let node_count = read_varint(data, &mut pos)? as usize;
        let mut layer = HnswLayer::default();
        for _ in 0..node_count {
            let node_id = read_varint(data, &mut pos)?;
            let neighbor_count = read_varint(data, &mut pos)? as usize;
            let mut neighbors = Vec::with_capacity(neighbor_count);
            for _ in 0..neighbor_count {
                neighbors.push(read_varint(data, &mut pos)?);
            }
            layer.adjacency.insert(node_id, neighbors);
        }
        graph.layers.push(layer);
    }

    // A graph must always have at least layer 0 for searches to function.
    if graph.layers.is_empty() {
        graph.layers.push(HnswLayer::default());
    }

    graph.entry_point = entry_point;
    graph.max_layer = max_layer;

    Ok(graph)
}

/// Read one LEB128 varint from `data` at `*pos`, advancing `*pos`.
fn read_varint(data: &[u8], pos: &mut usize) -> Result<u64, GraphCodecError> {
    if *pos >= data.len() {
        return Err(GraphCodecError::Truncated);
    }
    let (value, consumed) =
        decode_varint(&data[*pos..]).ok_or(GraphCodecError::InvalidVarint)?;
    *pos += consumed;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distance::l2_distance;
    use crate::traits::InMemoryVectorStore;
    use alloc::vec;

    fn build_sample_graph(n: usize, dim: usize) -> HnswGraph {
        let vectors: Vec<Vec<f32>> = (0..n)
            .map(|i| (0..dim).map(|d| (i * dim + d) as f32).collect())
            .collect();
        let store = InMemoryVectorStore::new(vectors);
        let config = HnswConfig {
            m: 8,
            m0: 16,
            ef_construction: 50,
        };
        let mut graph = HnswGraph::new(&config);
        for i in 0..n as u64 {
            let rng = ((i * 7 + 3) % 100) as f64 / 100.0;
            graph.insert(i, rng.clamp(0.001, 0.999), &store, &l2_distance);
        }
        graph
    }

    #[test]
    fn round_trip_preserves_structure() {
        let graph = build_sample_graph(80, 4);
        let bytes = serialize_graph(&graph);
        let restored = deserialize_graph(&bytes).unwrap();

        assert_eq!(restored.node_count(), graph.node_count());
        assert_eq!(restored.entry_point, graph.entry_point);
        assert_eq!(restored.max_layer, graph.max_layer);
        assert_eq!(restored.layers.len(), graph.layers.len());
        assert_eq!(restored.m, graph.m);
        assert_eq!(restored.m0, graph.m0);
        assert_eq!(restored.ef_construction, graph.ef_construction);

        for (orig, rest) in graph.layers.iter().zip(restored.layers.iter()) {
            assert_eq!(orig.adjacency, rest.adjacency);
        }
    }

    #[test]
    fn round_trip_non_contiguous_ids() {
        // IDs deliberately sparse / non-contiguous to prove the format does
        // not assume 0..N like the progressive codec does.
        let dim = 4;
        let ids = [5u64, 17, 999, 1_000_000, 42];
        let vectors: Vec<Vec<f32>> = (0..ids.len())
            .map(|i| (0..dim).map(|d| (i * dim + d) as f32).collect())
            .collect();
        // Map sparse ID -> dense vector slot via a wrapper store.
        struct SparseStore {
            map: alloc::collections::BTreeMap<u64, Vec<f32>>,
            dim: usize,
        }
        impl crate::traits::VectorStore for SparseStore {
            fn get_vector(&self, id: u64) -> Option<&[f32]> {
                self.map.get(&id).map(|v| v.as_slice())
            }
            fn dimension(&self) -> usize {
                self.dim
            }
        }
        let mut map = alloc::collections::BTreeMap::new();
        for (i, &id) in ids.iter().enumerate() {
            map.insert(id, vectors[i].clone());
        }
        let store = SparseStore { map, dim };
        let config = HnswConfig {
            m: 8,
            m0: 16,
            ef_construction: 50,
        };
        let mut graph = HnswGraph::new(&config);
        for (i, &id) in ids.iter().enumerate() {
            let rng = ((i * 7 + 3) % 100) as f64 / 100.0;
            graph.insert(id, rng, &store, &l2_distance);
        }

        let bytes = serialize_graph(&graph);
        let restored = deserialize_graph(&bytes).unwrap();
        assert_eq!(restored.entry_point, graph.entry_point);
        for (orig, rest) in graph.layers.iter().zip(restored.layers.iter()) {
            assert_eq!(orig.adjacency, rest.adjacency);
        }
        // Spot-check a known sparse ID survived.
        assert!(restored.layers[0].adjacency.contains_key(&1_000_000));
    }

    #[test]
    fn empty_graph_round_trips() {
        let config = HnswConfig::default();
        let graph = HnswGraph::new(&config);
        let bytes = serialize_graph(&graph);
        let restored = deserialize_graph(&bytes).unwrap();
        assert_eq!(restored.entry_point, None);
        assert_eq!(restored.node_count(), 0);
    }

    #[test]
    fn bad_magic_rejected() {
        let bytes = vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0];
        assert_eq!(
            deserialize_graph(&bytes).err(),
            Some(GraphCodecError::BadMagic)
        );
    }

    #[test]
    fn truncated_rejected() {
        let graph = build_sample_graph(20, 4);
        let mut bytes = serialize_graph(&graph);
        bytes.truncate(bytes.len() / 2);
        assert!(deserialize_graph(&bytes).is_err());
    }
}
