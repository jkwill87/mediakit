//! Parses ASF and WMV header-object metadata.
//!
//! Advanced Systems Format is an object graph: every object begins with a
//! 128-bit GUID and a little-endian 64-bit size that includes the 24-byte
//! object header. The top-level Header Object contains File Properties and one
//! Stream Properties Object per stream. This parser stops there; it does not
//! read ASF data packets.
//!
//! GUIDs below are written in their on-disk byte order. The first three GUID
//! fields are little-endian in the ASF byte stream, so their byte arrays look
//! different from the conventional hyphenated GUID spelling in the comments.

use super::binary::{fourcc, invalid, read_region, u16_le, u32_le, u64_le};
use super::{
    Probe, WAVE_FORMAT_EXTENSIBLE, audio_layout, video_codec, video_resolution, wave_audio_codec,
};
use crate::probe::{AudioStream, VideoStream};
use std::fs::File;
use std::io;
use std::mem::size_of;
use std::time::Duration;

/// ASF Header Object (`75B22630-668E-11CF-A6D9-00AA0062CE6C`).
const HEADER_GUID: [u8; ASF_GUID_BYTES] = [
    0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C,
];
/// ASF File Properties Object (`8CABDCA1-A947-11CF-8EE4-00C00C205365`).
const FILE_PROPERTIES_GUID: [u8; ASF_GUID_BYTES] = [
    0xA1, 0xDC, 0xAB, 0x8C, 0x47, 0xA9, 0xCF, 0x11, 0x8E, 0xE4, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65,
];
/// ASF Stream Properties Object (`B7DC0791-A9B7-11CF-8EE6-00C00C205365`).
const STREAM_PROPERTIES_GUID: [u8; ASF_GUID_BYTES] = [
    0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF, 0x11, 0x8E, 0xE6, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65,
];
/// ASF audio-media stream type (`F8699E40-5B4D-11CF-A8FD-00805F5C442B`).
const AUDIO_MEDIA_GUID: [u8; ASF_GUID_BYTES] = [
    0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B,
];
/// ASF video-media stream type (`BC19EFC0-5B4D-11CF-A8FD-00805F5C442B`).
const VIDEO_MEDIA_GUID: [u8; ASF_GUID_BYTES] = [
    0xC0, 0xEF, 0x19, 0xBC, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B,
];

/// Encoded width of every ASF GUID.
const ASF_GUID_BYTES: usize = 16;
/// Fixed bytes before the Header Object's child objects.
const HEADER_OBJECT_PREFIX_BYTES: usize = 30;
/// Offset of the Header Object's 64-bit size field.
const HEADER_OBJECT_SIZE_OFFSET: usize = 16;
/// Offset of the Header Object's 32-bit child-object count.
const HEADER_OBJECT_COUNT_OFFSET: usize = 24;
/// Bytes in every ASF child-object GUID-and-size header.
const OBJECT_HEADER_BYTES: usize = 24;
/// Offset of an ASF child object's 64-bit size field.
const OBJECT_SIZE_OFFSET: usize = 16;
/// Minimum File Properties payload needed through the preroll field.
const FILE_PROPERTIES_MIN_BYTES: usize = 80;
/// File Properties payload offset of Play Duration.
const FILE_PROPERTIES_PLAY_DURATION_OFFSET: usize = 40;
/// File Properties payload offset of Preroll.
const FILE_PROPERTIES_PREROLL_OFFSET: usize = 56;
/// Nanoseconds represented by one ASF Play Duration unit.
const ASF_DURATION_UNIT_NANOSECONDS: u64 = 100;
/// Fixed Stream Properties payload prefix before Type-Specific Data.
const STREAM_PROPERTIES_PREFIX_BYTES: usize = 54;
/// Stream Properties payload offset of Type-Specific Data length.
const STREAM_PROPERTIES_TYPE_SIZE_OFFSET: usize = 40;
/// Minimum encoded size of `WAVEFORMATEX` through `nBlockAlign`.
const WAVE_FORMAT_MIN_BYTES: usize = 16;
/// `WAVEFORMATEX.nChannels` offset.
const WAVE_CHANNELS_OFFSET: usize = 2;
/// `WAVEFORMATEX.nAvgBytesPerSec` offset.
const WAVE_AVERAGE_BYTES_OFFSET: usize = 8;
/// Minimum encoded size of `WAVEFORMATEXTENSIBLE` used by this parser.
const WAVE_EXTENSIBLE_MIN_BYTES: usize = 40;
/// `WAVEFORMATEXTENSIBLE.dwChannelMask` offset.
const WAVE_CHANNEL_MASK_OFFSET: usize = 20;
/// Offset of the format tag embedded at the start of the SubFormat GUID.
const WAVE_SUBFORMAT_TAG_OFFSET: usize = 24;
/// Fixed ASF video Type-Specific Data prefix before `BITMAPINFOHEADER`.
const ASF_VIDEO_PREFIX_BYTES: usize = 11;
/// Offset of the ASF video format-size field.
const ASF_VIDEO_FORMAT_SIZE_OFFSET: usize = 9;
/// Minimum `BITMAPINFOHEADER` size needed through the compression FourCC.
const BITMAP_HEADER_MIN_BYTES: usize = 20;
/// `BITMAPINFOHEADER.biWidth` offset.
const BITMAP_WIDTH_OFFSET: usize = 4;
/// `BITMAPINFOHEADER.biHeight` offset.
const BITMAP_HEIGHT_OFFSET: usize = 8;
/// `BITMAPINFOHEADER.biCompression` FourCC offset.
const BITMAP_COMPRESSION_OFFSET: usize = 16;

pub fn parse(file: &mut File, file_len: u64) -> io::Result<Probe> {
    // The fixed Header Object prefix is GUID (16), size (8), object count (4),
    // plus two reserved bytes. Child objects follow at byte 30.
    let header = read_region(file, 0, HEADER_OBJECT_PREFIX_BYTES as u64, file_len)?;
    if header[..ASF_GUID_BYTES] != HEADER_GUID {
        return Err(invalid("ASF header GUID missing"));
    }
    let header_size = u64_le(&header, HEADER_OBJECT_SIZE_OFFSET)?;
    if header_size < HEADER_OBJECT_PREFIX_BYTES as u64 {
        return Err(invalid("invalid ASF header size"));
    }
    let data = read_region(file, 0, header_size, file_len)?;
    let object_count = usize::try_from(u32_le(&data, HEADER_OBJECT_COUNT_OFFSET)?)
        .map_err(|_| invalid("ASF object count is too large"))?;
    let mut offset = HEADER_OBJECT_PREFIX_BYTES;
    let mut probe = Probe::new("wmv");
    for _ in 0..object_count {
        // Every ASF child-object size includes its 16-byte GUID and 8-byte
        // little-endian size field.
        if offset + OBJECT_HEADER_BYTES > data.len() {
            return Err(invalid("truncated ASF object"));
        }
        let guid: [u8; ASF_GUID_BYTES] = data[offset..offset + ASF_GUID_BYTES]
            .try_into()
            .expect("sixteen-byte slice");
        let size = usize::try_from(u64_le(&data, offset + OBJECT_SIZE_OFFSET)?)
            .map_err(|_| invalid("ASF object is too large"))?;
        if size < OBJECT_HEADER_BYTES {
            return Err(invalid("invalid ASF object size"));
        }
        let end = offset
            .checked_add(size)
            .ok_or_else(|| invalid("ASF object offset overflow"))?;
        let payload = data
            .get(offset + OBJECT_HEADER_BYTES..end)
            .ok_or_else(|| invalid("ASF object exceeds header"))?;
        if guid == FILE_PROPERTIES_GUID {
            parse_file_properties(payload, &mut probe)?;
        } else if guid == STREAM_PROPERTIES_GUID {
            parse_stream_properties(payload, &mut probe)?;
        }
        offset = end;
    }
    Ok(probe)
}

fn parse_file_properties(data: &[u8], probe: &mut Probe) -> io::Result<()> {
    if data.len() < FILE_PROPERTIES_MIN_BYTES {
        return Err(invalid("truncated ASF file properties"));
    }
    // Offsets are relative to the object payload (after its 24-byte header).
    // Play Duration is measured in 100-nanosecond units and includes Preroll;
    // Preroll itself is stored in milliseconds.
    let play_duration = u64_le(data, FILE_PROPERTIES_PLAY_DURATION_OFFSET)?;
    let preroll_ms = u64_le(data, FILE_PROPERTIES_PREROLL_OFFSET)?;
    probe.duration = Some(
        Duration::from_nanos(play_duration.saturating_mul(ASF_DURATION_UNIT_NANOSECONDS))
            .saturating_sub(Duration::from_millis(preroll_ms)),
    );
    Ok(())
}

fn parse_stream_properties(data: &[u8], probe: &mut Probe) -> io::Result<()> {
    // The 54-byte fixed prefix contains two stream-type GUIDs, time offset,
    // type/error-correction lengths, and stream flags. Type-Specific Data then
    // holds WAVEFORMATEX or ASF's video-format structure.
    if data.len() < STREAM_PROPERTIES_PREFIX_BYTES {
        return Err(invalid("truncated ASF stream properties"));
    }
    let stream_type: [u8; ASF_GUID_BYTES] = data[..ASF_GUID_BYTES]
        .try_into()
        .expect("sixteen-byte slice");
    let type_size = usize::try_from(u32_le(data, STREAM_PROPERTIES_TYPE_SIZE_OFFSET)?)
        .map_err(|_| invalid("ASF stream type data is too large"))?;
    let type_data = data
        .get(STREAM_PROPERTIES_PREFIX_BYTES..STREAM_PROPERTIES_PREFIX_BYTES + type_size)
        .ok_or_else(|| invalid("truncated ASF stream type data"))?;
    if stream_type == AUDIO_MEDIA_GUID {
        probe.audio_streams.push(parse_audio(type_data)?);
    } else if stream_type == VIDEO_MEDIA_GUID {
        probe.video_streams.push(parse_video(type_data)?);
    }
    Ok(())
}

fn parse_audio(data: &[u8]) -> io::Result<AudioStream> {
    if data.len() < WAVE_FORMAT_MIN_BYTES {
        return Err(invalid("truncated ASF audio format"));
    }
    let mut format_tag = u16_le(data, 0)?;
    let channels = u64::from(u16_le(data, WAVE_CHANNELS_OFFSET)?);
    let average_bytes = u32_le(data, WAVE_AVERAGE_BYTES_OFFSET)?;
    let mut channel_mask = None;
    // WAVE_FORMAT_EXTENSIBLE (0xFFFE) appends valid-bits metadata, a speaker
    // channel mask, and a SubFormat GUID whose first 16 bits repeat the actual
    // WAVE format tag.
    if format_tag == WAVE_FORMAT_EXTENSIBLE && data.len() >= WAVE_EXTENSIBLE_MIN_BYTES {
        channel_mask = Some(u32_le(data, WAVE_CHANNEL_MASK_OFFSET)?);
        format_tag = u16_le(data, WAVE_SUBFORMAT_TAG_OFFSET)?;
    }
    Ok(AudioStream {
        codec: wave_audio_codec(format_tag),
        layout: audio_layout(channels, channel_mask),
        bit_rate: average_bytes.checked_mul(u8::BITS),
        ..AudioStream::default()
    })
}

fn parse_video(data: &[u8]) -> io::Result<VideoStream> {
    if data.len() < ASF_VIDEO_PREFIX_BYTES + BITMAP_HEADER_MIN_BYTES {
        return Err(invalid("truncated ASF video format"));
    }
    // ASF video Type-Specific Data has an 11-byte prefix followed by a Windows
    // BITMAPINFOHEADER. Its signed height is negative for top-down images, so
    // only the magnitude is relevant to display resolution.
    let format_size = usize::from(u16_le(data, ASF_VIDEO_FORMAT_SIZE_OFFSET)?);
    let bitmap = data
        .get(ASF_VIDEO_PREFIX_BYTES..ASF_VIDEO_PREFIX_BYTES + format_size)
        .ok_or_else(|| invalid("truncated ASF bitmap format"))?;
    if bitmap.len() < BITMAP_HEADER_MIN_BYTES {
        return Err(invalid("truncated ASF bitmap header"));
    }
    let width = u64::from(u32_le(bitmap, BITMAP_WIDTH_OFFSET)?);
    let height = i32::from_le_bytes(
        bitmap[BITMAP_HEIGHT_OFFSET..BITMAP_HEIGHT_OFFSET + size_of::<i32>()]
            .try_into()
            .expect("four-byte bitmap height"),
    )
    .unsigned_abs()
    .into();
    let compression = fourcc(bitmap, BITMAP_COMPRESSION_OFFSET)?;
    Ok(VideoStream {
        codec: video_codec(&compression),
        width: u32::try_from(width).ok().filter(|value| *value > 0),
        height: u32::try_from(height).ok().filter(|value| *value > 0),
        resolution: video_resolution(width, height, None),
        ..VideoStream::default()
    })
}

crate::unit_tests!("asf.test.rs");
