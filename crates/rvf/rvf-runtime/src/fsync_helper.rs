//! Two-tier durability sync helpers for the RVF atomic root-pointer protocol.
//!
//! ADR-0167 (Amendment AM-4, 2026-05-10) identifies two distinct sync points
//! in the root-slot flip path:
//!
//!   1. **Ordering barrier** (`barrier_sync`): guarantees that all writes issued
//!      before the call are observable in-order by subsequent writes, without
//!      requiring a full device flush. On Darwin this maps to `F_BARRIERFSYNC`,
//!      which skips the drive's hardware write-cache drain. AM-4 benchmarks show
//!      a ~40% wall-time reduction on the `fsync` budget at N=8 concurrent writers
//!      when using `F_BARRIERFSYNC` for the manifest→slot ordering point.
//!
//!   2. **Durable commit** (`durable_sync`): guarantees the write has reached
//!      stable storage past the device's write cache. On Darwin this maps to
//!      `F_FULLFSYNC`, which issues a hardware FLUSH CACHE command. Used only at
//!      the final RootSlot commit point where crash-recovery correctness requires
//!      true durability.
//!
//! Both functions accept a `&std::fs::File` and return `std::io::Result<()>`.
//! On non-Darwin Unix platforms `barrier_sync` calls `fdatasync(2)` (sufficient
//! for ordering since a RootSlot flip does not change file size or mtime).

use std::io;

/// Ordering-only sync — guarantees write ordering without a device-level flush.
///
/// See the module-level documentation for the two-tier model. Use this between
/// the manifest-segment write and the root-slot flip to enforce ordering.
#[inline]
pub fn barrier_sync(file: &std::fs::File) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        use std::os::unix::io::AsRawFd;

        // F_BARRIERFSYNC = 0x55 = 85. Exposed in libc ≥ 0.2.155; if the
        // bound libc is older the constant may be absent — declare a local
        // fallback so we compile either way.
        #[allow(dead_code)]
        const F_BARRIERFSYNC: libc::c_int = 85; // 0x55

        let fd = file.as_raw_fd();
        let rc = unsafe { libc::fcntl(fd, F_BARRIERFSYNC) };
        if rc == -1 {
            let err = io::Error::last_os_error();
            // EINVAL: kernel does not support F_BARRIERFSYNC (pre-macOS 12
            // or a virtual device). Fall back to fdatasync-equivalent.
            if err.raw_os_error() == Some(libc::EINVAL) {
                return file.sync_data();
            }
            return Err(err);
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        // fdatasync(2): flushes data without forcing metadata (size/mtime)
        // to disk. Sufficient for ordering when the RootSlot flip does not
        // change file length.
        file.sync_data()
    }

    #[cfg(target_os = "windows")]
    {
        // std maps sync_data() to FlushFileBuffers on Windows.
        file.sync_data()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        file.sync_data()
    }
}

/// Device-level durable commit — guarantees writes are on stable storage.
///
/// See the module-level documentation for the two-tier model. Use this at the
/// final RootSlot commit point where crash-recovery correctness requires true
/// durability past the device's write cache.
#[inline]
pub fn durable_sync(file: &std::fs::File) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        use std::os::unix::io::AsRawFd;

        // F_FULLFSYNC = 0x33 = 51. Issues a hardware FLUSH CACHE command.
        #[allow(dead_code)]
        const F_FULLFSYNC: libc::c_int = 51; // 0x33

        let fd = file.as_raw_fd();
        let rc = unsafe { libc::fcntl(fd, F_FULLFSYNC) };
        if rc == -1 {
            let err = io::Error::last_os_error();
            // EINVAL: kernel does not support F_FULLFSYNC. Fall back to
            // fsync-equivalent which is the best we can do.
            if err.raw_os_error() == Some(libc::EINVAL) {
                return file.sync_all();
            }
            return Err(err);
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        file.sync_all()
    }

    #[cfg(target_os = "windows")]
    {
        file.sync_all()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        file.sync_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    #[test]
    fn barrier_sync_on_tempfile_returns_ok() {
        let mut tf = tempfile::NamedTempFile::new().unwrap();
        tf.write_all(b"\x00").unwrap();
        assert!(barrier_sync(tf.as_file()).is_ok());
    }

    #[test]
    fn durable_sync_on_tempfile_returns_ok() {
        let mut tf = tempfile::NamedTempFile::new().unwrap();
        tf.write_all(b"\x00").unwrap();
        assert!(durable_sync(tf.as_file()).is_ok());
    }

    /// On macOS the actual `F_BARRIERFSYNC` / `F_FULLFSYNC` paths run above;
    /// this test confirms they succeed on an ordinary tempfile (the EINVAL
    /// fallback fires only on very old kernels or non-regular files, so on a
    /// modern macOS CI machine we expect `rc == 0` directly — either way the
    /// function must return `Ok`).
    #[test]
    fn macos_einval_fallback_does_not_panic() {
        let mut tf = tempfile::NamedTempFile::new().unwrap();
        tf.write_all(b"\x42").unwrap();
        // Both calls must return Ok regardless of whether the fcntl
        // succeeds or falls back through the EINVAL branch.
        assert!(barrier_sync(tf.as_file()).is_ok());
        assert!(durable_sync(tf.as_file()).is_ok());
    }
}
