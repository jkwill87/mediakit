//! Parses ISO-BMFF and QuickTime container metadata.

use super::binary::{checked_end, fourcc, invalid, read_region, u16_be, u32_be, u64_be};
use super::{
    Probe, audio_codec, audio_layout, avc_profile, hevc_profile, video_codec, video_resolution,
};
use crate::meta::fields::{AudioProfile, VideoDynamicRange};
use crate::probe::{AudioStream, VideoStream};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::time::Duration;

const MAX_BOX_DEPTH: u8 = 12;

#[derive(Clone, Copy)]
struct BoxView<'a> {
    kind: [u8; 4],
    payload: &'a [u8],
}

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
        match track.handler.as_ref() {
            Some(b"soun") => probe.audio_streams.push(audio_stream(track)),
            Some(b"vide") => probe.video_streams.push(video_stream(track)),
            _ => {}
        }
    }
    Ok(probe)
}

fn audio_stream(track: &Track) -> AudioStream {
    AudioStream {
        is_enabled: track.enabled,
        is_default: false,
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
        codec: track.codec.as_ref().and_then(video_codec),
        profile: track.video_profile.clone(),
        width: u32::try_from(track.width).ok().filter(|value| *value > 0),
        height: u32::try_from(track.height).ok().filter(|value| *value > 0),
        resolution: video_resolution(track.width, track.height, None),
        frame_rate,
        dynamic_range: track.dynamic_range.clone(),
    }
}

fn parse_duration(data: &[u8]) -> io::Result<Option<f64>> {
    let version = *data
        .first()
        .ok_or_else(|| invalid("truncated movie header"))?;
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
    let (timescale_offset, duration_offset, duration_size) = if version == 1 {
        (20, 24, 8)
    } else {
        (12, 16, 4)
    };
    track.timescale = u64::from(u32_be(data, timescale_offset)?);
    track.duration = if duration_size == 8 {
        u64_be(data, duration_offset)?
    } else {
        u64::from(u32_be(data, duration_offset)?)
    };
    Ok(())
}

fn parse_sample_description(data: &[u8], track: &mut Track) -> io::Result<()> {
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
                track.video_profile = avc_profile(child.payload[1]);
            }
            b"hvcC" if child.payload.len() >= 18 => {
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
        return match config >> 3 {
            2 => Some(AudioProfile::LowComplexity),
            5 | 29 => Some(AudioProfile::HighEfficiency),
            _ => None,
        };
    }
    None
}

fn parse_time_to_sample(data: &[u8], track: &mut Track) -> io::Result<()> {
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
    let value = (bytes as u128)
        .checked_mul(8)?
        .checked_mul(u128::from(track.timescale))?
        / u128::from(track.duration);
    u32::try_from(value).ok()
}

crate::unit_tests!("mp4.test.rs");
