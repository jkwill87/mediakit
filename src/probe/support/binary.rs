//! Bounded binary-reading primitives shared by container probes.
//!
//! Container metadata is untrusted input. These helpers centralize checked range arithmetic, the
//! per-allocation limit, exact reads, explicit byte order, and FourCC extraction so format probes
//! cannot silently truncate or wrap an offset.

use super::FOURCC_BYTES;
use std::io::{self, Read, Seek, SeekFrom};

/// Maximum size of any single metadata region copied into memory (64 MiB).
///
/// This is an allocation-safety limit, not a limit on media-file size. Probes may seek through
/// larger files and read several separately bounded regions.
pub(in crate::probe) const MAX_METADATA_BYTES: usize = 64 * 1024 * 1024;
/// Encoded width of a 16-bit integer.
const U16_BYTES: usize = 2;
/// Encoded width of a 32-bit integer.
const U32_BYTES: usize = 4;
/// Encoded width of a 64-bit integer.
const U64_BYTES: usize = 8;

/// Creates the common `InvalidData` error used for malformed containers.
pub(in crate::probe) fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

/// Returns `offset + size` after checking both integer overflow and the containing region's
/// exclusive `limit`.
pub(in crate::probe) fn checked_end(offset: u64, size: u64, limit: u64) -> io::Result<u64> {
    let end = offset
        .checked_add(size)
        .ok_or_else(|| invalid("media offset overflow"))?;
    if end > limit {
        return Err(invalid("media element exceeds parent"));
    }
    Ok(end)
}

/// Seeks to and copies an exactly sized region after validating its bounds and the metadata
/// allocation limit.
pub(in crate::probe) fn read_region<R: Read + Seek>(
    reader: &mut R,
    offset: u64,
    size: u64,
    limit: u64,
) -> io::Result<Vec<u8>> {
    checked_end(offset, size, limit)?;
    let size = usize::try_from(size).map_err(|_| invalid("media element is too large"))?;
    if size > MAX_METADATA_BYTES {
        return Err(invalid("media metadata exceeds allocation limit"));
    }
    reader.seek(SeekFrom::Start(offset))?;
    let mut data = vec![0; size];
    reader.read_exact(&mut data)?;
    Ok(data)
}

/// Copies a fixed-width byte array at `offset` with checked range arithmetic.
fn read_array<const N: usize>(
    data: &[u8],
    offset: usize,
    truncated: &'static str,
) -> io::Result<[u8; N]> {
    let end = offset.checked_add(N).ok_or_else(|| invalid(truncated))?;
    data.get(offset..end)
        .ok_or_else(|| invalid(truncated))?
        .try_into()
        .map_err(|_| invalid(truncated))
}

/// Reads a little-endian 16-bit unsigned integer at `offset`.
pub(in crate::probe) fn u16_le(data: &[u8], offset: usize) -> io::Result<u16> {
    Ok(u16::from_le_bytes(read_array::<U16_BYTES>(
        data,
        offset,
        "truncated u16",
    )?))
}

/// Reads a little-endian 32-bit unsigned integer at `offset`.
pub(in crate::probe) fn u32_le(data: &[u8], offset: usize) -> io::Result<u32> {
    Ok(u32::from_le_bytes(read_array::<U32_BYTES>(
        data,
        offset,
        "truncated u32",
    )?))
}

/// Reads a little-endian 32-bit signed integer at `offset`.
pub(in crate::probe) fn i32_le(data: &[u8], offset: usize) -> io::Result<i32> {
    Ok(i32::from_le_bytes(read_array::<U32_BYTES>(
        data,
        offset,
        "truncated i32",
    )?))
}

/// Reads a little-endian 64-bit unsigned integer at `offset`.
pub(in crate::probe) fn u64_le(data: &[u8], offset: usize) -> io::Result<u64> {
    Ok(u64::from_le_bytes(read_array::<U64_BYTES>(
        data,
        offset,
        "truncated u64",
    )?))
}

/// Reads a big-endian 16-bit unsigned integer at `offset`.
pub(in crate::probe) fn u16_be(data: &[u8], offset: usize) -> io::Result<u16> {
    Ok(u16::from_be_bytes(read_array::<U16_BYTES>(
        data,
        offset,
        "truncated u16",
    )?))
}

/// Reads a big-endian 32-bit unsigned integer at `offset`.
pub(in crate::probe) fn u32_be(data: &[u8], offset: usize) -> io::Result<u32> {
    Ok(u32::from_be_bytes(read_array::<U32_BYTES>(
        data,
        offset,
        "truncated u32",
    )?))
}

/// Reads a big-endian 64-bit unsigned integer at `offset`.
pub(in crate::probe) fn u64_be(data: &[u8], offset: usize) -> io::Result<u64> {
    Ok(u64::from_be_bytes(read_array::<U64_BYTES>(
        data,
        offset,
        "truncated u64",
    )?))
}

/// Copies a four-byte character code at `offset` without assuming it is UTF-8.
pub(in crate::probe) fn fourcc(data: &[u8], offset: usize) -> io::Result<[u8; FOURCC_BYTES]> {
    read_array::<FOURCC_BYTES>(data, offset, "truncated fourcc")
}
