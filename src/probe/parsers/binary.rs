//! Bounded binary-reading primitives shared by container parsers.
//!
//! Container metadata is untrusted input. These helpers centralize checked
//! range arithmetic, the per-allocation limit, exact reads, explicit byte
//! order, and FourCC extraction so format parsers cannot silently truncate or
//! wrap an offset.

use std::io::{self, Read, Seek, SeekFrom};

/// Maximum size of any single metadata region copied into memory (64 MiB).
///
/// This is an allocation-safety limit, not a limit on media-file size. Parsers
/// may seek through larger files and read several separately bounded regions.
pub(super) const MAX_METADATA_BYTES: usize = 64 * 1024 * 1024;
/// Encoded width of a 16-bit integer.
const U16_BYTES: usize = 2;
/// Encoded width of a 32-bit integer.
const U32_BYTES: usize = 4;
/// Encoded width of a 64-bit integer.
const U64_BYTES: usize = 8;
/// Encoded width of a four-byte character code.
const FOURCC_BYTES: usize = 4;

/// Creates the common `InvalidData` error used for malformed containers.
pub(super) fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

/// Returns `offset + size` after checking both integer overflow and the
/// containing region's exclusive `limit`.
pub(super) fn checked_end(offset: u64, size: u64, limit: u64) -> io::Result<u64> {
    let end = offset
        .checked_add(size)
        .ok_or_else(|| invalid("media offset overflow"))?;
    if end > limit {
        return Err(invalid("media element exceeds parent"));
    }
    Ok(end)
}

/// Seeks to and copies an exactly sized region after validating its bounds and
/// the metadata allocation limit.
pub(super) fn read_region<R: Read + Seek>(
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

/// Reads a little-endian 16-bit unsigned integer at `offset`.
pub(super) fn u16_le(data: &[u8], offset: usize) -> io::Result<u16> {
    let bytes = data
        .get(offset..offset + U16_BYTES)
        .ok_or_else(|| invalid("truncated u16"))?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

/// Reads a little-endian 32-bit unsigned integer at `offset`.
pub(super) fn u32_le(data: &[u8], offset: usize) -> io::Result<u32> {
    let bytes = data
        .get(offset..offset + U32_BYTES)
        .ok_or_else(|| invalid("truncated u32"))?;
    Ok(u32::from_le_bytes(
        bytes.try_into().expect("four-byte slice"),
    ))
}

/// Reads a little-endian 32-bit signed integer at `offset`.
pub(super) fn i32_le(data: &[u8], offset: usize) -> io::Result<i32> {
    let bytes = data
        .get(offset..offset + U32_BYTES)
        .ok_or_else(|| invalid("truncated i32"))?;
    Ok(i32::from_le_bytes(
        bytes.try_into().expect("four-byte slice"),
    ))
}

/// Reads a little-endian 64-bit unsigned integer at `offset`.
pub(super) fn u64_le(data: &[u8], offset: usize) -> io::Result<u64> {
    let bytes = data
        .get(offset..offset + U64_BYTES)
        .ok_or_else(|| invalid("truncated u64"))?;
    Ok(u64::from_le_bytes(
        bytes.try_into().expect("eight-byte slice"),
    ))
}

/// Reads a big-endian 16-bit unsigned integer at `offset`.
pub(super) fn u16_be(data: &[u8], offset: usize) -> io::Result<u16> {
    let bytes = data
        .get(offset..offset + U16_BYTES)
        .ok_or_else(|| invalid("truncated u16"))?;
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

/// Reads a big-endian 32-bit unsigned integer at `offset`.
pub(super) fn u32_be(data: &[u8], offset: usize) -> io::Result<u32> {
    let bytes = data
        .get(offset..offset + U32_BYTES)
        .ok_or_else(|| invalid("truncated u32"))?;
    Ok(u32::from_be_bytes(
        bytes.try_into().expect("four-byte slice"),
    ))
}

/// Reads a big-endian 64-bit unsigned integer at `offset`.
pub(super) fn u64_be(data: &[u8], offset: usize) -> io::Result<u64> {
    let bytes = data
        .get(offset..offset + U64_BYTES)
        .ok_or_else(|| invalid("truncated u64"))?;
    Ok(u64::from_be_bytes(
        bytes.try_into().expect("eight-byte slice"),
    ))
}

/// Copies a four-byte character code at `offset` without assuming it is UTF-8.
pub(super) fn fourcc(data: &[u8], offset: usize) -> io::Result<[u8; FOURCC_BYTES]> {
    data.get(offset..offset + FOURCC_BYTES)
        .ok_or_else(|| invalid("truncated fourcc"))?
        .try_into()
        .map_err(|_| invalid("invalid fourcc"))
}
