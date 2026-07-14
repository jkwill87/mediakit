//! Parses stream metadata from MPEG transport-stream program tables.
//!
//! A transport stream interleaves fixed-size packets identified by 13-bit
//! packet identifiers (PIDs). Program Specific Information (PSI) maps those
//! PIDs to programs and elementary streams: the Program Association Table
//! (PAT) identifies a Program Map Table (PMT), and the PMT supplies stream-type
//! codes plus descriptors for codecs, subtitles, and languages. This parser
//! reassembles the first PAT and PMT from a bounded prefix; it never decodes
//! packetized elementary-stream payloads.
//!
//! The same 188-byte transport packet may be wrapped by a four-byte M2TS
//! timestamp prefix or followed by 16 Reed-Solomon parity bytes, producing the
//! 192- and 204-byte layouts recognized below.

use super::Probe;
use super::binary::{invalid, read_region};
use crate::meta::fields::{AudioCodec, Language, VideoCodec};
use crate::probe::{AudioStream, SubtitleStream, VideoStream};
use std::fs::File;
use std::io;

/// Maximum transport-stream prefix scanned for PAT and PMT sections (8 MiB).
///
/// PSI tables are normally repeated frequently. The bound keeps probing
/// deterministic when a malformed or unusual stream never presents them.
const MAX_TS_SCAN_BYTES: u64 = 8 * 1024 * 1024;

/// Physical packet stride and the location of the MPEG-TS sync byte within it.
#[derive(Clone, Copy)]
struct Layout {
    size: usize,
    sync_offset: usize,
}

pub fn parse(file: &mut File, file_len: u64) -> io::Result<Probe> {
    let data = read_region(file, 0, file_len.min(MAX_TS_SCAN_BYTES), file_len)?;
    let layout = detect_layout(&data).ok_or_else(|| invalid("MPEG-TS packet sync not found"))?;
    // PID 0 and table_id 0x00 are reserved for the PAT. The selected program's
    // PMT uses table_id 0x02 on the PID announced by that PAT.
    let pat =
        collect_section(&data, layout, 0, 0x00)?.ok_or_else(|| invalid("MPEG-TS PAT not found"))?;
    let pmt_pid = parse_pat(&pat).ok_or_else(|| invalid("MPEG-TS PAT has no program"))?;
    let pmt = collect_section(&data, layout, pmt_pid, 0x02)?
        .ok_or_else(|| invalid("MPEG-TS PMT not found"))?;
    let (video_streams, audio_streams, subtitle_streams) = parse_pmt(&pmt)?;
    let mut probe = Probe::new(if layout.size == 192 { "m2ts" } else { "ts" });
    probe.video_streams = video_streams;
    probe.audio_streams = audio_streams;
    probe.subtitle_streams = subtitle_streams;
    Ok(probe)
}

pub fn matches(file: &mut File, file_len: u64) -> io::Result<bool> {
    // Five sync points provide a stronger signature than a single 0x47 byte;
    // 204 is the largest supported physical packet stride.
    let data = read_region(file, 0, file_len.min(5 * 204), file_len)?;
    Ok(detect_layout(&data).is_some())
}

fn detect_layout(data: &[u8]) -> Option<Layout> {
    [
        // Plain ISO/IEC 13818-1 transport packets.
        Layout {
            size: 188,
            sync_offset: 0,
        },
        // Blu-ray M2TS: a four-byte arrival timestamp precedes each TS packet.
        Layout {
            size: 192,
            sync_offset: 4,
        },
        // Broadcast TS with 16 trailing Reed-Solomon error-correction bytes.
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
        // Every MPEG-TS packet starts with the fixed sync byte 0x47.
        if packet[sync] != 0x47 {
            return Err(invalid("MPEG-TS packet lost synchronization"));
        }
        // Header byte 1 carries payload_unit_start_indicator in bit 6 and the
        // high five PID bits; byte 2 carries the remaining eight PID bits.
        let pid = (u16::from(packet[sync + 1] & 0x1F) << 8) | u16::from(packet[sync + 2]);
        let unit_start = packet[sync + 1] & 0x40 != 0;
        // adaptation_field_control occupies header byte 3 bits 5..4. Bit zero
        // of the extracted value means payload is present; bit one means an
        // adaptation field precedes it.
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
                // PSI payloads beginning a new section start with a pointer
                // field whose value skips any bytes before the section.
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
                    // PSI section_length is the low 12 bits of bytes 1..2 and
                    // counts everything after those first three bytes,
                    // including the trailing CRC32.
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
    // A PAT has an eight-byte header, four bytes per program mapping, and a
    // four-byte CRC. Program number zero denotes network information rather
    // than a PMT, so the first nonzero program is selected.
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

fn parse_pmt(
    section: &[u8],
) -> io::Result<(Vec<VideoStream>, Vec<AudioStream>, Vec<SubtitleStream>)> {
    if section.len() < 16 {
        return Err(invalid("truncated MPEG-TS PMT"));
    }
    // PMT bytes 10..11 contain a 12-bit program_info_length. Each following
    // elementary-stream entry has a five-byte fixed header and a 12-bit
    // ES_info_length for its descriptor loop. The final four bytes are CRC32.
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
    let mut subtitles = Vec::new();
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
        let language = descriptor_language(descriptors);
        if let Some(codec) = video_stream_type(stream_type, descriptors) {
            video.push(VideoStream {
                language,
                codec: Some(codec),
                ..VideoStream::default()
            });
        }
        if let Some(codec) = audio_stream_type(stream_type, descriptors) {
            audio.push(AudioStream {
                language,
                codec: Some(codec),
                ..AudioStream::default()
            });
        }
        if let Some(codec) = subtitle_stream_type(stream_type, descriptors) {
            subtitles.push(SubtitleStream {
                language,
                codec: Some(codec.to_owned()),
                ..SubtitleStream::default()
            });
        }
        offset = descriptor_end;
    }
    Ok((video, audio, subtitles))
}

fn video_stream_type(stream_type: u8, descriptors: &[u8]) -> Option<VideoCodec> {
    // Values through 0x24 are ISO/IEC 13818-1 assignments; 0xEA is a common
    // Blu-ray private assignment. Private PES type 0x06 may instead carry a
    // registration descriptor naming HEVC.
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
    // 0x03/0x04 are MPEG audio, 0x0F/0x11 are AAC (ADTS/LATM), and values
    // 0x81..0x87 are common ATSC/Blu-ray private assignments for AC-3, DTS,
    // DTS variants, and E-AC-3. Type 0x06 relies on descriptors.
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

fn subtitle_stream_type(stream_type: u8, descriptors: &[u8]) -> Option<&'static str> {
    // 0x90 and 0x92 are Blu-ray private assignments. DVB subtitle and
    // teletext streams use generic private PES type 0x06 and are distinguished
    // by descriptor tags 0x59 and 0x56 respectively.
    match stream_type {
        0x90 => Some("pgs"),
        0x92 => Some("text"),
        0x06 if has_descriptor(descriptors, 0x59) => Some("dvb_subtitle"),
        0x06 if has_descriptor(descriptors, 0x56) => Some("teletext"),
        _ => None,
    }
}

fn has_descriptor(descriptors: &[u8], wanted: u8) -> bool {
    // PMT descriptors are a compact tag-length-value sequence.
    let mut offset = 0_usize;
    while offset + 2 <= descriptors.len() {
        let tag = descriptors[offset];
        let length = usize::from(descriptors[offset + 1]);
        let Some(end) = offset.checked_add(2 + length) else {
            return false;
        };
        if end > descriptors.len() {
            return false;
        }
        if tag == wanted {
            return true;
        }
        offset = end;
    }
    false
}

fn descriptor_language(descriptors: &[u8]) -> Option<Language> {
    // ISO 639 language (0x0A), teletext (0x56), and subtitling (0x59)
    // descriptors all begin their payload/entry with a three-byte ISO 639-2
    // language code.
    let mut offset = 0_usize;
    while offset + 2 <= descriptors.len() {
        let tag = descriptors[offset];
        let length = usize::from(descriptors[offset + 1]);
        let end = offset.checked_add(2 + length)?;
        let payload = descriptors.get(offset + 2..end)?;
        if matches!(tag, 0x0A | 0x56 | 0x59) && payload.len() >= 3 {
            let language = std::str::from_utf8(&payload[..3]).ok()?;
            if language.bytes().all(|byte| byte.is_ascii_alphabetic()) {
                return Language::from_identifier(language);
            }
        }
        offset = end;
    }
    None
}

fn descriptor_audio_codec(descriptors: &[u8]) -> Option<AudioCodec> {
    // Registration descriptor 0x05 carries a four-byte format identifier.
    // DVB also assigns dedicated descriptor tags 0x6A (AC-3), 0x7A (enhanced
    // AC-3), and 0x7B (DTS).
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
    // Return the four-byte format_identifier from registration descriptor
    // 0x05. Additional identification bytes, when present, are not needed.
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
