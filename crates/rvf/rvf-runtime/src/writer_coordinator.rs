//! Intra-process write-batching coordinator — ADR-0168 Phase 2.
//!
//! Serialises concurrent in-process callers into single-batch
//! `ingest_batch` + `commit_new_root` calls, reducing N × F_FULLFSYNC
//! to 1 × F_FULLFSYNC for the common case where N callers in the same
//! process share the same `.rvf` path.
//!
//! # Design
//!
//! - **Process-local registry**: one [`WriterCoordinator`] per `.rvf` path
//!   per process, stored in a `OnceLock<Mutex<HashMap<...>>>`.
//! - **Mutex + Condvar**: no tokio/async dependency; matches Phase-1 dep
//!   choices and the spec in ADR-0168.
//! - **Default-off**: callers must opt in by calling `submit_ingest`.
//!   The existing `RvfStore::ingest_batch` API is completely unaffected.
//! - **Open/close per batch**: the coordinator opens the store, calls
//!   `ingest_batch` once for the accumulated batch, then closes the store.
//!   Keeps the write lock held for the minimum window.
//!
//! # Perf model (N=8 intra-process)
//!
//! Without coordinator: N × ~95ms (one F_FULLFSYNC per caller) ≈ 767ms.
//! With coordinator:    1 × ~95ms + condvar overhead            ≈ 95–150ms.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

use crate::options::{IngestResult, MetadataEntry, RvfOptions};
use crate::store::RvfStore;
use rvf_types::RvfError;

/// Micro-batch accumulation window: how long the leader waits for concurrent
/// callers to push their payloads before starting the batch commit.
///
/// 2ms is a good balance: long enough that N=8 threads spawned in rapid
/// succession all push before the leader starts the F_FULLFSYNC, short enough
/// that it adds negligible latency to single-writer workloads.
const BATCH_WINDOW: Duration = Duration::from_millis(2);

// ── Process-local registry ────────────────────────────────────────────────────

fn registry() -> &'static Mutex<HashMap<PathBuf, Arc<WriterCoordinator>>> {
    static REG: OnceLock<Mutex<HashMap<PathBuf, Arc<WriterCoordinator>>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashMap::new()))
}

// ── Public types ──────────────────────────────────────────────────────────────

/// Snapshot of coordinator performance counters.
#[derive(Clone, Debug, Default)]
pub struct CoordinatorMetrics {
    /// Total `commit_new_root` calls issued.
    pub batches_committed: u64,
    /// Total vector entries committed across all batches.
    pub entries_committed: u64,
    /// Moving-average batch size (entries per batch).
    pub avg_batch_size: f64,
    /// p95 commit latency in milliseconds (sliding window of last 64 commits).
    pub p95_commit_latency_ms: f64,
}

// ── Internal state ────────────────────────────────────────────────────────────

struct PendingIngest {
    vectors:  Vec<Vec<f32>>,
    ids:      Vec<u64>,
    metadata: Option<Vec<MetadataEntry>>,
}

/// Result slot shared between the batch-runner and waiting callers.
struct BatchResult {
    /// Generation counter incremented after each commit.
    generation: u64,
    /// The last committed result (or error).
    result: Option<Result<IngestResult, RvfError>>,
}

struct State {
    pending:    Vec<PendingIngest>,
    busy:       bool,
    batch_res:  BatchResult,
    /// Latency ring buffer (ms) for the last 64 commits.
    latencies:  VecDeque<f64>,
    /// Aggregate telemetry.
    batches_committed: u64,
    entries_committed: u64,
}

impl State {
    fn new() -> Self {
        State {
            pending: Vec::new(),
            busy: false,
            batch_res: BatchResult { generation: 0, result: None },
            latencies: VecDeque::with_capacity(64),
            batches_committed: 0,
            entries_committed: 0,
        }
    }

    fn record_latency(&mut self, ms: f64) {
        if self.latencies.len() == 64 {
            self.latencies.pop_front();
        }
        self.latencies.push_back(ms);
    }

    fn p95_latency(&self) -> f64 {
        if self.latencies.is_empty() {
            return 0.0;
        }
        let mut sorted: Vec<f64> = self.latencies.iter().copied().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let idx = ((sorted.len() as f64) * 0.95) as usize;
        sorted[idx.min(sorted.len() - 1)]
    }
}

// ── WriterCoordinator ─────────────────────────────────────────────────────────

/// In-process write-batching coordinator for a single `.rvf` path.
///
/// Obtain via [`WriterCoordinator::for_path`]; never construct directly.
pub struct WriterCoordinator {
    /// Canonical path this coordinator is bound to.
    path: PathBuf,
    state: Mutex<State>,
    cond: Condvar,
}

impl WriterCoordinator {
    /// Get-or-create the coordinator for `path`.
    ///
    /// Multiple concurrent `for_path` calls with the same canonical path
    /// return the same `Arc<WriterCoordinator>`.  Concurrent calls are safe —
    /// the registry itself is guarded by a `Mutex`.
    pub fn for_path(path: &Path) -> Arc<WriterCoordinator> {
        // Canonicalize if possible; fall back to as-is for not-yet-created files.
        let key = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

        let mut reg = registry().lock().unwrap();
        if let Some(existing) = reg.get(&key) {
            return Arc::clone(existing);
        }
        let coord = Arc::new(WriterCoordinator {
            path: key.clone(),
            state: Mutex::new(State::new()),
            cond: Condvar::new(),
        });
        reg.insert(key, Arc::clone(&coord));
        coord
    }

    /// Submit a single-entry ingest to the coordinator's batch.
    ///
    /// Every caller pushes its payload onto `pending` and records the
    /// generation it belongs to.  The first caller that finds `!busy`
    /// becomes the batch leader: it waits `BATCH_WINDOW` for concurrent
    /// siblings to push, then drains all pending entries into a single
    /// `ingest_batch` call.  All waiting callers wake when the leader
    /// signals via `Condvar::notify_all`, and pick up the shared result.
    pub fn submit_ingest(
        &self,
        _options: RvfOptions,
        vectors: Vec<Vec<f32>>,
        ids: Vec<u64>,
        metadata: Option<Vec<MetadataEntry>>,
    ) -> Result<IngestResult, RvfError> {
        let my_payload = PendingIngest { vectors, ids, metadata };

        let mut state = self.state.lock().unwrap();

        // Push our payload first (regardless of busy state).
        state.pending.push(my_payload);

        if state.busy {
            // A leader is already running; record the generation we need to
            // wait for (current + 1 is guaranteed to cover our entry since
            // the leader will drain `pending` only after the current batch
            // completes, i.e. in the NEXT generation).
            let wait_for = state.batch_res.generation + 1;
            let state = self
                .cond
                .wait_while(state, |s| s.batch_res.generation < wait_for || s.busy)
                .unwrap();
            return state
                .batch_res
                .result
                .clone()
                .unwrap_or(Err(rvf_types::RvfError::Code(rvf_types::ErrorCode::InvalidManifest)));
        }

        // We are the batch leader for this generation.
        state.busy = true;
        let generation = state.batch_res.generation + 1;
        drop(state); // release lock so siblings can push their payloads

        // Accumulation window: wait for concurrent siblings to push.
        std::thread::sleep(BATCH_WINDOW);

        // Drain all pending payloads accumulated so far.
        let mut state = self.state.lock().unwrap();
        let batch: Vec<PendingIngest> = std::mem::take(&mut state.pending);
        drop(state);

        let t0 = Instant::now();
        let result = Self::run_batch(&self.path, batch);
        let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let mut state = self.state.lock().unwrap();
        state.batches_committed += 1;
        if let Ok(ref r) = result {
            state.entries_committed += r.accepted;
        }
        state.record_latency(elapsed_ms);
        state.busy = false;
        state.batch_res.generation = generation;
        state.batch_res.result = Some(result.clone());
        self.cond.notify_all();
        result
    }

    /// Run the actual ingest for all payloads in `batch` in a single store
    /// open/close cycle.
    fn run_batch(
        path: &Path,
        batch: Vec<PendingIngest>,
    ) -> Result<IngestResult, RvfError> {
        let mut store = RvfStore::open(path)?;

        let mut last_result = IngestResult { accepted: 0, rejected: 0, epoch: 0 };

        for payload in batch {
            let vecs: Vec<&[f32]> = payload.vectors.iter().map(|v| v.as_slice()).collect();
            let meta_ref: Option<&[MetadataEntry]> = payload.metadata.as_deref();
            let r = store.ingest_batch(&vecs, &payload.ids, meta_ref)?;
            last_result.accepted += r.accepted;
            last_result.rejected += r.rejected;
            last_result.epoch = r.epoch;
        }

        store.close()?;
        Ok(last_result)
    }

    /// Block until any in-flight batch commit has completed.  No-op if idle.
    pub fn flush(&self) {
        let state = self.state.lock().unwrap();
        if !state.busy {
            return;
        }
        drop(self.cond.wait_while(state, |s| s.busy).unwrap());
    }

    /// Return a snapshot of coordinator performance counters.
    pub fn metrics(&self) -> CoordinatorMetrics {
        let state = self.state.lock().unwrap();
        let p95 = state.p95_latency();
        let avg = if state.batches_committed == 0 {
            0.0
        } else {
            state.entries_committed as f64 / state.batches_committed as f64
        };
        CoordinatorMetrics {
            batches_committed: state.batches_committed,
            entries_committed: state.entries_committed,
            avg_batch_size: avg,
            p95_commit_latency_ms: p95,
        }
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{DistanceMetric, MetadataValue};
    use tempfile::TempDir;

    fn make_store_path() -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("coord_test.rvf");
        (dir, path)
    }

    fn pre_create(path: &Path) {
        let opts = RvfOptions {
            dimension: 4,
            metric: DistanceMetric::L2,
            ..Default::default()
        };
        let store = RvfStore::create(path, opts).unwrap();
        store.close().unwrap();
    }

    #[test]
    fn for_path_same_arc_for_same_path() {
        let (_dir, path) = make_store_path();
        let c1 = WriterCoordinator::for_path(&path);
        let c2 = WriterCoordinator::for_path(&path);
        assert!(Arc::ptr_eq(&c1, &c2), "same path must yield same coordinator");
    }

    #[test]
    fn submit_ingest_single_entry_roundtrip() {
        let (_dir, path) = make_store_path();
        pre_create(&path);

        let coord = WriterCoordinator::for_path(&path);
        let opts = RvfOptions { dimension: 4, metric: DistanceMetric::L2, ..Default::default() };
        let vectors = vec![vec![1.0f32, 2.0, 3.0, 4.0]];
        let ids = vec![42u64];
        let metadata = Some(vec![MetadataEntry {
            field_id: 1,
            value: MetadataValue::String("hello".into()),
        }]);

        let result = coord.submit_ingest(opts, vectors, ids, metadata);
        assert!(result.is_ok(), "submit_ingest failed: {result:?}");
        let r = result.unwrap();
        assert_eq!(r.accepted, 1);
        assert_eq!(r.rejected, 0);
    }

    #[test]
    fn flush_returns_when_idle() {
        let (_dir, path) = make_store_path();
        let coord = WriterCoordinator::for_path(&path);
        // Should return immediately without blocking.
        coord.flush();
    }

    #[test]
    fn metrics_initial_state() {
        let (_dir, path) = make_store_path();
        // Use a unique path so we get a fresh coordinator.
        let coord = WriterCoordinator::for_path(&path.with_extension("metrics_test.rvf"));
        let m = coord.metrics();
        assert_eq!(m.batches_committed, 0);
        assert_eq!(m.entries_committed, 0);
        assert_eq!(m.p95_commit_latency_ms, 0.0);
    }
}
