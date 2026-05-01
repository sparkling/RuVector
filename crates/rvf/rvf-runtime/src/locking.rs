//! Writer lock management for single-writer / multi-reader concurrency.
//!
//! ADR-0095 amendment (2026-05-01, t3-2 fix — fork-only, sparkleideas/ruvector):
//! the original implementation used `O_CREAT|O_EXCL` with a PID-stamped lock
//! file and userspace stale-detection. Under N concurrent CLI processes that
//! design produced two failure modes:
//!
//!   1. **LockHeld budget exhaustion.** `O_CREAT|O_EXCL` returns `WouldBlock`
//!      immediately on contention, never blocks; callers had to retry from
//!      userspace with a 5s budget. The native `RvfStore` holds the lock for
//!      its entire lifetime (= whole CLI process), so the last-in-line writer
//!      among N=6 must wait for all N-1 prior writers to fully exit (~1-3s
//!      each) — easily exceeding the 5s retry budget.
//!
//!   2. **PID-stale-detection races.** Process death between writing the lock
//!      file and acquiring it leaves an orphan; if the next peer reads the
//!      stale lock and the dead PID is reused (or the OS recycles it before
//!      the 30s threshold elapses), the next acquirer is blocked or — worse
//!      — both peers think they hold the lock.
//!
//! The replacement: kernel `flock(LOCK_EX)` on a sibling `.lock` file that is
//! never unlinked. This gives us:
//!
//!   - **FIFO blocking.** `LOCK_EX` queues writers in the kernel; no
//!     userspace retry budget, no fairness pathologies.
//!   - **Auto-release on process death.** Kernel closes all fds when a
//!     process exits — including when N-API skips Rust `Drop` due to
//!     `process.exit(0)`. The flock is dropped with the fd. No stale-lock
//!     detection needed.
//!   - **Same-inode flock queue.** Because the lock file is never unlinked,
//!     concurrent opens all share the same inode, so all peers join the
//!     same kernel flock queue.
//!
//! The same pattern is already used by the fork-side `IngestLockGuard` in
//! `rvf-node/src/lib.rs:45-114`. This change extends it to cover the
//! `RvfStore::create/open/derive` paths previously guarded by the broken
//! O_EXCL `WriterLock`.
//!
//! Public API is preserved: `WriterLock::acquire`, `release`, `Drop`,
//! `is_valid`, and `lock_path_for` all still exist with the same signatures
//! so `store.rs` does not need changes. `acquire` now BLOCKS instead of
//! returning `WouldBlock` on contention — callers that wrapped the failure
//! in `LockHeld` retry loops will simply never see those errors.
//!
//! Platform support: Unix only (Linux, macOS, BSD). Windows gets a no-op
//! stub matching `IngestLockGuard`'s pattern (out of scope per ADR-0095).

use std::io;
use std::path::{Path, PathBuf};

/// Represents an acquired writer lock.
///
/// On Unix this owns a file descriptor with `flock(LOCK_EX)` held. On Drop
/// the fd is closed (which releases the flock automatically per POSIX
/// semantics), and we additionally call `flock(LOCK_UN)` explicitly so
/// queued peers wake up deterministically.
pub(crate) struct WriterLock {
    #[cfg(unix)]
    fd: libc::c_int,
    #[cfg(unix)]
    lock_path: PathBuf,
    #[cfg(not(unix))]
    _phantom: (),
}

impl WriterLock {
    /// Acquire the writer lock for the given RVF file path.
    ///
    /// **Blocking on Unix.** Waits in the kernel flock queue until any
    /// peer's lock is released. Returns `Ok(WriterLock)` on success or an
    /// `io::Error` only on syscall failure (ENFILE, EACCES, etc.) — never on
    /// contention.
    ///
    /// On non-Unix platforms this is currently a no-op (the previous PID-
    /// based implementation never worked correctly on Windows either, since
    /// `kill(pid, 0)` is Unix-specific). A real Windows implementation
    /// would use `LockFileEx`.
    pub(crate) fn acquire(rvf_path: &Path) -> io::Result<Self> {
        #[cfg(unix)]
        {
            use std::ffi::CString;
            use std::os::unix::ffi::OsStrExt;

            let lock_path = lock_path_for(rvf_path);
            let c_path = CString::new(lock_path.as_os_str().as_bytes())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

            // O_CREAT|O_RDWR, mode 0o644. The file is NEVER unlinked, so
            // concurrent opens across processes all refer to the same inode
            // and join the same kernel flock queue.
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

            // Blocking exclusive lock. LOCK_EX serializes every writer;
            // peers block in the kernel until we release (drop / close /
            // process death). No userspace retry budget; no PID-staleness
            // logic — the kernel handles all of that.
            let rc = unsafe { libc::flock(fd, libc::LOCK_EX) };
            if rc != 0 {
                let err = io::Error::last_os_error();
                unsafe { libc::close(fd) };
                return Err(err);
            }

            Ok(WriterLock { fd, lock_path })
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
    /// previous implementation (`store.rs:946-947` calls this in `close()`).
    pub(crate) fn release(self) -> io::Result<()> {
        // Drop runs automatically when `self` goes out of scope at end of
        // function; explicit drop is unnecessary but harmless.
        drop(self);
        Ok(())
    }

    /// Check if the lock is still held by us.
    ///
    /// Under the new flock-based design this is always `true` for a
    /// successfully-acquired guard until it is dropped — there is no
    /// "lock taken over" condition the way the PID-file design had. Kept
    /// for API compatibility with the old implementation's caller surface.
    #[allow(dead_code)]
    pub(crate) fn is_valid(&self) -> bool {
        #[cfg(unix)]
        {
            self.fd >= 0
        }
        #[cfg(not(unix))]
        {
            true
        }
    }
}

#[cfg(unix)]
impl Drop for WriterLock {
    fn drop(&mut self) {
        // Best-effort release. `close()` alone is enough to drop the flock
        // on every POSIX platform (kernel reaps fd state regardless), but
        // call `flock(LOCK_UN)` explicitly so queued peers wake up
        // deterministically even if fd cleanup is scheduler-deferred.
        //
        // Crucially: this runs on natural Rust drop. When N-API skips
        // Drop because of `process.exit(0)`, the kernel still releases
        // the flock when the process's fd table is torn down — so the
        // "leaked .lock" failure mode of the old PID-file design cannot
        // happen here. The lock_path field is unused at drop time but
        // kept so `is_valid()` could in principle re-open + check (left
        // as future work; current `is_valid()` just checks the fd
        // descriptor sentinel).
        unsafe {
            libc::flock(self.fd, libc::LOCK_UN);
            libc::close(self.fd);
        }
        let _ = &self.lock_path; // suppress dead-code warning
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

    // Note: a "second-acquisition-blocks" test cannot run within a single
    // process because Linux/macOS flock is not inherited by the same fd —
    // recursive flock calls from the same process succeed instead of
    // blocking. Cross-process behavior (the load-bearing case) is exercised
    // by `scripts/diag-rvf-interproc-race.mjs --trials 40` in the
    // ruflo-patch repo.
}
