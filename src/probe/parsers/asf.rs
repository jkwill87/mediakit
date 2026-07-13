//! Parses ASF and WMV container metadata.

use super::binary::{fourcc, invalid, read_region, u16_le, u32_le, u64_le};
use super::{Probe, audio_layout, video_codec, video_resolution, wave_audio_codec};
use crate::probe::{AudioStream, VideoStream};
use std::fs::File;
use std::io;
use std::time::Duration;

const HEADER_GUID: [u8; 16] = [
    0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C,
];
const FILE_PROPERTIES_GUID: [u8; 16] = [
    0xA1, 0xDC, 0xAB, 0x8C, 0x47, 0xA9, 0xCF, 0x11, 0x8E, 0xE4, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65,
];
const STREAM_PROPERTIES_GUID: [u8; 16] = [
    0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF, 0x11, 0x8E, 0xE6, 0x00, 0xC0, 0x0C, 0x20, 0x53, 0x65,
];
const AUDIO_MEDIA_GUID: [u8; 16] = [
    0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B,
];
const VIDEO_MEDIA_GUID: [u8; 16] = [
    0xC0, 0xEF, 0x19, 0xBC, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0x00, 0x80, 0x5F, 0x5C, 0x44, 0x2B,
];

pub fn parse(file: &mut File, file_len: u64) -> io::Result<Probe> {
    let header = read_region(file, 0, 30, file_len)?;
    if header[..16] != HEADER_GUID {
        return Err(invalid("ASF header GUID missing"));
    }
    let header_size = u64_le(&header, 16)?;
    if header_size < 30 {
        return Err(invalid("invalid ASF header size"));
    }
    let data = read_region(file, 0, header_size, file_len)?;
    let object_count = usize::try_from(u32_le(&data, 24)?)
        .map_err(|_| invalid("ASF object count is too large"))?;
    let mut offset = 30_usize;
    let mut probe = Probe::new("wmv");
    for _ in 0..object_count {
        if offset + 24 > data.len() {
            return Err(invalid("truncated ASF object"));
        }
        let guid: [u8; 16] = data[offset..offset + 16]
            .try_into()
            .expect("sixteen-byte slice");
        let size = usize::try_from(u64_le(&data, offset + 16)?)
            .map_err(|_| invalid("ASF object is too large"))?;
        if size < 24 {
            return Err(invalid("invalid ASF object size"));
        }
        let end = offset
            .checked_add(size)
            .ok_or_else(|| invalid("ASF object offset overflow"))?;
        let payload = data
            .get(offset + 24..end)
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
    if data.len() < 80 {
        return Err(invalid("truncated ASF file properties"));
    }
    let play_duration = u64_le(data, 40)?;
    let preroll_ms = u64_le(data, 56)?;
    probe.duration = Some(
        Duration::from_nanos(play_duration.saturating_mul(100))
            .saturating_sub(Duration::from_millis(preroll_ms)),
    );
    Ok(())
}

fn parse_stream_properties(data: &[u8], probe: &mut Probe) -> io::Result<()> {
    if data.len() < 54 {
        return Err(invalid("truncated ASF stream properties"));
    }
    let stream_type: [u8; 16] = data[..16].try_into().expect("sixteen-byte slice");
    let type_size = usize::try_from(u32_le(data, 40)?)
        .map_err(|_| invalid("ASF stream type data is too large"))?;
    let type_data = data
        .get(54..54 + type_size)
        .ok_or_else(|| invalid("truncated ASF stream type data"))?;
    if stream_type == AUDIO_MEDIA_GUID {
        probe.audio_streams.push(parse_audio(type_data)?);
    } else if stream_type == VIDEO_MEDIA_GUID {
        probe.video_streams.push(parse_video(type_data)?);
    }
    Ok(())
}

fn parse_audio(data: &[u8]) -> io::Result<AudioStream> {
    if data.len() < 16 {
        return Err(invalid("truncated ASF audio format"));
    }
    let mut format_tag = u16_le(data, 0)?;
    let channels = u64::from(u16_le(data, 2)?);
    let average_bytes = u32_le(data, 8)?;
    let mut channel_mask = None;
    if format_tag == 0xFFFE && data.len() >= 40 {
        channel_mask = Some(u32_le(data, 20)?);
        format_tag = u16_le(data, 24)?;
    }
    Ok(AudioStream {
        codec: wave_audio_codec(format_tag),
        layout: audio_layout(channels, channel_mask),
        bit_rate: average_bytes.checked_mul(8),
        ..AudioStream::default()
    })
}

fn parse_video(data: &[u8]) -> io::Result<VideoStream> {
    if data.len() < 11 + 20 {
        return Err(invalid("truncated ASF video format"));
    }
    let format_size = usize::from(u16_le(data, 9)?);
    let bitmap = data
        .get(11..11 + format_size)
        .ok_or_else(|| invalid("truncated ASF bitmap format"))?;
    if bitmap.len() < 20 {
        return Err(invalid("truncated ASF bitmap header"));
    }
    let width = u64::from(u32_le(bitmap, 4)?);
    let height = i32::from_le_bytes(bitmap[8..12].try_into().expect("four-byte bitmap height"))
        .unsigned_abs()
        .into();
    let compression = fourcc(bitmap, 16)?;
    Ok(VideoStream {
        codec: video_codec(&compression),
        width: u32::try_from(width).ok().filter(|value| *value > 0),
        height: u32::try_from(height).ok().filter(|value| *value > 0),
        resolution: video_resolution(width, height, None),
        ..VideoStream::default()
    })
}

crate::unit_tests!("asf.test.rs");
