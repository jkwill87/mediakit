//! Parses Matroska and WebM metadata from their EBML element trees.
//!
//! EBML encodes every element as a variable-width element ID, a variable-width
//! payload size, and the payload itself. Matroska's top-level `Segment` can be
//! unknown-sized for streaming, so file-level traversal is seek-based while
//! bounded child payloads are traversed as slices. Only the EBML header,
//! `Info`, and `Tracks` metadata are read; `Cluster` media payloads are skipped.
//!
//! Element-ID constants retain the VINT marker bit and are written as the
//! big-endian hexadecimal IDs from the Matroska element registry.

use super::binary::{checked_end, invalid, read_region};
use super::{Probe, audio_layout, avc_profile, hevc_profile, video_resolution};
use crate::meta::fields::{AudioCodec, AudioProfile, Language, VideoCodec};
use crate::probe::{AudioStream, SubtitleStream, VideoStream};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::time::Duration;

/// EBML header master element (`0x1A45DFA3`).
const EBML: u64 = 0x1A45_DFA3;
/// EBML header `DocType` string (`0x4282`), normally `matroska` or `webm`.
const DOC_TYPE: u64 = 0x4282;
/// Matroska `Segment` master element (`0x18538067`).
const SEGMENT: u64 = 0x1853_8067;
/// Segment `Info` master element (`0x1549A966`).
const INFO: u64 = 0x1549_A966;
/// `TimestampScale` (`0x2AD7B1`), the nanoseconds represented by one Segment Tick.
const TIMECODE_SCALE: u64 = 0x002A_D7B1;
/// Segment `Duration` (`0x4489`), expressed as a floating-point count of Segment Ticks.
const DURATION: u64 = 0x4489;
/// Segment `Tracks` master element (`0x1654AE6B`).
const TRACKS: u64 = 0x1654_AE6B;
/// One `TrackEntry` master element (`0xAE`).
const TRACK_ENTRY: u64 = 0xAE;
/// `TrackType` (`0x83`): video 1, audio 2, or subtitle 17 for the types retained here.
const TRACK_TYPE: u64 = 0x83;
/// `FlagEnabled` (`0xB9`), whose schema default is true.
const FLAG_ENABLED: u64 = 0xB9;
/// `FlagDefault` (`0x88`), whose schema default is true.
const FLAG_DEFAULT: u64 = 0x88;
/// `DefaultDuration` (`0x23E383`), the nominal frame duration in nanoseconds.
const DEFAULT_DURATION: u64 = 0x0023_E383;
/// Track `CodecID` string (`0x86`).
const CODEC_ID: u64 = 0x86;
/// Codec initialization bytes in `CodecPrivate` (`0x63A2`).
const CODEC_PRIVATE: u64 = 0x63A2;
/// Legacy ISO 639-2 track `Language` string (`0x22B59C`).
const LANGUAGE: u64 = 0x0022_B59C;
/// Preferred BCP 47 `LanguageIETF` string (`0x22B59D`).
const LANGUAGE_IETF: u64 = 0x0022_B59D;
/// Track `Video` settings master element (`0xE0`).
const VIDEO: u64 = 0xE0;
/// Track `Audio` settings master element (`0xE1`).
const AUDIO: u64 = 0xE1;
/// Stored video `PixelWidth` (`0xB0`).
const PIXEL_WIDTH: u64 = 0xB0;
/// Stored video `PixelHeight` (`0xBA`).
const PIXEL_HEIGHT: u64 = 0xBA;
/// Video `FlagInterlaced` (`0x9A`): 0 unknown, 1 interlaced, 2 progressive.
const FLAG_INTERLACED: u64 = 0x9A;
/// Audio channel count (`0x9F`), whose schema default is one channel.
const CHANNELS: u64 = 0x9F;

/// A fully bounded EBML element borrowed from its parent's payload.
#[derive(Clone, Copy)]
struct Element<'a> {
    id: u64,
    payload: &'a [u8],
}

/// Iterates sibling EBML elements within a known-size parent payload.
struct Elements<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Elements<'a> {
    const fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }
}

impl<'a> Iterator for Elements<'a> {
    type Item = io::Result<Element<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.data.len() {
            return None;
        }
        let start = self.offset;
        // IDs and sizes share the VINT framing, but the ID retains its marker
        // bit while the size uses only the VINT data bits.
        let (id, id_len) = match read_vint(&self.data[start..], true) {
            Ok(Some(value)) => value,
            Ok(None) => return Some(Err(invalid("unknown EBML element ID"))),
            Err(error) => return Some(Err(error)),
        };
        let size_start = start + id_len;
        let (size, size_len) = match read_vint(&self.data[size_start..], false) {
            Ok(Some(value)) => value,
            Ok(None) => {
                // An all-ones size denotes an unknown-sized master element.
                // Within a bounded parent, the remaining bytes are its extent.
                let payload_start = size_start + 1;
                self.offset = self.data.len();
                return Some(Ok(Element {
                    id,
                    payload: &self.data[payload_start..],
                }));
            }
            Err(error) => return Some(Err(error)),
        };
        let payload_start = size_start + size_len;
        let size = match usize::try_from(size) {
            Ok(value) => value,
            Err(_) => return Some(Err(invalid("EBML element is too large"))),
        };
        let Some(end) = payload_start.checked_add(size) else {
            return Some(Err(invalid("EBML element offset overflow")));
        };
        if end > self.data.len() {
            self.offset = self.data.len();
            return Some(Err(invalid("EBML element exceeds parent")));
        }
        self.offset = end;
        Some(Ok(Element {
            id,
            payload: &self.data[payload_start..end],
        }))
    }
}

struct Track {
    kind: u64,
    enabled: bool,
    default: bool,
    codec_id: String,
    codec_private: Vec<u8>,
    language: Option<String>,
    language_ietf: Option<String>,
    default_duration: Option<u64>,
    width: u64,
    height: u64,
    interlaced: Option<bool>,
    channels: u64,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            kind: 0,
            enabled: true,
            default: true,
            codec_id: String::new(),
            codec_private: Vec::new(),
            language: None,
            language_ietf: None,
            default_duration: None,
            width: 0,
            height: 0,
            interlaced: None,
            channels: 0,
        }
    }
}

pub fn parse(file: &mut File, file_len: u64) -> io::Result<Probe> {
    file.seek(SeekFrom::Start(0))?;
    let ebml = read_file_element(file, file_len)?;
    if ebml.id != EBML {
        return Err(invalid("Matroska EBML header missing"));
    }
    let header = read_region(file, ebml.payload_offset, ebml.size, file_len)?;
    let mut doc_type = None;
    for element in Elements::new(&header) {
        let element = element?;
        if element.id == DOC_TYPE {
            doc_type = std::str::from_utf8(element.payload).ok();
        }
    }

    file.seek(SeekFrom::Start(ebml.end))?;
    let segment = read_file_element(file, file_len)?;
    if segment.id != SEGMENT {
        return Err(invalid("Matroska segment missing"));
    }
    // A streaming Matroska Segment may use EBML's unknown-size sentinel. At
    // the file level its effective bound is the physical end of the file.
    let segment_end = if segment.size_unknown {
        file_len
    } else {
        segment.end
    };
    // TimestampScale defaults to 1,000,000 ns, so one Segment Tick is 1 ms.
    let mut timecode_scale = 1_000_000_u64;
    let mut duration = None;
    let mut tracks = Vec::new();
    let mut offset = segment.payload_offset;
    while offset < segment_end {
        file.seek(SeekFrom::Start(offset))?;
        let element = read_file_element(file, segment_end)?;
        match element.id {
            INFO => {
                let data = read_region(file, element.payload_offset, element.size, segment_end)?;
                parse_info(&data, &mut timecode_scale, &mut duration)?;
            }
            TRACKS => {
                let data = read_region(file, element.payload_offset, element.size, segment_end)?;
                tracks = parse_tracks(&data)?;
            }
            _ => {}
        }
        if !tracks.is_empty() && duration.is_some() {
            break;
        }
        if element.end <= offset {
            return Err(invalid("Matroska element did not advance"));
        }
        offset = element.end;
    }

    let mut probe = Probe::new(if doc_type == Some("webm") {
        "webm"
    } else {
        "mkv"
    });
    if let Some(duration) = duration {
        // Info.Duration is measured in Segment Ticks, not seconds.
        let seconds = duration * timecode_scale as f64 / 1_000_000_000.0;
        if seconds.is_finite() && seconds >= 0.0 {
            probe.duration = Duration::try_from_secs_f64(seconds).ok();
        }
    }
    for track in &tracks {
        // Matroska TrackType values are registry assignments, not bit flags.
        match track.kind {
            1 => probe.video_streams.push(video_stream(track)),
            2 => probe.audio_streams.push(audio_stream(track)),
            17 => probe.subtitle_streams.push(subtitle_stream(track)),
            _ => {}
        }
    }
    Ok(probe)
}

fn audio_stream(track: &Track) -> AudioStream {
    AudioStream {
        is_enabled: track.enabled,
        is_default: track.default,
        language: track_language(track),
        codec: matroska_audio_codec(&track.codec_id),
        profile: matroska_audio_profile(&track.codec_id),
        layout: audio_layout(track.channels, None),
        bit_rate: None,
    }
}

fn video_stream(track: &Track) -> VideoStream {
    let codec = matroska_video_codec(&track.codec_id);
    // AVCDecoderConfigurationRecord stores profile_idc at byte 1. The HEVC
    // record stores general_profile_idc in byte 1's low five bits and
    // bitDepthLumaMinus8 in byte 17's low three bits.
    let profile = match codec.as_ref() {
        Some(VideoCodec::H264) => track
            .codec_private
            .get(1)
            .and_then(|value| avc_profile(*value)),
        Some(VideoCodec::H265) => track.codec_private.get(1).and_then(|value| {
            let profile = *value & 0x1F;
            let bit_depth = track.codec_private.get(17).map(|depth| 8 + (depth & 0x07));
            hevc_profile(profile, bit_depth)
        }),
        _ => None,
    };
    VideoStream {
        is_enabled: track.enabled,
        is_default: track.default,
        language: track_language(track),
        codec,
        profile,
        width: u32::try_from(track.width).ok().filter(|value| *value > 0),
        height: u32::try_from(track.height).ok().filter(|value| *value > 0),
        resolution: video_resolution(track.width, track.height, track.interlaced),
        frame_rate: track
            .default_duration
            .filter(|value| *value > 0)
            .map(|nanoseconds| (1_000_000_000.0 / nanoseconds as f64) as f32),
        dynamic_range: None,
    }
}

fn subtitle_stream(track: &Track) -> SubtitleStream {
    SubtitleStream {
        is_enabled: track.enabled,
        is_default: track.default,
        language: track_language(track),
        codec: (!track.codec_id.is_empty()).then(|| track.codec_id.clone()),
    }
}

fn track_language(track: &Track) -> Option<Language> {
    // LanguageIETF is the modern, more expressive field and takes precedence
    // over the legacy ISO 639-2 Language element when both are present.
    track
        .language_ietf
        .as_deref()
        .and_then(Language::from_identifier)
        .or_else(|| {
            track
                .language
                .as_deref()
                .and_then(Language::from_identifier)
        })
}

/// Seekable EBML element metadata whose payload has not yet been copied.
struct FileElement {
    id: u64,
    payload_offset: u64,
    size: u64,
    end: u64,
    size_unknown: bool,
}

fn read_file_element(file: &mut File, limit: u64) -> io::Result<FileElement> {
    let start = file.stream_position()?;
    let (id, _) = read_vint_from(file, true)?.ok_or_else(|| invalid("unknown EBML element ID"))?;
    let size_value = read_vint_from(file, false)?;
    let payload_offset = file.stream_position()?;
    let size_unknown = size_value.is_none();
    let size = size_value.map_or(limit - payload_offset, |(value, _)| value);
    let end = checked_end(payload_offset, size, limit)?;
    if payload_offset <= start {
        return Err(invalid("invalid EBML header"));
    }
    Ok(FileElement {
        id,
        payload_offset,
        size,
        end,
        size_unknown,
    })
}

fn read_vint_from<R: Read>(reader: &mut R, id: bool) -> io::Result<Option<(u64, usize)>> {
    let mut first = [0_u8; 1];
    reader.read_exact(&mut first)?;
    let length = vint_length(first[0])?;
    let mut bytes = [0_u8; 8];
    bytes[0] = first[0];
    reader.read_exact(&mut bytes[1..length])?;
    decode_vint(&bytes[..length], id)
}

fn read_vint(data: &[u8], id: bool) -> io::Result<Option<(u64, usize)>> {
    let first = *data
        .first()
        .ok_or_else(|| invalid("truncated EBML integer"))?;
    let length = vint_length(first)?;
    let bytes = data
        .get(..length)
        .ok_or_else(|| invalid("truncated EBML integer"))?;
    decode_vint(bytes, id)
}

fn vint_length(first: u8) -> io::Result<usize> {
    // A VINT begins with zero or more width bits followed by a one-bit marker;
    // the marker's position therefore gives the encoded byte length.
    let zeros = first.leading_zeros() as usize;
    if first == 0 || zeros >= 8 {
        return Err(invalid("invalid EBML integer marker"));
    }
    Ok(zeros + 1)
}

fn decode_vint(bytes: &[u8], id: bool) -> io::Result<Option<(u64, usize)>> {
    let marker = 0x80_u8 >> (bytes.len() - 1);
    // Element IDs include the marker as part of the registered identifier.
    // Element sizes clear it and interpret only the remaining data bits.
    let mut value = u64::from(if id {
        bytes[0]
    } else {
        bytes[0] & (marker - 1)
    });
    for byte in &bytes[1..] {
        value = value
            .checked_shl(8)
            .and_then(|value| value.checked_add(u64::from(*byte)))
            .ok_or_else(|| invalid("EBML integer overflow"))?;
    }
    if !id {
        // All VINT data bits set to one is the reserved unknown-size sentinel.
        let unknown =
            bytes[0] & (marker - 1) == marker - 1 && bytes[1..].iter().all(|byte| *byte == 0xFF);
        if unknown {
            return Ok(None);
        }
    }
    Ok(Some((value, bytes.len())))
}

fn parse_info(data: &[u8], scale: &mut u64, duration: &mut Option<f64>) -> io::Result<()> {
    for element in Elements::new(data) {
        let element = element?;
        match element.id {
            TIMECODE_SCALE => *scale = unsigned(element.payload)?,
            DURATION => *duration = Some(float(element.payload)?),
            _ => {}
        }
    }
    Ok(())
}

fn parse_tracks(data: &[u8]) -> io::Result<Vec<Track>> {
    let mut tracks = Vec::new();
    for element in Elements::new(data) {
        let element = element?;
        if element.id == TRACK_ENTRY {
            tracks.push(parse_track(element.payload)?);
        }
    }
    Ok(tracks)
}

fn parse_track(data: &[u8]) -> io::Result<Track> {
    let mut track = Track {
        enabled: true,
        default: true,
        ..Track::default()
    };
    for element in Elements::new(data) {
        let element = element?;
        match element.id {
            TRACK_TYPE => track.kind = unsigned(element.payload)?,
            FLAG_ENABLED => track.enabled = unsigned(element.payload)? != 0,
            FLAG_DEFAULT => track.default = unsigned(element.payload)? != 0,
            DEFAULT_DURATION => track.default_duration = Some(unsigned(element.payload)?),
            CODEC_ID => {
                track.codec_id = std::str::from_utf8(element.payload)
                    .map_err(|_| invalid("invalid Matroska codec ID"))?
                    .to_owned();
            }
            CODEC_PRIVATE => track.codec_private = element.payload.to_vec(),
            LANGUAGE => {
                track.language = Some(parse_text(element.payload, "invalid Matroska language")?);
            }
            LANGUAGE_IETF => {
                track.language_ietf = Some(parse_text(
                    element.payload,
                    "invalid Matroska IETF language",
                )?);
            }
            VIDEO => parse_video(element.payload, &mut track)?,
            AUDIO => parse_audio(element.payload, &mut track)?,
            _ => {}
        }
    }
    Ok(track)
}

fn parse_text(data: &[u8], error: &'static str) -> io::Result<String> {
    std::str::from_utf8(data)
        .map(str::to_owned)
        .map_err(|_| invalid(error))
}

fn parse_video(data: &[u8], track: &mut Track) -> io::Result<()> {
    for element in Elements::new(data) {
        let element = element?;
        match element.id {
            PIXEL_WIDTH => track.width = unsigned(element.payload)?,
            PIXEL_HEIGHT => track.height = unsigned(element.payload)?,
            FLAG_INTERLACED => {
                // Matroska uses an enumeration rather than a boolean here.
                track.interlaced = match unsigned(element.payload)? {
                    1 => Some(true),
                    2 => Some(false),
                    _ => None,
                };
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_audio(data: &[u8], track: &mut Track) -> io::Result<()> {
    for element in Elements::new(data) {
        let element = element?;
        if element.id == CHANNELS {
            track.channels = unsigned(element.payload)?;
        }
    }
    Ok(())
}

fn unsigned(data: &[u8]) -> io::Result<u64> {
    if data.is_empty() || data.len() > 8 {
        return Err(invalid("invalid EBML unsigned integer"));
    }
    Ok(data
        .iter()
        .fold(0_u64, |value, byte| (value << 8) | u64::from(*byte)))
}

fn float(data: &[u8]) -> io::Result<f64> {
    // EBML floats are big-endian IEEE-754 values and may be 32 or 64 bits.
    match data.len() {
        4 => Ok(f32::from_bits(u32::from_be_bytes(
            data.try_into().expect("four-byte slice"),
        )) as f64),
        8 => Ok(f64::from_bits(u64::from_be_bytes(
            data.try_into().expect("eight-byte slice"),
        ))),
        _ => Err(invalid("invalid EBML float")),
    }
}

fn matroska_audio_codec(id: &str) -> Option<AudioCodec> {
    if id.starts_with("A_AAC") {
        Some(AudioCodec::Aac)
    } else if id.starts_with("A_AC3") {
        Some(AudioCodec::DolbyDigital)
    } else if id.starts_with("A_EAC3") {
        Some(AudioCodec::DolbyDigitalPlus)
    } else if id.starts_with("A_DTS") {
        Some(AudioCodec::Dts)
    } else if id.starts_with("A_TRUEHD") {
        Some(AudioCodec::DolbyTrueHD)
    } else if id.starts_with("A_FLAC") {
        Some(AudioCodec::Flac)
    } else if id.starts_with("A_OPUS") {
        Some(AudioCodec::Opus)
    } else if id.starts_with("A_VORBIS") {
        Some(AudioCodec::Vorbis)
    } else if id.starts_with("A_MPEG/L3") {
        Some(AudioCodec::Mp3)
    } else if id.starts_with("A_PCM") {
        Some(AudioCodec::Pcm)
    } else if id.starts_with("A_ALAC") {
        Some(AudioCodec::Alac)
    } else {
        None
    }
}

fn matroska_audio_profile(id: &str) -> Option<AudioProfile> {
    if id.contains("LC/SBR") {
        Some(AudioProfile::HighEfficiency)
    } else if id.ends_with("/LC") {
        Some(AudioProfile::LowComplexity)
    } else {
        None
    }
}

fn matroska_video_codec(id: &str) -> Option<VideoCodec> {
    match id {
        "V_AV1" => Some(VideoCodec::Av1),
        "V_MPEG4/ISO/AVC" | "V_MPEG4/IS0/AVC" => Some(VideoCodec::H264),
        "V_MPEGH/ISO/HEVC" => Some(VideoCodec::H265),
        "V_MPEG1" | "V_MPEG2" => Some(VideoCodec::H262),
        "V_MPEG4/ISO/SP" | "V_MPEG4/ISO/ASP" | "V_MPEG4/ISO/AP" => Some(VideoCodec::Mpeg4Visual),
        "V_VP8" => Some(VideoCodec::Vp8),
        "V_VP9" => Some(VideoCodec::Vp9),
        value if value.starts_with("V_MS/VFW/FOURCC") && value.contains("WVC1") => {
            Some(VideoCodec::Vc1)
        }
        _ => None,
    }
}

crate::unit_tests!("matroska.test.rs");
