//! Parses MPEG-TS and M2TS stream metadata.

use super::Probe;
use super::binary::{invalid, read_region};
use crate::meta::fields::{AudioCodec, VideoCodec};
use crate::probe::{AudioStream, VideoStream};
use std::fs::File;
use std::io;

const MAX_TS_SCAN_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Clone, Copy)]
struct Layout {
    size: usize,
    sync_offset: usize,
}

pub fn parse(file: &mut File, file_len: u64) -> io::Result<Probe> {
    let data = read_region(file, 0, file_len.min(MAX_TS_SCAN_BYTES), file_len)?;
    let layout = detect_layout(&data).ok_or_else(|| invalid("MPEG-TS packet sync not found"))?;
    let pat =
        collect_section(&data, layout, 0, 0x00)?.ok_or_else(|| invalid("MPEG-TS PAT not found"))?;
    let pmt_pid = parse_pat(&pat).ok_or_else(|| invalid("MPEG-TS PAT has no program"))?;
    let pmt = collect_section(&data, layout, pmt_pid, 0x02)?
        .ok_or_else(|| invalid("MPEG-TS PMT not found"))?;
    let (video_streams, audio_streams) = parse_pmt(&pmt)?;
    let mut probe = Probe::new(if layout.size == 192 { "m2ts" } else { "ts" });
    probe.video_streams = video_streams;
    probe.audio_streams = audio_streams;
    Ok(probe)
}

pub fn matches(file: &mut File, file_len: u64) -> io::Result<bool> {
    let data = read_region(file, 0, file_len.min(5 * 204), file_len)?;
    Ok(detect_layout(&data).is_some())
}

fn detect_layout(data: &[u8]) -> Option<Layout> {
    [
        Layout {
            size: 188,
            sync_offset: 0,
        },
        Layout {
            size: 192,
            sync_offset: 4,
        },
        Layout {
            size: 204,
            sync_offset: 0,
        },
    ]
    .into_iter()
    .find(|layout| {
        (0..5).all(|index| data.get(layout.sync_offset + index * layout.size) == Some(&0x47))
    })
}

fn collect_section(
    data: &[u8],
    layout: Layout,
    wanted_pid: u16,
    table_id: u8,
) -> io::Result<Option<Vec<u8>>> {
    let mut section = Vec::new();
    let mut expected = None;
    let mut started = false;
    let mut packet_start = 0_usize;
    while packet_start + layout.size <= data.len() {
        let packet = &data[packet_start..packet_start + layout.size];
        let sync = layout.sync_offset;
        if packet[sync] != 0x47 {
            return Err(invalid("MPEG-TS packet lost synchronization"));
        }
        let pid = (u16::from(packet[sync + 1] & 0x1F) << 8) | u16::from(packet[sync + 2]);
        let unit_start = packet[sync + 1] & 0x40 != 0;
        let adaptation = (packet[sync + 3] >> 4) & 0x03;
        if pid == wanted_pid && adaptation & 0x01 != 0 {
            let mut offset = sync + 4;
            if adaptation & 0x02 != 0 {
                let length = usize::from(
                    *packet
                        .get(offset)
                        .ok_or_else(|| invalid("truncated adaptation field"))?,
                );
                offset = offset
                    .checked_add(length + 1)
                    .ok_or_else(|| invalid("adaptation offset overflow"))?;
            }
            if offset > packet.len() {
                return Err(invalid("adaptation field exceeds packet"));
            }
            if unit_start {
                let pointer = usize::from(
                    *packet
                        .get(offset)
                        .ok_or_else(|| invalid("truncated PSI pointer"))?,
                );
                offset = offset
                    .checked_add(pointer + 1)
                    .ok_or_else(|| invalid("PSI pointer overflow"))?;
                section.clear();
                expected = None;
                started = true;
            }
            if started && offset < packet.len() {
                section.extend_from_slice(&packet[offset..]);
                if section.first() != Some(&table_id) {
                    started = false;
                    section.clear();
                } else if section.len() >= 3 && expected.is_none() {
                    let length = ((usize::from(section[1] & 0x0F)) << 8) | usize::from(section[2]);
                    expected = Some(3 + length);
                }
                if let Some(length) = expected
                    && section.len() >= length
                {
                    section.truncate(length);
                    return Ok(Some(section));
                }
            }
        }
        packet_start += layout.size;
    }
    Ok(None)
}

fn parse_pat(section: &[u8]) -> Option<u16> {
    let mut offset = 8_usize;
    let end = section.len().checked_sub(4)?;
    while offset + 4 <= end {
        let program = u16::from_be_bytes(section[offset..offset + 2].try_into().ok()?);
        let pid = (u16::from(section[offset + 2] & 0x1F) << 8) | u16::from(section[offset + 3]);
        if program != 0 {
            return Some(pid);
        }
        offset += 4;
    }
    None
}

fn parse_pmt(section: &[u8]) -> io::Result<(Vec<VideoStream>, Vec<AudioStream>)> {
    if section.len() < 16 {
        return Err(invalid("truncated MPEG-TS PMT"));
    }
    let program_info_len = ((usize::from(section[10] & 0x0F)) << 8) | usize::from(section[11]);
    let mut offset = 12_usize
        .checked_add(program_info_len)
        .ok_or_else(|| invalid("PMT program descriptor overflow"))?;
    let end = section
        .len()
        .checked_sub(4)
        .ok_or_else(|| invalid("truncated PMT checksum"))?;
    let mut video = Vec::new();
    let mut audio = Vec::new();
    while offset + 5 <= end {
        let stream_type = section[offset];
        let descriptor_len =
            ((usize::from(section[offset + 3] & 0x0F)) << 8) | usize::from(section[offset + 4]);
        let descriptor_start = offset + 5;
        let descriptor_end = descriptor_start
            .checked_add(descriptor_len)
            .ok_or_else(|| invalid("PMT descriptor overflow"))?;
        let descriptors = section
            .get(descriptor_start..descriptor_end)
            .ok_or_else(|| invalid("PMT descriptor exceeds section"))?;
        if let Some(codec) = video_stream_type(stream_type, descriptors) {
            video.push(VideoStream {
                codec: Some(codec),
                ..VideoStream::default()
            });
        }
        if let Some(codec) = audio_stream_type(stream_type, descriptors) {
            audio.push(AudioStream {
                codec: Some(codec),
                ..AudioStream::default()
            });
        }
        offset = descriptor_end;
    }
    Ok((video, audio))
}

fn video_stream_type(stream_type: u8, descriptors: &[u8]) -> Option<VideoCodec> {
    match stream_type {
        0x01 | 0x02 => Some(VideoCodec::H262),
        0x1B => Some(VideoCodec::H264),
        0x24 => Some(VideoCodec::H265),
        0xEA => Some(VideoCodec::Vc1),
        0x06 if registration(descriptors) == Some(*b"HEVC") => Some(VideoCodec::H265),
        _ => None,
    }
}

fn audio_stream_type(stream_type: u8, descriptors: &[u8]) -> Option<AudioCodec> {
    match stream_type {
        0x03 | 0x04 => Some(AudioCodec::Mp3),
        0x0F | 0x11 => Some(AudioCodec::Aac),
        0x81 => Some(AudioCodec::DolbyDigital),
        0x82 | 0x85 | 0x86 => Some(AudioCodec::Dts),
        0x87 => Some(AudioCodec::DolbyDigitalPlus),
        0x06 => descriptor_audio_codec(descriptors),
        _ => None,
    }
}

fn descriptor_audio_codec(descriptors: &[u8]) -> Option<AudioCodec> {
    let mut offset = 0_usize;
    while offset + 2 <= descriptors.len() {
        let tag = descriptors[offset];
        let length = usize::from(descriptors[offset + 1]);
        let end = offset.checked_add(2 + length)?;
        let payload = descriptors.get(offset + 2..end)?;
        match tag {
            0x05 if payload.starts_with(b"AC-3") => return Some(AudioCodec::DolbyDigital),
            0x05 if payload.starts_with(b"EAC3") => return Some(AudioCodec::DolbyDigitalPlus),
            0x05 if payload.starts_with(b"DTS") => return Some(AudioCodec::Dts),
            0x6A => return Some(AudioCodec::DolbyDigital),
            0x7A => return Some(AudioCodec::DolbyDigitalPlus),
            0x7B => return Some(AudioCodec::Dts),
            _ => {}
        }
        offset = end;
    }
    None
}

fn registration(descriptors: &[u8]) -> Option<[u8; 4]> {
    let mut offset = 0_usize;
    while offset + 2 <= descriptors.len() {
        let tag = descriptors[offset];
        let length = usize::from(descriptors[offset + 1]);
        let end = offset.checked_add(2 + length)?;
        let payload = descriptors.get(offset + 2..end)?;
        if tag == 0x05 && payload.len() >= 4 {
            return payload[..4].try_into().ok();
        }
        offset = end;
    }
    None
}

crate::unit_tests!("mpeg_ts.test.rs");
