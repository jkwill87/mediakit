//! Parses ISO Base Media File Format and QuickTime metadata.
//!
//! Both formats are trees of boxes (historically called atoms in QuickTime).
//! Each box starts with a big-endian size and a four-byte type. The parser
//! locates the top-level file-type (`ftyp`) and movie (`moov`) boxes, then walks
//! only the movie headers, track headers, media descriptions, and sample tables
//! needed for metadata. Media-data (`mdat`) boxes are never loaded.
//!
//! A number of boxes are *FullBoxes*: their payload starts with one version
//! byte and three flag bytes. Version 1 commonly widens version 0's 32-bit time
//! fields to 64 bits, which accounts for several explicit offsets below.

use super::binary::{checked_end, fourcc, invalid, read_region, u16_be, u32_be, u64_be};
use super::{
    Probe, audio_codec, audio_layout, avc_profile, fourcc_string, hevc_profile, video_codec,
    video_resolution,
};
use crate::meta::fields::{AudioProfile, Language, VideoDynamicRange};
use crate::probe::{AudioStream, SubtitleStream, VideoStream};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::time::Duration;

/// Maximum recursive descent through nested movie/media/sample-table boxes.
///
/// Real files use only a handful of levels; the limit rejects adversarially
/// deep trees before recursive parsing can exhaust the stack.
const MAX_BOX_DEPTH: u8 = 12;

/// A box whose payload is borrowed from an already bounded parent.
#[derive(Clone, Copy)]
struct BoxView<'a> {
    kind: [u8; 4],
    payload: &'a [u8],
}

/// Iterates sibling ISO-BMFF boxes within a known-size parent payload.
struct Boxes<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Boxes<'a> {
    const fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }
}

impl<'a> Iterator for Boxes<'a> {
    type Item = io::Result<BoxView<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset == self.data.len() {
            return None;
        }
        if self.data.len().saturating_sub(self.offset) < 8 {
            self.offset = self.data.len();
            return Some(Err(invalid("truncated ISO-BMFF box")));
        }
        let start = self.offset;
        let size32 = match u32_be(self.data, start) {
            Ok(value) => value,
            Err(error) => return Some(Err(error)),
        };
        let kind = match fourcc(self.data, start + 4) {
            Ok(value) => value,
            Err(error) => return Some(Err(error)),
        };
        // A size of 1 selects the 16-byte header with a 64-bit `largesize`; a
        // size of 0 extends the box through the end of its parent. All other
        // sizes include the ordinary eight-byte header.
        let (header, size) = if size32 == 1 {
            if self.data.len().saturating_sub(start) < 16 {
                self.offset = self.data.len();
                return Some(Err(invalid("truncated extended ISO-BMFF box")));
            }
            match u64_be(self.data, start + 8).and_then(|size| {
                usize::try_from(size).map_err(|_| invalid("ISO-BMFF box is too large"))
            }) {
                Ok(size) => (16, size),
                Err(error) => return Some(Err(error)),
            }
        } else if size32 == 0 {
            (8, self.data.len() - start)
        } else {
            (8, size32 as usize)
        };
        if size < header
            || start
                .checked_add(size)
                .is_none_or(|end| end > self.data.len())
        {
            self.offset = self.data.len();
            return Some(Err(invalid("ISO-BMFF box exceeds parent")));
        }
        let end = start + size;
        self.offset = end;
        Some(Ok(BoxView {
            kind,
            payload: &self.data[start + header..end],
        }))
    }
}

#[derive(Default)]
struct Track {
    enabled: bool,
    handler: Option<[u8; 4]>,
    language: Option<Language>,
    timescale: u64,
    duration: u64,
    codec: Option<[u8; 4]>,
    width: u64,
    height: u64,
    channels: u64,
    average_sample_delta: Option<f64>,
    sample_bytes: Option<u64>,
    audio_profile: Option<AudioProfile>,
    video_profile: Option<crate::meta::fields::VideoProfile>,
    dynamic_range: Option<VideoDynamicRange>,
}

pub fn parse(file: &mut File, file_len: u64) -> io::Result<Probe> {
    let mut offset = 0_u64;
    let mut ftyp = None;
    let mut moov = None;
    while offset < file_len {
        file.seek(SeekFrom::Start(offset))?;
        let mut header = [0_u8; 16];
        file.read_exact(&mut header[..8])?;
        let size32 = u32::from_be_bytes(header[..4].try_into().expect("four-byte slice"));
        let kind: [u8; 4] = header[4..8].try_into().expect("four-byte slice");
        // Apply the same ordinary, extended-size, and to-end size rules used
        // by the in-memory box iterator.
        let (header_size, size) = if size32 == 1 {
            file.read_exact(&mut header[8..16])?;
            (
                16,
                u64::from_be_bytes(header[8..16].try_into().expect("eight-byte slice")),
            )
        } else if size32 == 0 {
            (8, file_len - offset)
        } else {
            (8, u64::from(size32))
        };
        if size < header_size {
            return Err(invalid("invalid ISO-BMFF top-level box size"));
        }
        checked_end(offset, size, file_len)?;
        match &kind {
            b"ftyp" => ftyp = Some((offset + header_size, size - header_size)),
            b"moov" => moov = Some((offset + header_size, size - header_size)),
            _ => {}
        }
        if size == 0 {
            break;
        }
        offset += size;
    }
    let (moov_offset, moov_size) = moov.ok_or_else(|| invalid("ISO-BMFF movie box not found"))?;
    let moov = read_region(file, moov_offset, moov_size, file_len)?;
    // `ftyp` is a short list of four-byte brands. Only a small bounded prefix
    // is needed to recognize QuickTime's `qt  ` brand.
    let ftyp = ftyp
        .map(|(offset, size)| read_region(file, offset, size.min(4096), file_len))
        .transpose()?;
    let container = if ftyp.as_deref().is_some_and(is_quicktime_brand) {
        "mov"
    } else {
        "mp4"
    };
    parse_movie(&moov, container)
}

fn is_quicktime_brand(ftyp: &[u8]) -> bool {
    // The payload starts with major_brand and minor_version, followed by zero
    // or more compatible brands. Treating every four-byte word as a candidate
    // safely includes the major brand and ignores the numeric minor version.
    ftyp.chunks_exact(4).any(|brand| brand == b"qt  ")
}

fn parse_movie(data: &[u8], container: &'static str) -> io::Result<Probe> {
    let mut probe = Probe::new(container);
    let mut movie_duration = None;
    let mut tracks = Vec::new();
    for child in Boxes::new(data) {
        let child = child?;
        match &child.kind {
            b"mvhd" => movie_duration = parse_duration(child.payload)?,
            b"trak" => tracks.push(parse_track(child.payload, 1)?),
            _ => {}
        }
    }
    probe.duration = movie_duration.and_then(|seconds| Duration::try_from_secs_f64(seconds).ok());

    for track in &tracks {
        // Handler types classify the media independently of its sample-entry
        // codec. The four subtitle values cover closed captions, subtitles,
        // MPEG-4 subtitle systems, and QuickTime text tracks respectively.
        match track.handler.as_ref() {
            Some(b"soun") => probe.audio_streams.push(audio_stream(track)),
            Some(b"vide") => probe.video_streams.push(video_stream(track)),
            Some(b"clcp" | b"sbtl" | b"subt" | b"text") => {
                probe.subtitle_streams.push(subtitle_stream(track));
            }
            _ => {}
        }
    }
    Ok(probe)
}

fn audio_stream(track: &Track) -> AudioStream {
    AudioStream {
        is_enabled: track.enabled,
        is_default: false,
        language: track.language,
        codec: track.codec.as_ref().and_then(audio_codec),
        profile: track.audio_profile.clone().or(match track.codec.as_ref() {
            Some(b"dtsh") => Some(AudioProfile::HighResolutionAudio),
            Some(b"dtsl") => Some(AudioProfile::MasterAudio),
            _ => None,
        }),
        layout: audio_layout(track.channels, None),
        bit_rate: audio_bit_rate(track),
    }
}

fn video_stream(track: &Track) -> VideoStream {
    let frame_rate = match (track.average_sample_delta, track.timescale) {
        (Some(delta), timescale) if delta > 0.0 && timescale > 0 => {
            Some((timescale as f64 / delta) as f32)
        }
        _ => None,
    };
    VideoStream {
        is_enabled: track.enabled,
        is_default: false,
        language: track.language,
        codec: track.codec.as_ref().and_then(video_codec),
        profile: track.video_profile.clone(),
        width: u32::try_from(track.width).ok().filter(|value| *value > 0),
        height: u32::try_from(track.height).ok().filter(|value| *value > 0),
        resolution: video_resolution(track.width, track.height, None),
        frame_rate,
        dynamic_range: track.dynamic_range.clone(),
    }
}

fn subtitle_stream(track: &Track) -> SubtitleStream {
    SubtitleStream {
        is_enabled: track.enabled,
        is_default: false,
        language: track.language,
        codec: track.codec.as_ref().and_then(fourcc_string),
    }
}

fn parse_duration(data: &[u8]) -> io::Result<Option<f64>> {
    let version = *data
        .first()
        .ok_or_else(|| invalid("truncated movie header"))?;
    // `mvhd` version 1 has 64-bit creation/modification times and duration;
    // version 0 uses 32-bit fields. The timescale remains 32-bit in both.
    let (timescale_offset, duration_offset, duration_size) = if version == 1 {
        (20, 24, 8)
    } else {
        (12, 16, 4)
    };
    let timescale = u64::from(u32_be(data, timescale_offset)?);
    let duration = if duration_size == 8 {
        u64_be(data, duration_offset)?
    } else {
        u64::from(u32_be(data, duration_offset)?)
    };
    // All-ones is the specified unknown-duration sentinel at either width.
    Ok(
        (timescale > 0 && duration != u64::MAX && duration != u64::from(u32::MAX))
            .then_some(duration as f64 / timescale as f64),
    )
}

fn parse_track(data: &[u8], depth: u8) -> io::Result<Track> {
    if depth > MAX_BOX_DEPTH {
        return Err(invalid("ISO-BMFF nesting exceeds limit"));
    }
    let mut track = Track::default();
    for child in Boxes::new(data) {
        let child = child?;
        match &child.kind {
            b"tkhd" => parse_track_header(child.payload, &mut track)?,
            b"mdia" => parse_media(child.payload, &mut track, depth + 1)?,
            _ => {}
        }
    }
    Ok(track)
}

fn parse_track_header(data: &[u8], track: &mut Track) -> io::Result<()> {
    // `tkhd` is a FullBox: enabled is flag bit 0, and the final width/height
    // fields are unsigned 16.16 fixed-point values regardless of box version.
    let flags = u32_be(data, 0)? & 0x00FF_FFFF;
    track.enabled = flags & 1 != 0;
    if data.len() >= 8 {
        track.width = u64::from(u32_be(data, data.len() - 8)?) >> 16;
        track.height = u64::from(u32_be(data, data.len() - 4)?) >> 16;
    }
    Ok(())
}

fn parse_media(data: &[u8], track: &mut Track, depth: u8) -> io::Result<()> {
    if depth > MAX_BOX_DEPTH {
        return Err(invalid("ISO-BMFF nesting exceeds limit"));
    }
    for child in Boxes::new(data) {
        let child = child?;
        match &child.kind {
            b"mdhd" => parse_media_header(child.payload, track)?,
            // `hdlr` stores handler_type after the four-byte FullBox prefix
            // and a four-byte pre_defined field.
            b"hdlr" => track.handler = Some(fourcc(child.payload, 8)?),
            b"minf" => parse_children(child.payload, track, depth + 1)?,
            _ => {}
        }
    }
    Ok(())
}

fn parse_children(data: &[u8], track: &mut Track, depth: u8) -> io::Result<()> {
    if depth > MAX_BOX_DEPTH {
        return Err(invalid("ISO-BMFF nesting exceeds limit"));
    }
    for child in Boxes::new(data) {
        let child = child?;
        match &child.kind {
            b"stbl" => parse_children(child.payload, track, depth + 1)?,
            b"stsd" => parse_sample_description(child.payload, track)?,
            b"stts" => parse_time_to_sample(child.payload, track)?,
            b"stsz" => parse_sample_sizes(child.payload, track)?,
            _ => {}
        }
    }
    Ok(())
}

fn parse_media_header(data: &[u8], track: &mut Track) -> io::Result<()> {
    let version = *data
        .first()
        .ok_or_else(|| invalid("truncated media header"))?;
    // As in `mvhd`, version 1 widens creation/modification times and duration
    // while timescale and the packed language field retain their widths.
    let (timescale_offset, duration_offset, duration_size, language_offset) = if version == 1 {
        (20, 24, 8, 32)
    } else {
        (12, 16, 4, 20)
    };
    track.timescale = u64::from(u32_be(data, timescale_offset)?);
    track.duration = if duration_size == 8 {
        u64_be(data, duration_offset)?
    } else {
        u64::from(u32_be(data, duration_offset)?)
    };
    track.language = data
        .get(language_offset..language_offset + 2)
        .map(|_| u16_be(data, language_offset))
        .transpose()?
        .and_then(mp4_language);
    Ok(())
}

fn mp4_language(value: u16) -> Option<Language> {
    // ISO language codes pack three lowercase letters into 15 bits. Each
    // 5-bit value is the ASCII letter minus 0x60 (`a` => 1, `z` => 26); zero
    // and 27..31 are not letters. Legacy QuickTime numeric language codes do
    // not use this packing and therefore intentionally fail this decoder.
    let mut identifier = String::with_capacity(3);
    for shift in [10, 5, 0] {
        let letter = u8::try_from((value >> shift) & 0x1F).ok()?;
        if !(1..=26).contains(&letter) {
            return None;
        }
        identifier.push(char::from(b'a' + letter - 1));
    }
    Language::from_identifier(&identifier)
}

fn parse_sample_description(data: &[u8], track: &mut Track) -> io::Result<()> {
    // `stsd` begins with a four-byte FullBox prefix and a four-byte entry
    // count. The first sample entry's type is the codec identifier retained by
    // this metadata-only parser.
    let entries = data
        .get(8..)
        .ok_or_else(|| invalid("truncated sample description"))?;
    let entry = Boxes::new(entries)
        .next()
        .transpose()?
        .ok_or_else(|| invalid("empty sample description"))?;
    track.codec = Some(entry.kind);
    match track.handler.as_ref() {
        Some(b"vide") => parse_video_entry(entry, track),
        Some(b"soun") => parse_audio_entry(entry, track),
        _ => Ok(()),
    }
}

fn parse_video_entry(entry: BoxView<'_>, track: &mut Track) -> io::Result<()> {
    // VisualSampleEntry has a 78-byte fixed payload before codec-specific child
    // boxes. Width and height are 16-bit integers at payload bytes 24 and 26.
    if entry.payload.len() < 78 {
        return Err(invalid("truncated video sample entry"));
    }
    track.width = u64::from(u16_be(entry.payload, 24)?);
    track.height = u64::from(u16_be(entry.payload, 26)?);
    if matches!(&entry.kind, b"dvhe" | b"dvh1") {
        track.dynamic_range = Some(VideoDynamicRange::DolbyVision);
    }
    for child in Boxes::new(&entry.payload[78..]) {
        let child = child?;
        match &child.kind {
            b"avcC" if child.payload.len() >= 2 => {
                // AVCDecoderConfigurationRecord byte 1 is AVCProfileIndication
                // (`profile_idc`).
                track.video_profile = avc_profile(child.payload[1]);
            }
            b"hvcC" if child.payload.len() >= 18 => {
                // HEVCDecoderConfigurationRecord stores general_profile_idc
                // in byte 1's low five bits and bitDepthLumaMinus8 in byte 17.
                let profile = child.payload[1] & 0x1F;
                let bit_depth = child.payload.get(17).map(|value| 8 + (value & 0x07));
                track.video_profile = hevc_profile(profile, bit_depth);
            }
            b"dvcC" | b"dvvC" => {
                track.dynamic_range = Some(VideoDynamicRange::DolbyVision);
            }
            b"colr"
                if child.payload.starts_with(b"nclx")
                    && child.payload.len() >= 8
                    // `nclx` transfer_characteristics 16 is SMPTE ST 2084
                    // (PQ), the transfer function used by HDR10.
                    && u16_be(child.payload, 6)? == 16 =>
            {
                track.dynamic_range = Some(VideoDynamicRange::HDR10);
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_audio_entry(entry: BoxView<'_>, track: &mut Track) -> io::Result<()> {
    // AudioSampleEntry version 0 has a 28-byte fixed payload. QuickTime
    // versions 1 and 2 append 16 and 36 bytes respectively before child boxes;
    // version 2 also carries a 32-bit channel count at payload byte 40.
    if entry.payload.len() < 28 {
        return Err(invalid("truncated audio sample entry"));
    }
    let version = u16_be(entry.payload, 8)?;
    track.channels = u64::from(if version == 2 {
        u32_be(entry.payload, 40)?
    } else {
        u32::from(u16_be(entry.payload, 16)?)
    });
    let child_offset = match version {
        1 => 44,
        2 => 64,
        _ => 28,
    };
    if child_offset <= entry.payload.len() {
        for child in Boxes::new(&entry.payload[child_offset..]) {
            let child = child?;
            if child.kind == *b"esds" {
                track.audio_profile = parse_aac_profile(child.payload);
            }
        }
    }
    Ok(())
}

fn parse_aac_profile(data: &[u8]) -> Option<AudioProfile> {
    // DecoderSpecificInfo is nested inside the ES and DecoderConfig descriptors.
    // Scan the already-bounded `esds` payload for a well-formed tag 0x05 rather
    // than duplicating the complete MPEG-4 descriptor grammar.
    for tag_offset in 4..data.len().saturating_sub(1) {
        if data[tag_offset] != 0x05 {
            continue;
        }
        let mut offset = tag_offset + 1;
        let mut size = 0_usize;
        let mut complete = false;
        for _ in 0..4 {
            let Some(byte) = data.get(offset).copied() else {
                break;
            };
            offset += 1;
            size = size.checked_shl(7)?.checked_add(usize::from(byte & 0x7F))?;
            if byte & 0x80 == 0 {
                complete = true;
                break;
            }
        }
        if !complete {
            continue;
        }
        let Some(payload) = data.get(offset..offset.checked_add(size)?) else {
            continue;
        };
        let Some(config) = payload.first() else {
            continue;
        };
        // The first five AudioSpecificConfig bits are audioObjectType:
        // 2 = AAC LC, 5 = SBR, and 29 = PS (the latter two are HE-AAC).
        return match config >> 3 {
            2 => Some(AudioProfile::LowComplexity),
            5 | 29 => Some(AudioProfile::HighEfficiency),
            _ => None,
        };
    }
    None
}

fn parse_time_to_sample(data: &[u8], track: &mut Track) -> io::Result<()> {
    // `stts` stores run-length pairs of (sample_count, sample_delta) after its
    // four-byte FullBox prefix and four-byte entry count. The weighted mean
    // delta later converts the media timescale into an average frame rate.
    let count = usize::try_from(u32_be(data, 4)?)
        .map_err(|_| invalid("sample timing count is too large"))?;
    let mut samples = 0_u64;
    let mut duration = 0_u64;
    for index in 0..count {
        let offset = 8 + index * 8;
        let sample_count = u64::from(u32_be(data, offset)?);
        let delta = u64::from(u32_be(data, offset + 4)?);
        samples = samples
            .checked_add(sample_count)
            .ok_or_else(|| invalid("sample count overflow"))?;
        duration = duration
            .checked_add(
                sample_count
                    .checked_mul(delta)
                    .ok_or_else(|| invalid("sample duration overflow"))?,
            )
            .ok_or_else(|| invalid("sample duration overflow"))?;
    }
    if samples > 0 {
        track.average_sample_delta = Some(duration as f64 / samples as f64);
    }
    Ok(())
}

fn parse_sample_sizes(data: &[u8], track: &mut Track) -> io::Result<()> {
    // `stsz` supplies one default byte size for every sample, or zero followed
    // by a 32-bit size for each sample. Summing sizes avoids reading `mdat`.
    let default_size = u64::from(u32_be(data, 4)?);
    let count =
        usize::try_from(u32_be(data, 8)?).map_err(|_| invalid("sample size count is too large"))?;
    track.sample_bytes = if default_size > 0 {
        default_size.checked_mul(count as u64)
    } else {
        let mut total = 0_u64;
        for index in 0..count {
            total = total
                .checked_add(u64::from(u32_be(data, 12 + index * 4)?))
                .ok_or_else(|| invalid("sample size overflow"))?;
        }
        Some(total)
    };
    Ok(())
}

fn audio_bit_rate(track: &Track) -> Option<u32> {
    let bytes = track.sample_bytes?;
    if track.timescale == 0 || track.duration == 0 {
        return None;
    }
    // Duration is in media-timescale ticks, so bits * timescale / duration is
    // the average number of bits per second.
    let value = (bytes as u128)
        .checked_mul(8)?
        .checked_mul(u128::from(track.timescale))?
        / u128::from(track.duration);
    u32::try_from(value).ok()
}

crate::unit_tests!("mp4.test.rs");
