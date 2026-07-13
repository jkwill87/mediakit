//! Parses AVI container metadata.

use super::binary::{fourcc, i32_le, invalid, read_region, u16_le, u32_le};
use super::{Probe, audio_layout, video_codec, video_resolution, wave_audio_codec};
use crate::probe::{AudioStream, VideoStream};
use std::fs::File;
use std::io;
use std::time::Duration;

const MAX_AVI_HEADER_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Default)]
struct Stream {
    kind: Option<[u8; 4]>,
    handler: Option<[u8; 4]>,
    disabled: bool,
    scale: u32,
    rate: u32,
    length: u32,
    format: Vec<u8>,
}

pub fn parse(file: &mut File, file_len: u64) -> io::Result<Probe> {
    let data = read_region(file, 0, file_len.min(MAX_AVI_HEADER_BYTES), file_len)?;
    if data.len() < 12 || &data[..4] != b"RIFF" || &data[8..12] != b"AVI " {
        return Err(invalid("AVI RIFF header missing"));
    }
    let mut probe = Probe::new("avi");
    let mut microseconds_per_frame = 0_u32;
    let mut total_frames = 0_u32;
    let mut width = 0_u64;
    let mut height = 0_u64;
    let mut streams = Vec::new();
    parse_chunks(
        &data[12..],
        &mut microseconds_per_frame,
        &mut total_frames,
        &mut width,
        &mut height,
        &mut streams,
    )?;

    if microseconds_per_frame > 0 && total_frames > 0 {
        probe.duration = Some(Duration::from_micros(
            u64::from(microseconds_per_frame).saturating_mul(u64::from(total_frames)),
        ));
    }
    for stream in &streams {
        match stream.kind.as_ref() {
            Some(b"vids") => {
                let video = video_stream(stream, width, height, microseconds_per_frame)?;
                if probe.duration.is_none()
                    && stream.scale > 0
                    && stream.rate > 0
                    && stream.length > 0
                {
                    probe.duration = Duration::try_from_secs_f64(
                        f64::from(stream.length) * f64::from(stream.scale) / f64::from(stream.rate),
                    )
                    .ok();
                }
                probe.video_streams.push(video);
            }
            Some(b"auds") => probe.audio_streams.push(audio_stream(stream)?),
            _ => {}
        }
    }
    Ok(probe)
}

fn video_stream(
    stream: &Stream,
    default_width: u64,
    default_height: u64,
    microseconds_per_frame: u32,
) -> io::Result<VideoStream> {
    let compression = stream
        .format
        .get(16..20)
        .and_then(|value| value.try_into().ok())
        .or(stream.handler);
    let (width, height) = if stream.format.len() >= 12 {
        (
            u64::from(i32_le(&stream.format, 4)?.unsigned_abs()),
            u64::from(i32_le(&stream.format, 8)?.unsigned_abs()),
        )
    } else {
        (default_width, default_height)
    };
    let frame_rate = if stream.scale > 0 && stream.rate > 0 {
        Some(stream.rate as f32 / stream.scale as f32)
    } else if microseconds_per_frame > 0 {
        Some(1_000_000.0 / microseconds_per_frame as f32)
    } else {
        None
    };
    Ok(VideoStream {
        is_enabled: !stream.disabled,
        is_default: false,
        codec: compression.as_ref().and_then(video_codec),
        profile: None,
        width: u32::try_from(width).ok().filter(|value| *value > 0),
        height: u32::try_from(height).ok().filter(|value| *value > 0),
        resolution: video_resolution(width, height, None),
        frame_rate,
        dynamic_range: None,
    })
}

fn audio_stream(stream: &Stream) -> io::Result<AudioStream> {
    if stream.format.len() < 16 {
        return Ok(AudioStream {
            is_enabled: !stream.disabled,
            ..AudioStream::default()
        });
    }
    let mut format_tag = u16_le(&stream.format, 0)?;
    let channels = u64::from(u16_le(&stream.format, 2)?);
    let average_bytes = u32_le(&stream.format, 8)?;
    let mut channel_mask = None;
    if format_tag == 0xFFFE && stream.format.len() >= 40 {
        channel_mask = Some(u32_le(&stream.format, 20)?);
        format_tag = u16_le(&stream.format, 24)?;
    }
    Ok(AudioStream {
        is_enabled: !stream.disabled,
        is_default: false,
        codec: wave_audio_codec(format_tag),
        profile: None,
        layout: audio_layout(channels, channel_mask),
        bit_rate: average_bytes.checked_mul(8),
    })
}

fn parse_chunks(
    data: &[u8],
    microseconds_per_frame: &mut u32,
    total_frames: &mut u32,
    width: &mut u64,
    height: &mut u64,
    streams: &mut Vec<Stream>,
) -> io::Result<()> {
    let mut offset = 0_usize;
    while offset + 8 <= data.len() {
        let kind = fourcc(data, offset)?;
        let size = usize::try_from(u32_le(data, offset + 4)?)
            .map_err(|_| invalid("AVI chunk is too large"))?;
        let payload_start = offset + 8;
        let end = payload_start
            .checked_add(size)
            .ok_or_else(|| invalid("AVI chunk offset overflow"))?;
        if end > data.len() {
            if kind == *b"LIST" && data.get(payload_start..payload_start + 4) == Some(b"movi") {
                break;
            }
            return Err(invalid("AVI chunk exceeds parent"));
        }
        let payload = &data[payload_start..end];
        match &kind {
            b"avih" if payload.len() >= 40 => {
                *microseconds_per_frame = u32_le(payload, 0)?;
                *total_frames = u32_le(payload, 16)?;
                *width = u64::from(u32_le(payload, 32)?);
                *height = u64::from(u32_le(payload, 36)?);
            }
            b"LIST" if payload.len() >= 4 && &payload[..4] == b"hdrl" => {
                parse_chunks(
                    &payload[4..],
                    microseconds_per_frame,
                    total_frames,
                    width,
                    height,
                    streams,
                )?;
            }
            b"LIST" if payload.len() >= 4 && &payload[..4] == b"strl" => {
                streams.push(parse_stream(&payload[4..])?);
            }
            _ => {}
        }
        offset = end + (size & 1);
    }
    Ok(())
}

fn parse_stream(data: &[u8]) -> io::Result<Stream> {
    let mut stream = Stream::default();
    let mut offset = 0_usize;
    while offset + 8 <= data.len() {
        let kind = fourcc(data, offset)?;
        let size = usize::try_from(u32_le(data, offset + 4)?)
            .map_err(|_| invalid("AVI stream chunk is too large"))?;
        let start = offset + 8;
        let end = start
            .checked_add(size)
            .ok_or_else(|| invalid("AVI stream chunk offset overflow"))?;
        let payload = data
            .get(start..end)
            .ok_or_else(|| invalid("AVI stream chunk exceeds parent"))?;
        match &kind {
            b"strh" if payload.len() >= 36 => {
                stream.kind = Some(fourcc(payload, 0)?);
                stream.handler = Some(fourcc(payload, 4)?);
                stream.disabled = u32_le(payload, 8)? & 1 != 0;
                stream.scale = u32_le(payload, 20)?;
                stream.rate = u32_le(payload, 24)?;
                stream.length = u32_le(payload, 32)?;
            }
            b"strf" => stream.format = payload.to_vec(),
            _ => {}
        }
        offset = end + (size & 1);
    }
    Ok(stream)
}

crate::unit_tests!("avi.test.rs");
