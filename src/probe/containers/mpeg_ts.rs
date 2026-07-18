//! Detects MPEG transport streams and probes metadata from their program tables.
//!
//! A transport stream interleaves fixed-size packets identified by 13-bit packet identifiers
//! (PIDs). Program Specific Information (PSI) maps those PIDs to programs and elementary streams:
//! the Program Association Table (PAT) identifies a Program Map Table (PMT), and the PMT supplies
//! stream-type codes plus descriptors for codecs, subtitles, and languages. This probe reassembles
//! the first PAT and PMT from a bounded prefix; it never decodes packetized elementary-stream
//! payloads.
//!
//! The same 188-byte transport packet may be wrapped by a four-byte M2TS timestamp prefix or
//! followed by 16 Reed-Solomon parity bytes, producing the 192- and 204-byte layouts recognized
//! below.

use super::binary::{invalid, read_region};
use super::{MediaInfo, ProbeInput};
use crate::meta::fields::{AudioCodec, Language, MediaFormat, VideoCodec};
use crate::probe::{AudioStream, StreamInfo, SubtitleStream, VideoStream};
use std::io;

/// Maximum transport-stream prefix scanned for PAT and PMT sections (8 MiB).
///
/// PSI tables are normally repeated frequently. The bound keeps probing deterministic when a
/// malformed or unusual stream never presents them.
const MAX_TS_SCAN_BYTES: u64 = 8 * 1024 * 1024;
/// Bytes in a plain ISO/IEC 13818-1 transport packet.
const TS_PACKET_BYTES: usize = 188;
/// Bytes in an M2TS packet including its four-byte arrival timestamp.
const M2TS_PACKET_BYTES: usize = 192;
/// Bytes in a broadcast TS packet including Reed-Solomon parity.
const FEC_TS_PACKET_BYTES: usize = 204;
/// M2TS offset from the arrival timestamp to the TS sync byte.
const M2TS_SYNC_OFFSET: usize = 4;
/// Consecutive packet sync points required to recognize a layout.
const SYNC_CHECK_PACKETS: usize = 5;
/// Maximum leading bytes required to recognize every supported transport-packet layout.
pub(super) const DETECTION_BYTES: usize = SYNC_CHECK_PACKETS * FEC_TS_PACKET_BYTES;
/// Fixed MPEG-TS packet sync byte.
const TS_SYNC_BYTE: u8 = 0x47;
/// PID reserved for the Program Association Table.
const PAT_PID: u16 = 0x0000;
/// PSI table identifier for the Program Association Table.
const PAT_TABLE_ID: u8 = 0x00;
/// PSI table identifier for a Program Map Table.
const PMT_TABLE_ID: u8 = 0x02;
/// Transport-header offset of the payload-start flag and high PID bits.
const TS_PID_HIGH_OFFSET: usize = 1;
/// Transport-header offset of the low PID bits.
const TS_PID_LOW_OFFSET: usize = 2;
/// Mask selecting the five high PID bits from the transport header.
const TS_PID_HIGH_MASK: u8 = 0x1F;
/// Flag indicating that a packet payload begins a PSI/PES unit.
const TS_PAYLOAD_UNIT_START: u8 = 0x40;
/// Transport-header offset of scrambling, adaptation, and continuity fields.
const TS_CONTROL_OFFSET: usize = 3;
/// Shift exposing `adaptation_field_control` from the control byte.
const TS_ADAPTATION_CONTROL_SHIFT: u32 = 4;
/// Mask selecting the two `adaptation_field_control` bits.
const TS_ADAPTATION_CONTROL_MASK: u8 = 0x03;
/// `adaptation_field_control` bit indicating payload presence.
const TS_PAYLOAD_PRESENT: u8 = 0x01;
/// `adaptation_field_control` bit indicating adaptation-field presence.
const TS_ADAPTATION_PRESENT: u8 = 0x02;
/// Bytes in the fixed MPEG-TS packet header.
const TS_HEADER_BYTES: usize = 4;
/// Bytes occupied by the adaptation-field length prefix.
const ADAPTATION_LENGTH_BYTES: usize = 1;
/// Bytes occupied by the PSI pointer field itself.
const PSI_POINTER_BYTES: usize = 1;
/// Bytes in a PSI table ID and 12-bit section-length prefix.
const PSI_SECTION_HEADER_BYTES: usize = 3;
/// PSI section-header offset of the high four section-length bits.
const PSI_SECTION_LENGTH_HIGH_OFFSET: usize = 1;
/// PSI section-header offset of the low eight section-length bits.
const PSI_SECTION_LENGTH_LOW_OFFSET: usize = 2;
/// Mask selecting the high four bits of a PSI section length.
const PSI_SECTION_LENGTH_HIGH_MASK: u8 = 0x0F;
/// Bytes in every PSI section's trailing CRC32.
const PSI_CRC_BYTES: usize = 4;
/// Fixed PAT bytes before four-byte program entries.
const PAT_HEADER_BYTES: usize = 8;
/// Bytes in one PAT program-number and PID entry.
const PAT_ENTRY_BYTES: usize = 4;
/// Bytes in the program-number field of a PAT entry.
const PAT_PROGRAM_BYTES: usize = 2;
/// PAT entry offset of the high PMT-PID bits.
const PAT_PID_HIGH_OFFSET: usize = 2;
/// PAT entry offset of the low PMT-PID bits.
const PAT_PID_LOW_OFFSET: usize = 3;
/// PAT program number reserved for network information rather than a PMT.
const PAT_NETWORK_PROGRAM: u16 = 0;
/// Minimum complete PMT size: fixed header plus CRC32.
const PMT_MIN_BYTES: usize = 16;
/// PMT offset of the high four `program_info_length` bits.
const PMT_PROGRAM_INFO_HIGH_OFFSET: usize = 10;
/// PMT offset of the low eight `program_info_length` bits.
const PMT_PROGRAM_INFO_LOW_OFFSET: usize = 11;
/// Fixed PMT bytes before program descriptors and elementary-stream entries.
const PMT_HEADER_BYTES: usize = 12;
/// Bytes in an elementary-stream entry before its descriptor loop.
const PMT_STREAM_ENTRY_BYTES: usize = 5;
/// Elementary-stream entry offset of `stream_type`.
const PMT_STREAM_TYPE_OFFSET: usize = 0;
/// Elementary-stream entry offset of the high four `ES_info_length` bits.
const PMT_DESCRIPTOR_LENGTH_HIGH_OFFSET: usize = 3;
/// Elementary-stream entry offset of the low eight `ES_info_length` bits.
const PMT_DESCRIPTOR_LENGTH_LOW_OFFSET: usize = 4;

/// MPEG-1 video stream type.
const STREAM_TYPE_MPEG1_VIDEO: u8 = 0x01;
/// MPEG-2/H.262 video stream type.
const STREAM_TYPE_MPEG2_VIDEO: u8 = 0x02;
/// AVC/H.264 video stream type.
const STREAM_TYPE_AVC: u8 = 0x1B;
/// HEVC/H.265 video stream type.
const STREAM_TYPE_HEVC: u8 = 0x24;
/// Common Blu-ray VC-1 video stream type.
const STREAM_TYPE_VC1: u8 = 0xEA;
/// Private PES stream type whose format is identified by descriptors.
const STREAM_TYPE_PRIVATE_PES: u8 = 0x06;
/// MPEG-1 audio stream type.
const STREAM_TYPE_MPEG1_AUDIO: u8 = 0x03;
/// MPEG-2 audio stream type.
const STREAM_TYPE_MPEG2_AUDIO: u8 = 0x04;
/// AAC with ADTS transport stream type.
const STREAM_TYPE_AAC_ADTS: u8 = 0x0F;
/// AAC with LATM transport stream type.
const STREAM_TYPE_AAC_LATM: u8 = 0x11;
/// Common ATSC/Blu-ray AC-3 audio stream type.
const STREAM_TYPE_AC3: u8 = 0x81;
/// Common Blu-ray DTS core audio stream type.
const STREAM_TYPE_DTS: u8 = 0x82;
/// Common Blu-ray DTS-HD High Resolution audio stream type.
const STREAM_TYPE_DTS_HD_HIGH_RESOLUTION: u8 = 0x85;
/// Common Blu-ray DTS-HD Master Audio stream type.
const STREAM_TYPE_DTS_HD_MASTER: u8 = 0x86;
/// Common ATSC/Blu-ray E-AC-3 audio stream type.
const STREAM_TYPE_EAC3: u8 = 0x87;
/// Blu-ray Presentation Graphics subtitle stream type.
const STREAM_TYPE_PGS: u8 = 0x90;
/// Blu-ray text subtitle stream type.
const STREAM_TYPE_TEXT_SUBTITLE: u8 = 0x92;

/// Bytes in a descriptor's tag-and-length prefix.
const DESCRIPTOR_HEADER_BYTES: usize = 2;
/// Descriptor-header offset of the tag.
const DESCRIPTOR_TAG_OFFSET: usize = 0;
/// Descriptor-header offset of the payload length.
const DESCRIPTOR_LENGTH_OFFSET: usize = 1;
/// MPEG registration descriptor tag.
const DESCRIPTOR_REGISTRATION: u8 = 0x05;
/// ISO 639 language descriptor tag.
const DESCRIPTOR_ISO_639_LANGUAGE: u8 = 0x0A;
/// DVB teletext descriptor tag.
const DESCRIPTOR_TELETEXT: u8 = 0x56;
/// DVB subtitling descriptor tag.
const DESCRIPTOR_SUBTITLING: u8 = 0x59;
/// DVB AC-3 descriptor tag.
const DESCRIPTOR_AC3: u8 = 0x6A;
/// DVB enhanced AC-3 descriptor tag.
const DESCRIPTOR_EAC3: u8 = 0x7A;
/// DVB DTS descriptor tag.
const DESCRIPTOR_DTS: u8 = 0x7B;
/// Bytes in an ISO 639-2 language code.
const ISO_639_LANGUAGE_BYTES: usize = 3;
/// Bytes in a registration descriptor's format identifier.
const REGISTRATION_IDENTIFIER_BYTES: usize = 4;

/// Physical packet stride and the location of the MPEG-TS sync byte within it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::probe) enum Layout {
    /// Plain 188-byte transport packets.
    Transport,
    /// 192-byte Blu-ray packets with a four-byte arrival timestamp.
    M2ts,
    /// 204-byte broadcast packets with trailing error-correction bytes.
    ErrorCorrection,
}

impl Layout {
    /// Returns the normalized media format represented by this packet layout.
    pub(in crate::probe) const fn media_format(self) -> MediaFormat {
        if matches!(self, Self::M2ts) {
            MediaFormat::M2ts
        } else {
            MediaFormat::Ts
        }
    }

    /// Returns the physical packet stride.
    const fn size(self) -> usize {
        match self {
            Self::Transport => TS_PACKET_BYTES,
            Self::M2ts => M2TS_PACKET_BYTES,
            Self::ErrorCorrection => FEC_TS_PACKET_BYTES,
        }
    }

    /// Returns the sync-byte offset within a physical packet.
    const fn sync_offset(self) -> usize {
        match self {
            Self::M2ts => M2TS_SYNC_OFFSET,
            Self::Transport | Self::ErrorCorrection => 0,
        }
    }
}

/// One bounded MPEG descriptor borrowed from a PMT descriptor loop.
#[derive(Clone, Copy)]
struct Descriptor<'a> {
    tag: u8,
    payload: &'a [u8],
}

/// Iterates tag-length-value descriptors and stops at malformed trailing data.
struct Descriptors<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Descriptors<'a> {
    const fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }
}

impl<'a> Iterator for Descriptors<'a> {
    type Item = Descriptor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len().saturating_sub(self.offset) < DESCRIPTOR_HEADER_BYTES {
            return None;
        }
        let tag = self.data[self.offset + DESCRIPTOR_TAG_OFFSET];
        let length = usize::from(self.data[self.offset + DESCRIPTOR_LENGTH_OFFSET]);
        let Some(end) = self
            .offset
            .checked_add(DESCRIPTOR_HEADER_BYTES)
            .and_then(|start| start.checked_add(length))
        else {
            self.offset = self.data.len();
            return None;
        };
        let Some(payload) = self.data.get(self.offset + DESCRIPTOR_HEADER_BYTES..end) else {
            self.offset = self.data.len();
            return None;
        };
        self.offset = end;
        Some(Descriptor { tag, payload })
    }
}

/// Probes a detected MPEG transport stream using its prepared packet layout.
pub(in crate::probe) fn probe(input: &mut ProbeInput, layout: Layout) -> io::Result<MediaInfo> {
    let file_len = input.len();
    let data = read_region(input.file(), 0, file_len.min(MAX_TS_SCAN_BYTES), file_len)?;
    // PID 0 and table_id 0x00 are reserved for the PAT. The selected program's PMT uses
    // table_id 0x02 on the PID announced by that PAT.
    let pat = collect_section(&data, layout, PAT_PID, PAT_TABLE_ID)?
        .ok_or_else(|| invalid("MPEG-TS PAT not found"))?;
    let pmt_pid = parse_pat(&pat).ok_or_else(|| invalid("MPEG-TS PAT has no program"))?;
    let pmt = collect_section(&data, layout, pmt_pid, PMT_TABLE_ID)?
        .ok_or_else(|| invalid("MPEG-TS PMT not found"))?;
    let (video_streams, audio_streams, subtitle_streams) = parse_pmt(&pmt)?;
    let mut media = MediaInfo::new(layout.media_format());
    media.video_streams = video_streams;
    media.audio_streams = audio_streams;
    media.subtitle_streams = subtitle_streams;
    Ok(media)
}

/// Detects the packet stride and sync-byte offset of an MPEG transport stream.
pub(in crate::probe) fn detect(data: &[u8]) -> Option<Layout> {
    [Layout::Transport, Layout::M2ts, Layout::ErrorCorrection]
        .into_iter()
        .find(|layout| {
            (0..SYNC_CHECK_PACKETS).all(|index| {
                data.get(layout.sync_offset() + index * layout.size()) == Some(&TS_SYNC_BYTE)
            })
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
    while packet_start + layout.size() <= data.len() {
        let packet = &data[packet_start..packet_start + layout.size()];
        let sync = layout.sync_offset();
        // Every MPEG-TS packet starts with the fixed sync byte 0x47.
        if packet[sync] != TS_SYNC_BYTE {
            return Err(invalid("MPEG-TS packet lost synchronization"));
        }
        // Header byte 1 carries payload_unit_start_indicator in bit 6 and the high five PID bits;
        // byte 2 carries the remaining eight PID bits.
        let pid = (u16::from(packet[sync + TS_PID_HIGH_OFFSET] & TS_PID_HIGH_MASK) << u8::BITS)
            | u16::from(packet[sync + TS_PID_LOW_OFFSET]);
        let unit_start = packet[sync + TS_PID_HIGH_OFFSET] & TS_PAYLOAD_UNIT_START != 0;
        // adaptation_field_control occupies header byte 3 bits 5..4. Bit zero of the extracted
        // value means payload is present; bit one means an adaptation field precedes it.
        let adaptation = (packet[sync + TS_CONTROL_OFFSET] >> TS_ADAPTATION_CONTROL_SHIFT)
            & TS_ADAPTATION_CONTROL_MASK;
        if pid == wanted_pid && adaptation & TS_PAYLOAD_PRESENT != 0 {
            let mut offset = sync + TS_HEADER_BYTES;
            if adaptation & TS_ADAPTATION_PRESENT != 0 {
                let length = usize::from(
                    *packet
                        .get(offset)
                        .ok_or_else(|| invalid("truncated adaptation field"))?,
                );
                offset = offset
                    .checked_add(length + ADAPTATION_LENGTH_BYTES)
                    .ok_or_else(|| invalid("adaptation offset overflow"))?;
            }
            if offset > packet.len() {
                return Err(invalid("adaptation field exceeds packet"));
            }
            if unit_start {
                // PSI payloads beginning a new section start with a pointer field whose value skips
                // any bytes before the section.
                let pointer = usize::from(
                    *packet
                        .get(offset)
                        .ok_or_else(|| invalid("truncated PSI pointer"))?,
                );
                offset = offset
                    .checked_add(pointer + PSI_POINTER_BYTES)
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
                } else if section.len() >= PSI_SECTION_HEADER_BYTES && expected.is_none() {
                    // PSI section_length is the low 12 bits of bytes 1..2 and counts everything
                    // after those first three bytes, including the trailing CRC32.
                    let length = ((usize::from(
                        section[PSI_SECTION_LENGTH_HIGH_OFFSET] & PSI_SECTION_LENGTH_HIGH_MASK,
                    )) << u8::BITS)
                        | usize::from(section[PSI_SECTION_LENGTH_LOW_OFFSET]);
                    expected = Some(PSI_SECTION_HEADER_BYTES + length);
                }
                if let Some(length) = expected
                    && section.len() >= length
                {
                    section.truncate(length);
                    return Ok(Some(section));
                }
            }
        }
        packet_start += layout.size();
    }
    Ok(None)
}

fn parse_pat(section: &[u8]) -> Option<u16> {
    // A PAT has an eight-byte header, four bytes per program mapping, and a four-byte CRC. Program
    // number zero denotes network information rather than a PMT, so the first nonzero program is
    // selected.
    let mut offset = PAT_HEADER_BYTES;
    let end = section.len().checked_sub(PSI_CRC_BYTES)?;
    while offset + PAT_ENTRY_BYTES <= end {
        let program = u16::from_be_bytes(
            section[offset..offset + PAT_PROGRAM_BYTES]
                .try_into()
                .ok()?,
        );
        let pid = (u16::from(section[offset + PAT_PID_HIGH_OFFSET] & TS_PID_HIGH_MASK) << u8::BITS)
            | u16::from(section[offset + PAT_PID_LOW_OFFSET]);
        if program != PAT_NETWORK_PROGRAM {
            return Some(pid);
        }
        offset += PAT_ENTRY_BYTES;
    }
    None
}

fn parse_pmt(
    section: &[u8],
) -> io::Result<(Vec<VideoStream>, Vec<AudioStream>, Vec<SubtitleStream>)> {
    if section.len() < PMT_MIN_BYTES {
        return Err(invalid("truncated MPEG-TS PMT"));
    }
    // PMT bytes 10..11 contain a 12-bit program_info_length. Each following elementary-stream entry
    // has a five-byte fixed header and a 12-bit ES_info_length for its descriptor loop. The final
    // four bytes are CRC32.
    let program_info_len =
        ((usize::from(section[PMT_PROGRAM_INFO_HIGH_OFFSET] & PSI_SECTION_LENGTH_HIGH_MASK))
            << u8::BITS)
            | usize::from(section[PMT_PROGRAM_INFO_LOW_OFFSET]);
    let mut offset = PMT_HEADER_BYTES
        .checked_add(program_info_len)
        .ok_or_else(|| invalid("PMT program descriptor overflow"))?;
    let end = section
        .len()
        .checked_sub(PSI_CRC_BYTES)
        .ok_or_else(|| invalid("truncated PMT checksum"))?;
    let mut video = Vec::new();
    let mut audio = Vec::new();
    let mut subtitles = Vec::new();
    while offset + PMT_STREAM_ENTRY_BYTES <= end {
        let stream_type = section[offset + PMT_STREAM_TYPE_OFFSET];
        let descriptor_len = ((usize::from(
            section[offset + PMT_DESCRIPTOR_LENGTH_HIGH_OFFSET] & PSI_SECTION_LENGTH_HIGH_MASK,
        )) << u8::BITS)
            | usize::from(section[offset + PMT_DESCRIPTOR_LENGTH_LOW_OFFSET]);
        let descriptor_start = offset + PMT_STREAM_ENTRY_BYTES;
        let descriptor_end = descriptor_start
            .checked_add(descriptor_len)
            .ok_or_else(|| invalid("PMT descriptor overflow"))?;
        let descriptors = section
            .get(descriptor_start..descriptor_end)
            .ok_or_else(|| invalid("PMT descriptor exceeds section"))?;
        let language = descriptor_language(descriptors);
        if let Some(codec) = video_stream_type(stream_type, descriptors) {
            video.push(VideoStream {
                info: StreamInfo {
                    language,
                    ..StreamInfo::default()
                },
                codec: Some(codec),
                ..VideoStream::default()
            });
        }
        if let Some(codec) = audio_stream_type(stream_type, descriptors) {
            audio.push(AudioStream {
                info: StreamInfo {
                    language,
                    ..StreamInfo::default()
                },
                codec: Some(codec),
                ..AudioStream::default()
            });
        }
        if let Some(codec) = subtitle_stream_type(stream_type, descriptors) {
            subtitles.push(SubtitleStream {
                info: StreamInfo {
                    language,
                    ..StreamInfo::default()
                },
                codec: Some(codec.to_owned()),
                ..SubtitleStream::default()
            });
        }
        offset = descriptor_end;
    }
    Ok((video, audio, subtitles))
}

fn video_stream_type(stream_type: u8, descriptors: &[u8]) -> Option<VideoCodec> {
    // Values through 0x24 are ISO/IEC 13818-1 assignments; 0xEA is a common Blu-ray private
    // assignment. Private PES type 0x06 may instead carry a registration descriptor naming HEVC.
    match stream_type {
        STREAM_TYPE_MPEG1_VIDEO | STREAM_TYPE_MPEG2_VIDEO => Some(VideoCodec::H262),
        STREAM_TYPE_AVC => Some(VideoCodec::H264),
        STREAM_TYPE_HEVC => Some(VideoCodec::H265),
        STREAM_TYPE_VC1 => Some(VideoCodec::Vc1),
        STREAM_TYPE_PRIVATE_PES if registration(descriptors) == Some(*b"HEVC") => {
            Some(VideoCodec::H265)
        }
        _ => None,
    }
}

fn audio_stream_type(stream_type: u8, descriptors: &[u8]) -> Option<AudioCodec> {
    // 0x03/0x04 are MPEG audio, 0x0F/0x11 are AAC (ADTS/LATM), and values 0x81..0x87 are common
    // ATSC/Blu-ray private assignments for AC-3, DTS, DTS variants, and E-AC-3. Type 0x06 relies on
    // descriptors.
    match stream_type {
        STREAM_TYPE_MPEG1_AUDIO | STREAM_TYPE_MPEG2_AUDIO => Some(AudioCodec::Mp3),
        STREAM_TYPE_AAC_ADTS | STREAM_TYPE_AAC_LATM => Some(AudioCodec::Aac),
        STREAM_TYPE_AC3 => Some(AudioCodec::DolbyDigital),
        STREAM_TYPE_DTS | STREAM_TYPE_DTS_HD_HIGH_RESOLUTION | STREAM_TYPE_DTS_HD_MASTER => {
            Some(AudioCodec::Dts)
        }
        STREAM_TYPE_EAC3 => Some(AudioCodec::DolbyDigitalPlus),
        STREAM_TYPE_PRIVATE_PES => descriptor_audio_codec(descriptors),
        _ => None,
    }
}

fn subtitle_stream_type(stream_type: u8, descriptors: &[u8]) -> Option<&'static str> {
    // 0x90 and 0x92 are Blu-ray private assignments. DVB subtitle and teletext streams use generic
    // private PES type 0x06 and are distinguished by descriptor tags 0x59 and 0x56 respectively.
    match stream_type {
        STREAM_TYPE_PGS => Some("pgs"),
        STREAM_TYPE_TEXT_SUBTITLE => Some("text"),
        STREAM_TYPE_PRIVATE_PES if has_descriptor(descriptors, DESCRIPTOR_SUBTITLING) => {
            Some("dvb_subtitle")
        }
        STREAM_TYPE_PRIVATE_PES if has_descriptor(descriptors, DESCRIPTOR_TELETEXT) => {
            Some("teletext")
        }
        _ => None,
    }
}

fn has_descriptor(descriptors: &[u8], wanted: u8) -> bool {
    Descriptors::new(descriptors).any(|descriptor| descriptor.tag == wanted)
}

fn descriptor_language(descriptors: &[u8]) -> Option<Language> {
    // ISO 639 language (0x0A), teletext (0x56), and subtitling (0x59) descriptors all begin their
    // payload/entry with a three-byte ISO 639-2 language code.
    for descriptor in Descriptors::new(descriptors) {
        if matches!(
            descriptor.tag,
            DESCRIPTOR_ISO_639_LANGUAGE | DESCRIPTOR_TELETEXT | DESCRIPTOR_SUBTITLING
        ) && descriptor.payload.len() >= ISO_639_LANGUAGE_BYTES
        {
            let language =
                std::str::from_utf8(&descriptor.payload[..ISO_639_LANGUAGE_BYTES]).ok()?;
            if language.bytes().all(|byte| byte.is_ascii_alphabetic()) {
                return Language::from_identifier(language);
            }
        }
    }
    None
}

fn descriptor_audio_codec(descriptors: &[u8]) -> Option<AudioCodec> {
    // Registration descriptor 0x05 carries a four-byte format identifier. DVB also assigns
    // dedicated descriptor tags 0x6A (AC-3), 0x7A (enhanced AC-3), and 0x7B (DTS).
    for descriptor in Descriptors::new(descriptors) {
        match descriptor.tag {
            DESCRIPTOR_REGISTRATION if descriptor.payload.starts_with(b"AC-3") => {
                return Some(AudioCodec::DolbyDigital);
            }
            DESCRIPTOR_REGISTRATION if descriptor.payload.starts_with(b"EAC3") => {
                return Some(AudioCodec::DolbyDigitalPlus);
            }
            DESCRIPTOR_REGISTRATION if descriptor.payload.starts_with(b"DTS") => {
                return Some(AudioCodec::Dts);
            }
            DESCRIPTOR_AC3 => return Some(AudioCodec::DolbyDigital),
            DESCRIPTOR_EAC3 => return Some(AudioCodec::DolbyDigitalPlus),
            DESCRIPTOR_DTS => return Some(AudioCodec::Dts),
            _ => {}
        }
    }
    None
}

fn registration(descriptors: &[u8]) -> Option<[u8; REGISTRATION_IDENTIFIER_BYTES]> {
    // Return the four-byte format_identifier from registration descriptor 0x05. Additional
    // identification bytes, when present, are not needed.
    for descriptor in Descriptors::new(descriptors) {
        if descriptor.tag == DESCRIPTOR_REGISTRATION
            && descriptor.payload.len() >= REGISTRATION_IDENTIFIER_BYTES
        {
            return descriptor.payload[..REGISTRATION_IDENTIFIER_BYTES]
                .try_into()
                .ok();
        }
    }
    None
}

crate::unit_tests!("mpeg_ts.test.rs");
