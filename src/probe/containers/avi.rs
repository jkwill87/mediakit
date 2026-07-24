//! Detects AVI containers and probes metadata from their RIFF header lists.
//!
//! AVI stores little-endian RIFF chunks identified by FourCC values. The `hdrl` list contains one
//! `avih` main header plus a `strl` list for each stream; the usually large `movi` list contains
//! media payloads and is not parsed. RIFF chunk sizes exclude the eight-byte chunk header and
//! odd-sized payloads are followed by one padding byte.

use super::binary::{fourcc, invalid, read_region, u32_le};
use super::windows_media::{
    BITMAP_INFO_MIN_BYTES, WAVE_FORMAT_MIN_BYTES, parse_bitmap_info, parse_wave_audio,
};
use super::{
    ProbeInput, ProbeResult, pixel_dimension, subtitle_codec, video_codec, video_resolution,
};
use crate::meta::fields::MediaFormat;
use crate::probe::{AudioTrack, SubtitleTrack, Track, TrackInfo, VideoTrack};
use std::io;
use std::time::Duration;

/// Maximum prefix inspected while looking for AVI header lists (16 MiB).
///
/// The metadata normally precedes `movi`; bounding the prefix prevents a malformed RIFF size from
/// making the probe copy media payloads into memory.
const MAX_AVI_HEADER_BYTES: u64 = 16 * 1024 * 1024;
/// Bytes in the RIFF identifier, size, and AVI form-type prefix.
const RIFF_AVI_HEADER_BYTES: usize = 12;
/// Offset of the AVI form type within the RIFF prefix.
const RIFF_FORM_TYPE_OFFSET: usize = 8;
/// Bytes in a RIFF chunk's FourCC and 32-bit payload size.
const RIFF_CHUNK_HEADER_BYTES: usize = 8;
/// Offset of a RIFF chunk's 32-bit payload size.
const RIFF_CHUNK_SIZE_OFFSET: usize = 4;
/// Bytes occupied by a RIFF FourCC or LIST type.
const RIFF_FOURCC_BYTES: usize = 4;
/// Low size bit indicating that a RIFF payload needs one alignment byte.
const RIFF_PADDING_MASK: usize = 1;
/// Minimum `AVIMAINHEADER` payload needed through `dwHeight`.
const AVI_MAIN_HEADER_MIN_BYTES: usize = 40;
/// `AVIMAINHEADER.dwMicroSecPerFrame` offset.
const AVI_MICROSECONDS_PER_FRAME_OFFSET: usize = 0;
/// `AVIMAINHEADER.dwTotalFrames` offset.
const AVI_TOTAL_FRAMES_OFFSET: usize = 16;
/// `AVIMAINHEADER.dwWidth` offset.
const AVI_WIDTH_OFFSET: usize = 32;
/// `AVIMAINHEADER.dwHeight` offset.
const AVI_HEIGHT_OFFSET: usize = 36;
/// Microseconds in one second, used to derive frame rate.
const MICROSECONDS_PER_SECOND: f32 = 1_000_000.0;
/// Minimum `AVISTREAMHEADER` payload needed through `dwLength`.
const AVI_STREAM_HEADER_MIN_BYTES: usize = 36;
/// `AVISTREAMHEADER.fccType` offset.
const AVI_STREAM_TYPE_OFFSET: usize = 0;
/// `AVISTREAMHEADER.fccHandler` offset.
const AVI_STREAM_HANDLER_OFFSET: usize = 4;
/// `AVISTREAMHEADER.dwFlags` offset.
const AVI_STREAM_FLAGS_OFFSET: usize = 8;
/// `AVISTREAMHEADER.dwScale` offset.
const AVI_STREAM_SCALE_OFFSET: usize = 20;
/// `AVISTREAMHEADER.dwRate` offset.
const AVI_STREAM_RATE_OFFSET: usize = 24;
/// `AVISTREAMHEADER.dwLength` offset.
const AVI_STREAM_LENGTH_OFFSET: usize = 32;
/// `AVISF_DISABLED` bit in `AVISTREAMHEADER.dwFlags`.
const AVI_STREAM_DISABLED_FLAG: u32 = 0x0000_0001;

/// Fields retained from an `AVISTREAMHEADER` (`strh`) and its adjacent stream format (`strf`)
/// chunk.
#[derive(Default)]
struct Stream {
    kind: Option<[u8; RIFF_FOURCC_BYTES]>,
    handler: Option<[u8; RIFF_FOURCC_BYTES]>,
    disabled: bool,
    scale: u32,
    rate: u32,
    length: u32,
    format: Vec<u8>,
}

/// Metadata accumulated while walking the AVI header lists.
#[derive(Default)]
struct Headers {
    microseconds_per_frame: u32,
    total_frames: u32,
    width: u64,
    height: u64,
    streams: Vec<Stream>,
}

/// A bounded RIFF chunk borrowed from its parent list.
#[derive(Clone, Copy)]
struct RiffChunk<'a> {
    kind: [u8; RIFF_FOURCC_BYTES],
    payload: &'a [u8],
}

/// Iterates word-aligned RIFF chunks while validating their declared sizes.
struct RiffChunks<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> RiffChunks<'a> {
    const fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }
}

impl<'a> Iterator for RiffChunks<'a> {
    type Item = io::Result<RiffChunk<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len().saturating_sub(self.offset) < RIFF_CHUNK_HEADER_BYTES {
            return None;
        }
        let kind = match fourcc(self.data, self.offset) {
            Ok(kind) => kind,
            Err(error) => return Some(Err(error)),
        };
        let size = match u32_le(self.data, self.offset + RIFF_CHUNK_SIZE_OFFSET)
            .and_then(|size| usize::try_from(size).map_err(|_| invalid("AVI chunk is too large")))
        {
            Ok(size) => size,
            Err(error) => return Some(Err(error)),
        };
        let payload_start = self.offset + RIFF_CHUNK_HEADER_BYTES;
        let Some(end) = payload_start.checked_add(size) else {
            self.offset = self.data.len();
            return Some(Err(invalid("AVI chunk offset overflow")));
        };
        let Some(payload) = self.data.get(payload_start..end) else {
            self.offset = self.data.len();
            // A bounded header prefix may stop at the usually large `movi` payload. Its declared
            // size is allowed to extend beyond the copied prefix because media data is not parsed.
            if kind == *b"LIST"
                && self
                    .data
                    .get(payload_start..payload_start.saturating_add(RIFF_FOURCC_BYTES))
                    == Some(b"movi")
            {
                return None;
            }
            return Some(Err(invalid("AVI chunk exceeds parent")));
        };
        self.offset = end.saturating_add(size & RIFF_PADDING_MASK);
        Some(Ok(RiffChunk { kind, payload }))
    }
}

/// Detects the AVI form type within a RIFF prefix.
pub(in crate::probe) fn matches(prefix: &[u8]) -> bool {
    prefix.len() >= RIFF_AVI_HEADER_BYTES
        && &prefix[..RIFF_FOURCC_BYTES] == b"RIFF"
        && &prefix[RIFF_FORM_TYPE_OFFSET..RIFF_AVI_HEADER_BYTES] == b"AVI "
}

/// Probes a detected AVI container.
pub(in crate::probe) fn probe(input: &mut ProbeInput) -> io::Result<ProbeResult> {
    let file_len = input.len();
    let data = read_region(
        input.file(),
        0,
        file_len.min(MAX_AVI_HEADER_BYTES),
        file_len,
    )?;
    if !matches(&data) {
        return Err(invalid("AVI RIFF header missing"));
    }
    let mut media = ProbeResult::new(MediaFormat::Avi);
    let mut headers = Headers::default();
    parse_chunks(&data[RIFF_AVI_HEADER_BYTES..], &mut headers)?;

    if headers.microseconds_per_frame > 0 && headers.total_frames > 0 {
        // AVIMAINHEADER gives overall timing as frame count multiplied by the nominal
        // microseconds between frames.
        media.duration = Some(Duration::from_micros(
            u64::from(headers.microseconds_per_frame)
                .saturating_mul(u64::from(headers.total_frames)),
        ));
    }
    for stream in &headers.streams {
        match stream.kind.as_ref() {
            Some(b"vids") => {
                let video = video_stream(
                    stream,
                    headers.width,
                    headers.height,
                    headers.microseconds_per_frame,
                )?;
                if media.duration.is_none()
                    && stream.scale > 0
                    && stream.rate > 0
                    && stream.length > 0
                {
                    // AVISTREAMHEADER defines rate / scale as samples per second, so length *
                    // scale / rate is the stream duration.
                    media.duration = Duration::try_from_secs_f64(
                        f64::from(stream.length) * f64::from(stream.scale) / f64::from(stream.rate),
                    )
                    .ok();
                }
                media.tracks.push(Track::Video(video));
            }
            Some(b"auds") => media.tracks.push(Track::Audio(audio_stream(stream)?)),
            Some(b"txts") => media.tracks.push(Track::Subtitle(SubtitleTrack {
                info: TrackInfo {
                    is_enabled: !stream.disabled,
                    ..TrackInfo::default()
                },
                codec: stream.handler.as_ref().and_then(subtitle_codec),
                ..SubtitleTrack::default()
            })),
            _ => {}
        }
    }
    Ok(media)
}

fn video_stream(
    stream: &Stream,
    default_width: u64,
    default_height: u64,
    microseconds_per_frame: u32,
) -> io::Result<VideoTrack> {
    // A video `strf` is a BITMAPINFOHEADER: dimensions begin at byte 4 and the compression FourCC
    // at byte 16. The stream handler is the fallback codec identifier when that structure is absent
    // or abbreviated.
    let bitmap = (stream.format.len() >= BITMAP_INFO_MIN_BYTES)
        .then(|| parse_bitmap_info(&stream.format))
        .transpose()?;
    let compression = bitmap
        .as_ref()
        .and_then(|bitmap| bitmap.compression)
        .or(stream.handler);
    let (width, height) = bitmap.map_or((default_width, default_height), |bitmap| {
        (bitmap.width, bitmap.height)
    });
    let frame_rate = if stream.scale > 0 && stream.rate > 0 {
        Some(stream.rate as f32 / stream.scale as f32)
    } else if microseconds_per_frame > 0 {
        Some(MICROSECONDS_PER_SECOND / microseconds_per_frame as f32)
    } else {
        None
    };
    Ok(VideoTrack {
        info: TrackInfo {
            is_enabled: !stream.disabled,
            ..TrackInfo::default()
        },
        codec: compression.as_ref().and_then(video_codec),
        profile: None,
        width: pixel_dimension(width),
        height: pixel_dimension(height),
        resolution: video_resolution(width, height, None),
        frame_rate,
        dynamic_range: None,
    })
}

fn audio_stream(stream: &Stream) -> io::Result<AudioTrack> {
    if stream.format.len() < WAVE_FORMAT_MIN_BYTES {
        return Ok(AudioTrack {
            info: TrackInfo {
                is_enabled: !stream.disabled,
                ..TrackInfo::default()
            },
            ..AudioTrack::default()
        });
    }
    parse_wave_audio(
        &stream.format,
        TrackInfo {
            is_enabled: !stream.disabled,
            ..TrackInfo::default()
        },
    )
}

fn parse_chunks(data: &[u8], headers: &mut Headers) -> io::Result<()> {
    for chunk in RiffChunks::new(data) {
        let chunk = chunk?;
        match &chunk.kind {
            b"avih" if chunk.payload.len() >= AVI_MAIN_HEADER_MIN_BYTES => {
                // AVIMAINHEADER fields used here are dwMicroSecPerFrame, dwTotalFrames, dwWidth,
                // and dwHeight respectively.
                headers.microseconds_per_frame =
                    u32_le(chunk.payload, AVI_MICROSECONDS_PER_FRAME_OFFSET)?;
                headers.total_frames = u32_le(chunk.payload, AVI_TOTAL_FRAMES_OFFSET)?;
                headers.width = u64::from(u32_le(chunk.payload, AVI_WIDTH_OFFSET)?);
                headers.height = u64::from(u32_le(chunk.payload, AVI_HEIGHT_OFFSET)?);
            }
            b"LIST"
                if chunk.payload.len() >= RIFF_FOURCC_BYTES
                    && &chunk.payload[..RIFF_FOURCC_BYTES] == b"hdrl" =>
            {
                parse_chunks(&chunk.payload[RIFF_FOURCC_BYTES..], headers)?;
            }
            b"LIST"
                if chunk.payload.len() >= RIFF_FOURCC_BYTES
                    && &chunk.payload[..RIFF_FOURCC_BYTES] == b"strl" =>
            {
                headers
                    .streams
                    .push(parse_stream(&chunk.payload[RIFF_FOURCC_BYTES..])?);
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_stream(data: &[u8]) -> io::Result<Stream> {
    let mut stream = Stream::default();
    for chunk in RiffChunks::new(data) {
        let chunk = chunk?;
        match &chunk.kind {
            b"strh" if chunk.payload.len() >= AVI_STREAM_HEADER_MIN_BYTES => {
                // AVISTREAMHEADER: fccType, fccHandler, dwFlags, dwScale, dwRate, and dwLength.
                // AVISF_DISABLED is flag bit zero.
                stream.kind = Some(fourcc(chunk.payload, AVI_STREAM_TYPE_OFFSET)?);
                stream.handler = Some(fourcc(chunk.payload, AVI_STREAM_HANDLER_OFFSET)?);
                stream.disabled =
                    u32_le(chunk.payload, AVI_STREAM_FLAGS_OFFSET)? & AVI_STREAM_DISABLED_FLAG != 0;
                stream.scale = u32_le(chunk.payload, AVI_STREAM_SCALE_OFFSET)?;
                stream.rate = u32_le(chunk.payload, AVI_STREAM_RATE_OFFSET)?;
                stream.length = u32_le(chunk.payload, AVI_STREAM_LENGTH_OFFSET)?;
            }
            b"strf" => stream.format = chunk.payload.to_vec(),
            _ => {}
        }
    }
    Ok(stream)
}

crate::unit_tests!("avi.test.rs");
