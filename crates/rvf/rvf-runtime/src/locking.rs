//! Writer lock management for single-writer / multi-reader concurrency.
//!
//! ADR-0095 amendment (2026-05-01, t3-2 fix — fork-only, sparkleideas/ruvector):
//! the original implementation used `O_CREAT|O_EXCL` with a PID-stamped lock
//! file and userspace stale-detection. Under N concurrent CLI processes that
//! design produced LockHeld budget exhaustion and PID-stale-detection races.
//! It was replaced with kernel `flock(LOCK_EX)` on a sibling `.lock` file that
//! is never unlinked — giving FIFO blocking, auto-release on process death, and
//! a same-inode flock queue shared by every cross-process peer.
//!
//! ADR-0095 amendment (2026-06-01, t3-2 concurrent-write regression fix —
//! reverts the 2026-05-04 change below):
//!
//!   The 2026-05-04 "deadlock postmortem fix" replaced the blocking
//!   `flock(LOCK_EX)` with a NON-blocking `flock(LOCK_EX|LOCK_NB)` poll loop
//!   (100ms sleep, give up after `RVF_LOCK_ACQUIRE_TIMEOUT_MS`). That traded a
//!   *rare same-process self-deadlock* for *common cross-process starvation*:
//!   `LOCK_EX|LOCK_NB` does NOT join the kernel FIFO queue, so under N-way
//!   contention each waiter polls every 100ms and most miss the brief free
//!   window. The losing writers spent the whole budget polling-and-missing,
//!   then timed out → `LockHeld` (loud loss). Raising the timeout converted the
//!   loud errors into SILENT loss, because the create/open/unpark paths in
//!   `store.rs` are written assuming the lock provides true FIFO ordering
//!   ("the kernel flock guarantees FIFO ordering across processes" — they
//!   re-check existence / re-validate txnid only AFTER acquiring, trusting the
//!   queue to serialise them). The unfair poll lock broke that invariant, so
//!   serialised-looking writers still clobbered each other's manifests.
//!
//!   This amendment restores TRUE blocking `flock(LOCK_EX)` (fair kernel FIFO,
//!   no userspace timeout, no starvation) for the cross-process path. The
//!   same-process self-deadlock the 2026-05-04 change was avoiding — two
//!   in-process `RvfStore` handles on the same path each opening a distinct fd
//!   and blocking against each other (macOS `flock` is per-OFD) — is now closed
//!   PROPERLY with a per-path in-process `Mutex` held across the entire
//!   check → acquire → record critical section. A second same-process acquirer
//!   blocks on that mutex (a normal in-process lock), observes the refcount
//!   once the first acquirer has recorded it, and shares the existing flock via
//!   refcount instead of opening a second fd. Cross-process peers live in a
//!   different process with a different per-path mutex and contend only on the
//!   kernel flock — so the mutex cannot cause a cross-process hang, and the
//!   kernel flock cannot cause a same-process hang.
//!
//! Properties of the restored design:
//!
//!   - **FIFO blocking.** `LOCK_EX` queues writers in the kernel; no userspace
//!     retry budget, no poll-and-miss lottery, no give-up.
//!   - **Auto-release on process death.** The kernel closes all fds when a
//!     process exits — including when N-API skips Rust `Drop` due to
//!     `process.exit(0)`. The flock is dropped with the fd; no stale-lock
//!     detection needed.
//!   - **Same-inode flock queue.** The lock file is never unlinked, so
//!     concurrent opens across processes share one inode and one kernel queue.
//!   - **Deadlock-free same-process re-entrancy.** The per-path mutex +
//!     refcount means nested / concurrent in-process acquisitions on one path
//!     never open a second competing fd.
//!
//! Public API is preserved: `WriterLock::acquire`, `release`, `Drop`,
//! `is_valid`, and `lock_path_for` all keep their signatures so `store.rs`
//! needs no changes. `acquire` BLOCKS on cross-process contention (it never
//! returns `WouldBlock`); callers that wrapped failures in `LockHeld` retry
//! loops simply never see those errors.
//!
//! Platform support: Unix only (Linux, macOS, BSD). Windows gets a no-op stub
//! (out of scope per ADR-0095).

use std::io;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::collections::HashMap;
#[cfg(unix)]
use std::sync::{Arc, Mutex, OnceLock};

/// Per-path coordination state guarding ONE rvf path's writer flock.
///
/// The enclosing `Mutex<PathLock>` is held across the entire
/// check → `flock(LOCK_EX)` → record sequence in [`WriterLock::acquire`], so a
/// second same-process acquirer for the same path blocks on the mutex (a normal
/// in-process lock) rather than opening a second fd and blocking against this
/// process's own first fd. That is what makes blocking `flock` safe to use
/// here without re-introducing the 2026-05-04 self-deadlock.
#[cfg(unix)]
struct PathLock {
    /// Number of live `WriterLock` guards in this process sharing the flock.
    /// Nested / concurrent in-process acquisitions bump this; the kernel flock
    /// is taken once (when 0 → 1) and released once (when 1 → 0).
    refcount: usize,
    /// The fd owning the kernel flock while `refcount > 0`; `-1` when idle.
    fd: libc::c_int,
}

/// Get-or-create the process-local coordination mutex for `lock_path`.
///
/// Entries are never removed: a stable `Arc<Mutex<PathLock>>` per path
/// guarantees all same-process acquirers serialise on the SAME mutex (removing
/// and re-creating would open a window where two acquirers hold two different
/// mutexes for one path and could each take a competing flock — the very
/// self-deadlock we are closing). The registry holds one small struct per
/// distinct `.rvf` path touched by the process; that set is bounded.
#[cfg(unix)]
fn path_lock_for(lock_path: &Path) -> Arc<Mutex<PathLock>> {
    static REG: OnceLock<Mutex<HashMap<PathBuf, Arc<Mutex<PathLock>>>>> = OnceLock::new();
    let reg = REG.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = reg.lock().unwrap();
    map.entry(lock_path.to_path_buf())
        .or_insert_with(|| Arc::new(Mutex::new(PathLock { refcount: 0, fd: -1 })))
        .clone()
}

/// Represents an acquired writer lock.
///
/// On Unix every guard holds an `Arc` to its path's [`PathLock`]; the kernel
/// `flock(LOCK_EX)` is owned by the `PathLock::fd` while any guard is live.
/// On Drop the refcount is decremented and, when it reaches zero, the kernel
/// flock is released and the owning fd closed.
pub(crate) struct WriterLock {
    #[cfg(unix)]
    path_lock: Arc<Mutex<PathLock>>,
    #[cfg(not(unix))]
    _phantom: (),
}

impl WriterLock {
    /// Acquire the writer lock for the given RVF file path.
    ///
    /// **Blocking on Unix.** Takes a true `flock(LOCK_EX)` on the never-unlinked
    /// sibling `.lock` file, joining the kernel FIFO queue. Cross-process peers
    /// serialise fairly with no timeout and no starvation; the lock
    /// auto-releases if this process dies. Same-process re-acquisitions of a
    /// path already held by this process share the existing flock via a
    /// refcount (no second fd, no self-deadlock). Returns `Ok(WriterLock)` once
    /// held, or an `io::Error` on a genuine syscall failure (ENFILE, EACCES,
    /// EBADF, ENOLCK, …) — `EINTR` is retried, never surfaced.
    ///
    /// On non-Unix platforms this is a no-op (the previous PID-based design
    /// never worked on Windows either). A real Windows port would use
    /// `LockFileEx`.
    pub(crate) fn acquire(rvf_path: &Path) -> io::Result<Self> {
        #[cfg(unix)]
        {
            use std::ffi::CString;
            use std::os::unix::ffi::OsStrExt;

            let lock_path = lock_path_for(rvf_path);
            let path_lock = path_lock_for(&lock_path);

            // Hold the per-path mutex across the whole check → flock → record
            // sequence. A concurrent same-process acquirer for this path blocks
            // HERE on the mutex (not on its own flock fd), so the macOS
            // per-OFD self-deadlock window is closed without abandoning kernel
            // FIFO fairness for the cross-process case.
            let mut state = path_lock.lock().unwrap();

            // Same-process re-entrancy: the flock is already held by this
            // process for this path. `flock(LOCK_EX)` is per-fd on Linux/macOS,
            // so opening a second fd and blocking on it would deadlock against
            // our own first fd. Refcount onto the existing flock instead.
            if state.refcount > 0 {
                state.refcount += 1;
                drop(state);
                return Ok(WriterLock { path_lock });
            }

            // First acquirer in this process: open the lock file and take a
            // TRUE blocking exclusive flock.
            let c_path = CString::new(lock_path.as_os_str().as_bytes())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

            // O_CREAT|O_RDWR, mode 0o644. The file is NEVER unlinked, so
            // concurrent opens across processes all refer to the same inode and
            // join the same kernel flock queue.
            let fd = unsafe {
                libc::open(
                    c_path.as_ptr(),
                    libc::O_RDWR | libc::O_CREAT | libc::O_CLOEXEC,
                    0o644,
                )
            };
            if fd < 0 {
                return Err(io::Error::last_os_error());
            }

            // Blocking LOCK_EX. The kernel parks this thread until the lock is
            // free and hands it off fairly — no poll, no timeout, no give-up.
            // `EINTR` (a signal interrupting the blocked syscall — e.g. Node's
            // SIGCHLD) is NOT a lock failure: retry. Any other error is a real
            // syscall failure → surface it loud per ADR-0082 (no silent
            // fallback). A dead peer's flock is auto-released by the kernel, so
            // blocking here cannot wedge on a crashed holder.
            loop {
                let rc = unsafe { libc::flock(fd, libc::LOCK_EX) };
                if rc == 0 {
                    break; // acquired
                }
                let e = io::Error::last_os_error();
                if e.raw_os_error() == Some(libc::EINTR) {
                    continue;
                }
                unsafe { libc::close(fd) };
                return Err(e);
            }

            // Record the holder so future same-process acquisitions refcount.
            state.fd = fd;
            state.refcount = 1;
            drop(state);

            Ok(WriterLock { path_lock })
        }
        #[cfg(not(unix))]
        {
            let _ = rvf_path;
            Ok(WriterLock { _phantom: () })
        }
    }

    /// Release the writer lock explicitly.
    ///
    /// Equivalent to dropping the guard. Kept for API compatibility with the
    /// previous implementation (`store.rs` calls this in `close()`).
    pub(crate) fn release(self) -> io::Result<()> {
        // Drop runs automatically when `self` goes out of scope; explicit drop
        // is unnecessary but harmless.
        drop(self);
        Ok(())
    }

    /// Check if the lock is still held by us.
    ///
    /// Under the flock-based design a successfully-acquired guard is held until
    /// dropped — there is no "lock taken over" condition the way the old
    /// PID-file design had. Kept for API compatibility with the old caller
    /// surface.
    #[allow(dead_code)]
    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

#[cfg(unix)]
impl Drop for WriterLock {
    fn drop(&mut self) {
        // Decrement the process-local refcount. The last live guard for this
        // path (refcount 1 → 0) releases the kernel flock and closes the owning
        // fd; nested holders (refcount > 1) just decrement. Because the fd
        // lives in the shared `PathLock` (not the guard), out-of-order guard
        // drops can never leak the fd or leave the flock held.
        let mut state = self.path_lock.lock().unwrap();
        if state.refcount > 1 {
            state.refcount -= 1;
            return;
        }
        state.refcount = 0;
        if state.fd >= 0 {
            // Explicit LOCK_UN wakes queued peers deterministically; close()
            // alone would also release on the kernel side.
            unsafe {
                libc::flock(state.fd, libc::LOCK_UN);
                libc::close(state.fd);
            }
            state.fd = -1;
        }
    }
}

#[cfg(not(unix))]
impl Drop for WriterLock {
    fn drop(&mut self) {
        // No-op stub on non-Unix.
    }
}

/// Compute the lock file path for a given RVF file.
///
/// Path is unchanged from the prior implementation (`<rvf>.lock`). Existing
/// stale lock files on disk left over from the old PID-file format are
/// harmless — when the new code calls `open(lock_path, O_CREAT|O_RDWR)` it
/// reuses the existing file (or creates one if absent). The first peer to
/// `flock(LOCK_EX)` on it wins; the file's binary content is irrelevant to
/// the new code.
pub(crate) fn lock_path_for(rvf_path: &Path) -> PathBuf {
    let mut p = rvf_path.as_os_str().to_os_string();
    p.push(".lock");
    PathBuf::from(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn lock_path_computation() {
        let p = Path::new("/tmp/data.rvf");
        assert_eq!(lock_path_for(p), PathBuf::from("/tmp/data.rvf.lock"));
    }

    #[test]
    fn acquire_and_release() {
        let dir = TempDir::new().unwrap();
        let rvf_path = dir.path().join("test.rvf");
        std::fs::write(&rvf_path, b"").unwrap();

        let lock = WriterLock::acquire(&rvf_path).unwrap();
        assert!(lock.is_valid());

        lock.release().unwrap();

        // Re-acquisition after explicit release succeeds.
        let lock2 = WriterLock::acquire(&rvf_path).unwrap();
        assert!(lock2.is_valid());
    }

    #[test]
    fn same_process_reentrant_acquire_refcounts_not_deadlocks() {
        // Two guards on the same path in the SAME process must both succeed by
        // sharing one flock via refcount — never block against each other.
        // (With straight blocking flock and no per-path mutex this would
        // deadlock on macOS per-OFD semantics; the mutex + refcount closes it.)
        let dir = TempDir::new().unwrap();
        let rvf_path = dir.path().join("reentrant.rvf");
        std::fs::write(&rvf_path, b"").unwrap();

        let a = WriterLock::acquire(&rvf_path).unwrap();
        let b = WriterLock::acquire(&rvf_path).unwrap();
        assert!(a.is_valid());
        assert!(b.is_valid());
        drop(b);
        // `a` still holds the flock after the nested guard drops.
        assert!(a.is_valid());
        drop(a);

        // Fully released — a fresh acquire takes a brand-new flock.
        let c = WriterLock::acquire(&rvf_path).unwrap();
        assert!(c.is_valid());
    }

    // Note: a cross-process "second-acquisition-blocks" test cannot run within
    // a single process (recursive flock from the same process refcounts here
    // instead of blocking). Cross-process behavior — the load-bearing case — is
    // exercised by the ruflo-patch interproc race harness and the
    // `tier3_concurrent_writers` integration test.
}
