//! Atomic root pointer for the RVF format — Phase 1 of ADR-0167.
//!
//! Implements the LMDB/redb-style double-buffered root header:
//!
//! - Two 64-byte [`RootSlot`]s at fixed offsets 0 and 64 in every new `.rvf`
//!   file.  Each slot carries a monotonic `txnid`, a manifest pointer, and a
//!   CRC32-IEEE checksum over all preceding fields.
//! - The active slot is the one with the higher *valid* `txnid`.  A torn slot
//!   (bad magic, zero txnid, wrong CRC) is silently skipped.
//! - Writers commit via [`write_slot`]: single `pwrite(2)` at the slot offset,
//!   then [`barrier_sync`] (ordering), then [`durable_sync`] (device-level
//!   durability).  The two-tier sync maps to AM-4 (`F_BARRIERFSYNC` /
//!   `F_FULLFSYNC` on Darwin; `fdatasync` / `fsync` on Linux).
//! - AM-1: the `file_identity` field detects stale-handle reads after
//!   `compact()` replaces the inode.
//! - AM-3: magic is `*b"RVFROOT\0"` (8 bytes; matches SFVR's pattern).
//! - AM-5: if the active slot's manifest CRC fails, the fallback slot's
//!   manifest is attempted before bubbling `RvfCorruptError`.

use std::fs::File;
use std::io;

use crc32fast::Hasher as Crc32Hasher;
use zerocopy::{AsBytes, FromBytes, FromZeroes};

use crate::fsync_helper::{barrier_sync, durable_sync};

// ── Layout constants ─────────────────────────────────────────────────────────

/// Total bytes reserved for the RootHeader (slot 0 + slot 1).
pub const ROOT_HEADER_SIZE: u64 = 128;

/// Size of a single [`RootSlot`].
pub const ROOT_SLOT_SIZE: u64 = 64;

/// Magic bytes identifying a valid [`RootSlot`] (AM-3).
pub const ROOT_MAGIC: [u8; 8] = *b"RVFROOT\0";

// ── RootSlot ─────────────────────────────────────────────────────────────────

/// One 64-byte root slot written at offset 0 or 64 of the `.rvf` file.
///
/// Layout (little-endian, fixed):
/// ```text
/// 0x00  8  magic            *b"RVFROOT\0"
/// 0x08  8  txnid            monotonic (0 = invalid)
/// 0x10  8  manifest_offset  absolute byte offset of manifest segment header
/// 0x18  8  manifest_len     payload length (excluding the 64-byte seg header)
/// 0x20  16 file_identity    copy of FileIdentity.file_id (AM-1)
/// 0x30  4  crc32            CRC32-IEEE over bytes [0x00..0x30]
/// 0x34  12 _padding         must be zero
/// ```
#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, FromZeroes)]
pub struct RootSlot {
    pub magic: [u8; 8],
    pub txnid_le: zerocopy::byteorder::U64<zerocopy::LE>,
    pub manifest_offset_le: zerocopy::byteorder::U64<zerocopy::LE>,
    pub manifest_len_le: zerocopy::byteorder::U64<zerocopy::LE>,
    pub file_identity: [u8; 16],
    pub crc32_le: zerocopy::byteorder::U32<zerocopy::LE>,
    pub _padding: [u8; 12],
}

// Compile-time size guard.
const _SLOT_SIZE_CHECK: () = assert!(
    std::mem::size_of::<RootSlot>() == ROOT_SLOT_SIZE as usize,
    "RootSlot must be exactly 64 bytes"
);

impl RootSlot {
    /// Return a zeroed slot.
    pub fn zeroed() -> Self {
        <Self as FromZeroes>::new_zeroed()
    }

    /// Build a slot with all data fields populated but CRC not yet computed.
    ///
    /// Call [`finalize`](Self::finalize) before writing to disk.
    pub fn new(
        txnid: u64,
        manifest_offset: u64,
        manifest_len: u64,
        file_identity: [u8; 16],
    ) -> Self {
        let mut s = Self::zeroed();
        s.magic = ROOT_MAGIC;
        s.txnid_le = zerocopy::byteorder::U64::new(txnid);
        s.manifest_offset_le = zerocopy::byteorder::U64::new(manifest_offset);
        s.manifest_len_le = zerocopy::byteorder::U64::new(manifest_len);
        s.file_identity = file_identity;
        s
    }

    /// True iff magic matches, txnid is non-zero, and CRC verifies.
    pub fn is_valid(&self) -> bool {
        self.magic == ROOT_MAGIC && self.txnid_le.get() != 0 && self.verify_crc()
    }

    pub fn txnid(&self) -> u64 {
        self.txnid_le.get()
    }

    pub fn manifest_offset(&self) -> u64 {
        self.manifest_offset_le.get()
    }

    pub fn manifest_len(&self) -> u64 {
        self.manifest_len_le.get()
    }

    pub fn file_identity(&self) -> [u8; 16] {
        self.file_identity
    }

    /// Compute and store the CRC32-IEEE over `[0x00..0x30]`.
    pub fn finalize(&mut self) {
        let crc = self.compute_crc();
        self.crc32_le = zerocopy::byteorder::U32::new(crc);
    }

    /// Return `true` iff `self.crc32_le` matches a fresh computation.
    pub fn verify_crc(&self) -> bool {
        self.crc32_le.get() == self.compute_crc()
    }

    // ── private ──────────────────────────────────────────────────────────────

    fn compute_crc(&self) -> u32 {
        // Cover magic(8) + txnid(8) + offset(8) + len(8) + identity(16) = 48 = 0x30.
        let covered = &self.as_bytes()[..0x30];
        let mut h = Crc32Hasher::new();
        h.update(covered);
        h.finalize()
    }
}

// ── RootHeaderError ───────────────────────────────────────────────────────────

/// Errors returned by RootHeader operations.
#[derive(Debug)]
pub enum RootHeaderError {
    Io(io::Error),
    /// Neither slot has valid magic + non-zero txnid + correct CRC.
    NoValidSlots,
    /// Both slots carry the `RVFROOT\0` magic but both CRCs are wrong
    /// (power-loss mid-write in both slots — extremely rare).
    BothSlotsTorn,
}

impl From<io::Error> for RootHeaderError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for RootHeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "root header I/O error: {e}"),
            Self::NoValidSlots => {
                write!(f, "no valid root slots (legacy file or first write not yet committed)")
            }
            Self::BothSlotsTorn => write!(
                f,
                "both root slots have torn CRCs — power-loss recovery required"
            ),
        }
    }
}

impl std::error::Error for RootHeaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Io(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

// ── RootHeader ────────────────────────────────────────────────────────────────

/// The parsed two-slot root header from offsets 0..128 of the file.
pub struct RootHeader {
    pub slot0: RootSlot,
    pub slot1: RootSlot,
}

impl RootHeader {
    /// Read both 64-byte slots from the file.
    ///
    /// Returns `Ok(RootHeader)` regardless of validity — call
    /// [`select_active`](Self::select_active) to check.
    /// Returns `Err(Io)` only on I/O failure.
    pub fn read(file: &File) -> Result<RootHeader, RootHeaderError> {
        let slot0 = read_slot_at(file, 0)?;
        let slot1 = read_slot_at(file, 1)?;
        Ok(RootHeader { slot0, slot1 })
    }

    /// Return the index (0 or 1) of the slot with the highest valid `txnid`.
    ///
    /// Selection rule (LMDB/redb txnid-comparison):
    /// - Both valid → higher txnid wins (slot 1 wins on tie).
    /// - One valid → that one.
    /// - Neither valid → `Err(NoValidSlots)`.
    pub fn select_active(&self) -> Result<u8, RootHeaderError> {
        match (self.slot0.is_valid(), self.slot1.is_valid()) {
            (true, true) => {
                if self.slot1.txnid() >= self.slot0.txnid() {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            (true, false) => Ok(0),
            (false, true) => Ok(1),
            (false, false) => Err(RootHeaderError::NoValidSlots),
        }
    }

    /// Return a reference to the active slot.
    pub fn active_slot(&self) -> Result<&RootSlot, RootHeaderError> {
        Ok(self.slot(self.select_active()?))
    }

    /// Return the index of the slot that should be overwritten on the next commit.
    pub fn inactive_slot_idx(&self) -> Result<u8, RootHeaderError> {
        Ok(1 - self.select_active()?)
    }

    fn slot(&self, idx: u8) -> &RootSlot {
        if idx == 0 { &self.slot0 } else { &self.slot1 }
    }
}

// ── Public write API ──────────────────────────────────────────────────────────

/// Initialise a brand-new file's RootHeader.
///
/// Slot 0 gets `txnid = 1`; slot 1 is zeroed/invalid so the next
/// [`commit_new_root`] writes there with `txnid = 2`.
///
/// Sync: barrier then durable (AM-4).
pub fn write_initial_header(
    file: &File,
    manifest_offset: u64,
    manifest_len: u64,
    file_identity: [u8; 16],
) -> io::Result<()> {
    let mut slot0 = RootSlot::new(1, manifest_offset, manifest_len, file_identity);
    slot0.finalize();
    let slot1 = RootSlot::zeroed();

    pwrite_slot_at(file, 0, &slot0)?;
    pwrite_slot_at(file, 1, &slot1)?;
    barrier_sync(file)?;
    durable_sync(file)?;
    Ok(())
}

/// Atomically commit a finalized slot at `slot_idx * 64` via pwrite + two-tier sync.
///
/// The slot must already have [`RootSlot::finalize`] called (CRC filled).
///
/// Sync order (AM-4):
/// 1. `pwrite` 64 bytes atomically at the slot offset.
/// 2. `barrier_sync` — orders the slot write after prior manifest writes.
/// 3. `durable_sync` — commits bytes to stable storage.
pub fn write_slot(file: &File, slot_idx: u8, slot: &RootSlot) -> io::Result<()> {
    pwrite_slot_at(file, slot_idx, slot)?;
    barrier_sync(file)?;
    durable_sync(file)?;
    Ok(())
}

/// Read current header, build a new slot in the inactive position, and commit.
///
/// Returns the new `txnid` on success.  Called from `store::write_manifest()`
/// after the manifest segment is already durably flushed.
pub fn commit_new_root(
    file: &File,
    manifest_offset: u64,
    manifest_len: u64,
    file_identity: [u8; 16],
) -> Result<u64, RootHeaderError> {
    let header = RootHeader::read(file)?;

    let (active_txnid, inactive_idx) = match header.select_active() {
        Ok(idx) => (header.slot(idx).txnid(), 1 - idx),
        Err(RootHeaderError::NoValidSlots) => (0, 0),
        Err(e) => return Err(e),
    };

    let new_txnid = active_txnid.saturating_add(1).max(1);
    let mut new_slot = RootSlot::new(new_txnid, manifest_offset, manifest_len, file_identity);
    new_slot.finalize();

    write_slot(file, inactive_idx, &new_slot)?;
    Ok(new_txnid)
}

// ── Platform-specific pread / pwrite ─────────────────────────────────────────

fn read_slot_at(file: &File, slot_idx: u8) -> Result<RootSlot, RootHeaderError> {
    let offset = slot_idx as u64 * ROOT_SLOT_SIZE;
    let mut buf = [0u8; ROOT_SLOT_SIZE as usize];
    let n = platform_pread(file, &mut buf, offset)?;
    if n < ROOT_SLOT_SIZE as usize {
        return Ok(RootSlot::zeroed());
    }
    // safe: buf is exactly ROOT_SLOT_SIZE bytes and RootSlot is FromBytes.
    Ok(RootSlot::read_from(&buf).expect("buffer length matches RootSlot size"))
}

fn pwrite_slot_at(file: &File, slot_idx: u8, slot: &RootSlot) -> io::Result<()> {
    let offset = slot_idx as u64 * ROOT_SLOT_SIZE;
    platform_pwrite(file, slot.as_bytes(), offset)
}

// ── Unix ──────────────────────────────────────────────────────────────────────

#[cfg(unix)]
fn platform_pread(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    use std::os::unix::io::AsRawFd;
    let fd = file.as_raw_fd();
    let rc = unsafe {
        libc::pread(
            fd,
            buf.as_mut_ptr() as *mut libc::c_void,
            buf.len(),
            offset as libc::off_t,
        )
    };
    if rc < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(rc as usize)
}

#[cfg(unix)]
fn platform_pwrite(file: &File, buf: &[u8], offset: u64) -> io::Result<()> {
    use std::os::unix::io::AsRawFd;
    let fd = file.as_raw_fd();
    let rc = unsafe {
        libc::pwrite(
            fd,
            buf.as_ptr() as *const libc::c_void,
            buf.len(),
            offset as libc::off_t,
        )
    };
    if rc != buf.len() as libc::ssize_t {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

// ── Windows ───────────────────────────────────────────────────────────────────

#[cfg(windows)]
fn platform_pread(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    use std::os::windows::fs::FileExt;
    file.seek_read(buf, offset)
}

#[cfg(windows)]
fn platform_pwrite(file: &File, buf: &[u8], offset: u64) -> io::Result<()> {
    use std::os::windows::fs::FileExt;
    let n = file.seek_write(buf, offset)?;
    if n != buf.len() {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero,
            "seek_write: short write on slot",
        ));
    }
    Ok(())
}

// ── Fallback (e.g. WASM, bare-metal) ─────────────────────────────────────────
// These targets do not have multi-process file semantics, so seek+read/write
// is equivalent in practice.

#[cfg(not(any(unix, windows)))]
fn platform_pread(file: &File, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = file.try_clone()?;
    f.seek(SeekFrom::Start(offset))?;
    f.read(buf)
}

#[cfg(not(any(unix, windows)))]
fn platform_pwrite(file: &File, buf: &[u8], offset: u64) -> io::Result<()> {
    use std::io::{Seek, SeekFrom, Write};
    let mut f = file.try_clone()?;
    f.seek(SeekFrom::Start(offset))?;
    let n = f.write(buf)?;
    if n != buf.len() {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero,
            "pwrite fallback: short write",
        ));
    }
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // 1. slot_round_trip
    #[test]
    fn slot_round_trip() {
        let identity = [0xAB_u8; 16];
        let mut slot = RootSlot::new(42, 128, 256, identity);
        slot.finalize();

        assert!(slot.is_valid(), "finalized slot must be valid");
        assert!(slot.verify_crc(), "CRC must verify after finalize");
        assert_eq!(slot.txnid(), 42);
        assert_eq!(slot.manifest_offset(), 128);
        assert_eq!(slot.manifest_len(), 256);
        assert_eq!(slot.file_identity(), identity);
    }

    // 2. crc_detects_corruption
    #[test]
    fn crc_detects_corruption() {
        let mut slot = RootSlot::new(7, 128, 64, [0u8; 16]);
        slot.finalize();
        assert!(slot.verify_crc(), "CRC should be good before corruption");

        // Flip a byte inside the txnid field (bytes 8..16 in the slot layout).
        slot.as_bytes_mut()[8] ^= 0xFF;

        assert!(!slot.verify_crc(), "corruption must invalidate CRC");
    }

    // 3. write_then_read_initial_header
    #[test]
    fn write_then_read_initial_header() {
        let tf = tempfile::NamedTempFile::new().unwrap();
        let file = tf.as_file();
        file.set_len(128).unwrap();

        let identity = [0x11_u8; 16];
        write_initial_header(file, 128, 64, identity).unwrap();

        let header = RootHeader::read(file).unwrap();
        assert!(header.slot0.is_valid(), "slot0 should be valid");
        assert_eq!(header.slot0.txnid(), 1, "slot0 txnid should be 1");
        assert_eq!(header.slot0.file_identity(), identity);

        assert!(!header.slot1.is_valid(), "slot1 should be invalid (zeroed)");
        assert_eq!(header.slot1.txnid(), 0);

        assert_eq!(header.select_active().unwrap(), 0, "slot0 should be active");
    }

    // 4. select_active_picks_higher_txnid
    #[test]
    fn select_active_picks_higher_txnid() {
        let mut s0 = RootSlot::new(5, 128, 64, [0u8; 16]);
        s0.finalize();
        let mut s1 = RootSlot::new(7, 256, 64, [0u8; 16]);
        s1.finalize();

        let header = RootHeader { slot0: s0, slot1: s1 };
        assert_eq!(
            header.select_active().unwrap(),
            1,
            "slot1 has higher txnid and should win"
        );
    }

    // 5. select_active_skips_torn_slot
    #[test]
    fn select_active_skips_torn_slot() {
        let mut s0 = RootSlot::new(5, 128, 64, [0u8; 16]);
        s0.finalize();

        // slot1: good magic + txnid, but CRC deliberately wrong.
        let mut s1 = RootSlot::new(9, 256, 64, [0u8; 16]);
        s1.finalize();
        s1.crc32_le = zerocopy::byteorder::U32::new(s1.crc32_le.get().wrapping_add(1));

        assert!(s0.is_valid());
        assert!(!s1.is_valid(), "torn slot1 must report invalid");

        let header = RootHeader { slot0: s0, slot1: s1 };
        assert_eq!(
            header.select_active().unwrap(),
            0,
            "slot0 is the only valid slot"
        );
    }

    // 6. select_active_errors_when_both_invalid
    #[test]
    fn select_active_errors_when_both_invalid() {
        let header = RootHeader {
            slot0: RootSlot::zeroed(),
            slot1: RootSlot::zeroed(),
        };
        assert!(
            matches!(header.select_active(), Err(RootHeaderError::NoValidSlots)),
            "zeroed header must return NoValidSlots"
        );
    }

    // 7. commit_new_root_alternates_slots
    #[test]
    fn commit_new_root_alternates_slots() {
        let tf = tempfile::NamedTempFile::new().unwrap();
        let file = tf.as_file();
        file.set_len(512).unwrap();

        let identity = [0x22_u8; 16];
        write_initial_header(file, 128, 32, identity).unwrap();

        // Initial: slot0 active (txnid=1).
        assert_eq!(RootHeader::read(file).unwrap().select_active().unwrap(), 0);

        // Commit 1 → slot1 (txnid=2).
        commit_new_root(file, 160, 32, identity).unwrap();
        let h1 = RootHeader::read(file).unwrap();
        assert_eq!(h1.select_active().unwrap(), 1);
        assert_eq!(h1.slot1.txnid(), 2);

        // Commit 2 → slot0 (txnid=3).
        commit_new_root(file, 192, 32, identity).unwrap();
        let h2 = RootHeader::read(file).unwrap();
        assert_eq!(h2.select_active().unwrap(), 0);
        assert_eq!(h2.slot0.txnid(), 3);
    }

    // 8. commit_new_root_increments_txnid
    #[test]
    fn commit_new_root_increments_txnid() {
        let tf = tempfile::NamedTempFile::new().unwrap();
        let file = tf.as_file();
        file.set_len(512).unwrap();

        let identity = [0u8; 16];
        write_initial_header(file, 128, 32, identity).unwrap();

        let t1 = commit_new_root(file, 160, 32, identity).unwrap();
        let t2 = commit_new_root(file, 192, 32, identity).unwrap();
        let t3 = commit_new_root(file, 224, 32, identity).unwrap();

        assert_eq!(t1, 2);
        assert_eq!(t2, 3);
        assert_eq!(t3, 4);
    }
}
