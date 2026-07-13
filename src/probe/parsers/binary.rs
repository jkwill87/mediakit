//! Implements bounded binary-reading primitives for container parsing.

use std::io::{self, Read, Seek, SeekFrom};

pub(super) const MAX_METADATA_BYTES: usize = 64 * 1024 * 1024;

pub(super) fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

pub(super) fn checked_end(offset: u64, size: u64, limit: u64) -> io::Result<u64> {
    let end = offset
        .checked_add(size)
        .ok_or_else(|| invalid("media offset overflow"))?;
    if end > limit {
        return Err(invalid("media element exceeds parent"));
    }
    Ok(end)
}

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

pub(super) fn u16_le(data: &[u8], offset: usize) -> io::Result<u16> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or_else(|| invalid("truncated u16"))?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

pub(super) fn u32_le(data: &[u8], offset: usize) -> io::Result<u32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| invalid("truncated u32"))?;
    Ok(u32::from_le_bytes(
        bytes.try_into().expect("four-byte slice"),
    ))
}

pub(super) fn i32_le(data: &[u8], offset: usize) -> io::Result<i32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| invalid("truncated i32"))?;
    Ok(i32::from_le_bytes(
        bytes.try_into().expect("four-byte slice"),
    ))
}

pub(super) fn u64_le(data: &[u8], offset: usize) -> io::Result<u64> {
    let bytes = data
        .get(offset..offset + 8)
        .ok_or_else(|| invalid("truncated u64"))?;
    Ok(u64::from_le_bytes(
        bytes.try_into().expect("eight-byte slice"),
    ))
}

pub(super) fn u16_be(data: &[u8], offset: usize) -> io::Result<u16> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or_else(|| invalid("truncated u16"))?;
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

pub(super) fn u32_be(data: &[u8], offset: usize) -> io::Result<u32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| invalid("truncated u32"))?;
    Ok(u32::from_be_bytes(
        bytes.try_into().expect("four-byte slice"),
    ))
}

pub(super) fn u64_be(data: &[u8], offset: usize) -> io::Result<u64> {
    let bytes = data
        .get(offset..offset + 8)
        .ok_or_else(|| invalid("truncated u64"))?;
    Ok(u64::from_be_bytes(
        bytes.try_into().expect("eight-byte slice"),
    ))
}

pub(super) fn fourcc(data: &[u8], offset: usize) -> io::Result<[u8; 4]> {
    data.get(offset..offset + 4)
        .ok_or_else(|| invalid("truncated fourcc"))?
        .try_into()
        .map_err(|_| invalid("invalid fourcc"))
}
