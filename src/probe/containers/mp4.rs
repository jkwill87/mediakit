//! Detects ISO-BMFF containers and probes their movie metadata.
//!
//! Both formats are trees of boxes (historically called atoms in QuickTime). Each box starts with a
//! big-endian size and a four-byte type. The probe locates the top-level file-type (`ftyp`) and
//! movie (`moov`) boxes, then walks only the movie headers, track headers, media descriptions, and
//! sample tables needed for metadata. Media-data (`mdat`) boxes are never loaded.
//!
//! A number of boxes are *FullBoxes*: their payload starts with one version byte and three flag
//! bytes. Version 1 commonly widens version 0's 32-bit time fields to 64 bits, which accounts for
//! several explicit offsets below.

use super::binary::{checked_end, fourcc, invalid, read_region, u16_be, u32_be, u64_be};
use super::{
    ProbeInput, ProbeResult, audio_codec, audio_layout, avc_profile, hevc_profile, pixel_dimension,
    subtitle_codec, video_codec, video_resolution,
};
use crate::meta::fields::{AudioProfile, Language, MediaFormat, VideoDynamicRange};
use crate::probe::{
    AudioTrack, ProbeError, SubtitleTrack, Track as MediaTrack, TrackInfo, VideoTrack,
};
use std::io::{self, Read, Seek, SeekFrom};
use std::time::Duration;

/// Maximum recursive descent through nested movie/media/sample-table boxes.
///
/// Real files use only a handful of levels; the limit rejects adversarially deep trees before
/// recursive parsing can exhaust the stack.
const MAX_BOX_DEPTH: u8 = 12;
/// Bytes in a box's 32-bit size field.
const BOX_SIZE32_BYTES: usize = 4;
/// Bytes in a box type or compatible brand.
const BOX_TYPE_BYTES: usize = 4;
/// Bytes in an ordinary 32-bit-size box header.
const BOX_HEADER_BYTES: usize = 8;
/// Bytes in a box header carrying a 64-bit `largesize`.
const EXTENDED_BOX_HEADER_BYTES: usize = 16;
/// Offset of the four-byte box type.
const BOX_TYPE_OFFSET: usize = 4;
/// Offset of `largesize` in an extended box header.
const BOX_LARGE_SIZE_OFFSET: usize = 8;
/// 32-bit box-size sentinel selecting the 64-bit `largesize` field.
const BOX_EXTENDED_SIZE: u32 = 1;
/// 32-bit box-size sentinel extending a box to the end of its parent.
const BOX_TO_END_SIZE: u32 = 0;
/// Maximum `ftyp` payload prefix needed for brand detection.
const MAX_FILE_TYPE_BYTES: u64 = 4_096;
/// Initial recursion depth assigned to a top-level track box.
const INITIAL_BOX_DEPTH: u8 = 1;
/// Depth added when descending into one nested box level.
const BOX_DEPTH_INCREMENT: u8 = 1;
/// FullBox version that uses widened 64-bit time fields.
const FULL_BOX_VERSION_1: u8 = 1;
/// Version 0 `mvhd` timescale offset.
const MOVIE_HEADER_V0_TIMESCALE_OFFSET: usize = 12;
/// Version 0 `mvhd` duration offset.
const MOVIE_HEADER_V0_DURATION_OFFSET: usize = 16;
/// Version 1 `mvhd` timescale offset.
const MOVIE_HEADER_V1_TIMESCALE_OFFSET: usize = 20;
/// Version 1 `mvhd` duration offset.
const MOVIE_HEADER_V1_DURATION_OFFSET: usize = 24;
/// Encoded width of a version 0 duration.
const DURATION32_BYTES: usize = 4;
/// Encoded width of a version 1 duration.
const DURATION64_BYTES: usize = 8;
/// Mask selecting the 24 flag bits from a FullBox version-and-flags word.
const FULL_BOX_FLAGS_MASK: u32 = 0x00FF_FFFF;
/// `tkhd` flag indicating that a track is enabled.
const TRACK_ENABLED_FLAG: u32 = 0x0000_0001;
/// Bytes occupied by the final `tkhd` width and height fields.
const TRACK_DIMENSIONS_BYTES: usize = 8;
/// Distance from the end of `tkhd` to its width field.
const TRACK_WIDTH_FROM_END: usize = 8;
/// Distance from the end of `tkhd` to its height field.
const TRACK_HEIGHT_FROM_END: usize = 4;
/// Fractional-bit count in unsigned 16.16 `tkhd` dimensions.
const FIXED_16_16_FRACTION_BITS: u32 = 16;
/// Offset of `handler_type` in an `hdlr` FullBox payload.
const HANDLER_TYPE_OFFSET: usize = 8;
/// Version 0 `mdhd` timescale offset.
const MEDIA_HEADER_V0_TIMESCALE_OFFSET: usize = 12;
/// Version 0 `mdhd` duration offset.
const MEDIA_HEADER_V0_DURATION_OFFSET: usize = 16;
/// Version 0 `mdhd` packed-language offset.
const MEDIA_HEADER_V0_LANGUAGE_OFFSET: usize = 20;
/// Version 1 `mdhd` timescale offset.
const MEDIA_HEADER_V1_TIMESCALE_OFFSET: usize = 20;
/// Version 1 `mdhd` duration offset.
const MEDIA_HEADER_V1_DURATION_OFFSET: usize = 24;
/// Version 1 `mdhd` packed-language offset.
const MEDIA_HEADER_V1_LANGUAGE_OFFSET: usize = 32;
/// Encoded width of an `mdhd` packed-language field.
const PACKED_LANGUAGE_BYTES: usize = 2;
/// Number of ISO 639 letters packed into an `mdhd` language field.
const PACKED_LANGUAGE_LETTERS: usize = 3;
/// Bit positions of the three five-bit packed-language characters.
const PACKED_LANGUAGE_SHIFTS: [u32; PACKED_LANGUAGE_LETTERS] = [10, 5, 0];
/// Mask selecting one packed-language character.
const PACKED_LANGUAGE_CHARACTER_MASK: u16 = 0x001F;
/// Smallest valid packed-language character value (`a`).
const PACKED_LANGUAGE_MIN_CHARACTER: u8 = 1;
/// Largest valid packed-language character value (`z`).
const PACKED_LANGUAGE_MAX_CHARACTER: u8 = 26;
/// ASCII bias converting packed-language values 1 through 26 to `a` through `z`.
const PACKED_LANGUAGE_ASCII_BIAS: u8 = b'a' - PACKED_LANGUAGE_MIN_CHARACTER;
/// Offset of sample entries after the `stsd` FullBox prefix and entry count.
const SAMPLE_DESCRIPTION_ENTRIES_OFFSET: usize = 8;
/// Fixed payload bytes in a visual sample entry before child boxes.
const VISUAL_SAMPLE_ENTRY_BYTES: usize = 78;
/// Visual sample-entry width offset.
const VISUAL_SAMPLE_WIDTH_OFFSET: usize = 24;
/// Visual sample-entry height offset.
const VISUAL_SAMPLE_HEIGHT_OFFSET: usize = 26;
/// Minimum AVC decoder configuration payload through `profile_idc`.
const AVC_CONFIG_MIN_BYTES: usize = 2;
/// AVC and HEVC decoder-configuration profile byte offset.
const DECODER_CONFIG_PROFILE_OFFSET: usize = 1;
/// Minimum HEVC decoder configuration payload through luma bit depth.
const HEVC_CONFIG_MIN_BYTES: usize = 18;
/// Mask selecting `general_profile_idc` from an HEVC configuration record.
const HEVC_PROFILE_IDC_MASK: u8 = 0x1F;
/// `bitDepthLumaMinus8` byte offset in an HEVC configuration record.
const HEVC_BIT_DEPTH_OFFSET: usize = 17;
/// Base luma bit depth added to `bitDepthLumaMinus8`.
const HEVC_BASE_BIT_DEPTH: u8 = 8;
/// Mask selecting `bitDepthLumaMinus8` from its configuration byte.
const HEVC_BIT_DEPTH_MASK: u8 = 0x07;
/// Minimum `nclx` color-information payload through transfer characteristics.
const NCLX_COLOR_MIN_BYTES: usize = 8;
/// Offset of `transfer_characteristics` in an `nclx` payload.
const NCLX_TRANSFER_CHARACTERISTICS_OFFSET: usize = 6;
/// `nclx` transfer-characteristics value for SMPTE ST 2084 perceptual quantization.
const NCLX_TRANSFER_PQ: u16 = 16;
/// Fixed payload bytes in a version 0 audio sample entry.
const AUDIO_SAMPLE_ENTRY_V0_BYTES: usize = 28;
/// Audio sample-entry version field offset.
const AUDIO_SAMPLE_ENTRY_VERSION_OFFSET: usize = 8;
/// Version 0/1 audio sample-entry channel-count offset.
const AUDIO_SAMPLE_ENTRY_CHANNELS_OFFSET: usize = 16;
/// QuickTime audio sample-entry version 1.
const AUDIO_SAMPLE_ENTRY_VERSION_1: u16 = 1;
/// QuickTime audio sample-entry version 2.
const AUDIO_SAMPLE_ENTRY_VERSION_2: u16 = 2;
/// Version 2 audio sample-entry channel-count offset.
const AUDIO_SAMPLE_ENTRY_V2_CHANNELS_OFFSET: usize = 40;
/// Child-box offset after a version 1 audio sample entry.
const AUDIO_SAMPLE_ENTRY_V1_CHILD_OFFSET: usize = 44;
/// Child-box offset after a version 2 audio sample entry.
const AUDIO_SAMPLE_ENTRY_V2_CHILD_OFFSET: usize = 64;
/// Bytes in the FullBox prefix before MPEG-4 `esds` descriptors.
const ESDS_FULL_BOX_BYTES: usize = 4;
/// MPEG-4 descriptor tag for `DecoderSpecificInfo`.
const DECODER_SPECIFIC_INFO_TAG: u8 = 0x05;
/// Bytes occupied by an MPEG-4 descriptor tag.
const DESCRIPTOR_TAG_BYTES: usize = 1;
/// Maximum bytes in an MPEG-4 expandable descriptor length.
const MAX_DESCRIPTOR_LENGTH_BYTES: usize = 4;
/// Data bits contributed by each expandable descriptor-length byte.
const DESCRIPTOR_LENGTH_DATA_BITS: u32 = 7;
/// Mask selecting data bits from an expandable descriptor-length byte.
const DESCRIPTOR_LENGTH_DATA_MASK: u8 = 0x7F;
/// Continuation bit in an expandable descriptor-length byte.
const DESCRIPTOR_LENGTH_CONTINUES: u8 = 0x80;
/// Bit shift exposing the first five-bit AAC audio object type.
const AAC_OBJECT_TYPE_SHIFT: u32 = 3;
/// AAC Low Complexity audio object type.
const AAC_OBJECT_TYPE_LOW_COMPLEXITY: u8 = 2;
/// AAC Spectral Band Replication audio object type.
const AAC_OBJECT_TYPE_SBR: u8 = 5;
/// AAC Parametric Stereo audio object type.
const AAC_OBJECT_TYPE_PARAMETRIC_STEREO: u8 = 29;
/// `stts` entry-count offset after its FullBox prefix.
const TIME_TO_SAMPLE_COUNT_OFFSET: usize = 4;
/// Offset of the first `stts` timing entry.
const TIME_TO_SAMPLE_ENTRIES_OFFSET: usize = 8;
/// Bytes in one `(sample_count, sample_delta)` timing entry.
const TIME_TO_SAMPLE_ENTRY_BYTES: usize = 8;
/// Offset of `sample_delta` within an `stts` entry.
const TIME_TO_SAMPLE_DELTA_OFFSET: usize = 4;
/// `stsz.sample_size` offset after its FullBox prefix.
const SAMPLE_SIZE_DEFAULT_OFFSET: usize = 4;
/// `stsz.sample_count` offset.
const SAMPLE_SIZE_COUNT_OFFSET: usize = 8;
/// Offset of per-sample sizes when no default size is supplied.
const SAMPLE_SIZE_ENTRIES_OFFSET: usize = 12;
/// Bytes in one per-sample `stsz` size entry.
const SAMPLE_SIZE_ENTRY_BYTES: usize = 4;

/// A box whose payload is borrowed from an already bounded parent.
#[derive(Clone, Copy)]
struct BoxView<'a> {
    kind: [u8; BOX_TYPE_BYTES],
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
        if self.data.len().saturating_sub(self.offset) < BOX_HEADER_BYTES {
            self.offset = self.data.len();
            return Some(Err(invalid("truncated ISO-BMFF box")));
        }
        let start = self.offset;
        let size32 = match u32_be(self.data, start) {
            Ok(value) => value,
            Err(error) => return Some(Err(error)),
        };
        let kind = match fourcc(self.data, start + BOX_TYPE_OFFSET) {
            Ok(value) => value,
            Err(error) => return Some(Err(error)),
        };
        // A size of 1 selects the 16-byte header with a 64-bit `largesize`; a size of 0 extends the
        // box through the end of its parent. All other sizes include the ordinary eight-byte
        // header.
        let (header, size) = if size32 == BOX_EXTENDED_SIZE {
            if self.data.len().saturating_sub(start) < EXTENDED_BOX_HEADER_BYTES {
                self.offset = self.data.len();
                return Some(Err(invalid("truncated extended ISO-BMFF box")));
            }
            match u64_be(self.data, start + BOX_LARGE_SIZE_OFFSET).and_then(|size| {
                usize::try_from(size).map_err(|_| invalid("ISO-BMFF box is too large"))
            }) {
                Ok(size) => (EXTENDED_BOX_HEADER_BYTES, size),
                Err(error) => return Some(Err(error)),
            }
        } else if size32 == BOX_TO_END_SIZE {
            (BOX_HEADER_BYTES, self.data.len() - start)
        } else {
            (BOX_HEADER_BYTES, size32 as usize)
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
    handler: Option<[u8; BOX_TYPE_BYTES]>,
    language: Option<Language>,
    timescale: u64,
    duration: u64,
    codec: Option<[u8; BOX_TYPE_BYTES]>,
    width: u64,
    height: u64,
    channels: u64,
    average_sample_delta: Option<f64>,
    sample_bytes: Option<u64>,
    audio_profile: Option<AudioProfile>,
    video_profile: Option<crate::meta::fields::VideoProfile>,
    dynamic_range: Option<VideoDynamicRange>,
}

/// Detects an ISO-BMFF file-type or movie box at the start of a file.
pub(in crate::probe) fn matches(prefix: &[u8]) -> bool {
    prefix.len() >= BOX_HEADER_BYTES
        && matches!(
            &prefix[BOX_TYPE_OFFSET..BOX_HEADER_BYTES],
            b"ftyp" | b"moov"
        )
}

/// Probes a detected ISO-BMFF container.
pub(in crate::probe) fn probe(input: &mut ProbeInput) -> Result<ProbeResult, ProbeError> {
    let file_len = input.len();
    let top_level = (|| -> io::Result<_> {
        let file = input.file();
        let mut offset = 0_u64;
        let mut ftyp = None;
        let mut moov = None;
        while offset < file_len {
            file.seek(SeekFrom::Start(offset))?;
            let mut header = [0_u8; EXTENDED_BOX_HEADER_BYTES];
            file.read_exact(&mut header[..BOX_HEADER_BYTES])?;
            let size32 = u32::from_be_bytes(
                header[..BOX_SIZE32_BYTES]
                    .try_into()
                    .expect("four-byte slice"),
            );
            let kind: [u8; BOX_TYPE_BYTES] = header[BOX_TYPE_OFFSET..BOX_HEADER_BYTES]
                .try_into()
                .expect("four-byte slice");
            // Apply the same ordinary, extended-size, and to-end size rules used by the in-memory
            // box iterator.
            let (header_size, size) = if size32 == BOX_EXTENDED_SIZE {
                file.read_exact(&mut header[BOX_HEADER_BYTES..EXTENDED_BOX_HEADER_BYTES])?;
                (
                    EXTENDED_BOX_HEADER_BYTES as u64,
                    u64::from_be_bytes(
                        header[BOX_HEADER_BYTES..EXTENDED_BOX_HEADER_BYTES]
                            .try_into()
                            .expect("eight-byte slice"),
                    ),
                )
            } else if size32 == BOX_TO_END_SIZE {
                (BOX_HEADER_BYTES as u64, file_len - offset)
            } else {
                (BOX_HEADER_BYTES as u64, u64::from(size32))
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
        Ok((ftyp, moov))
    })()
    .map_err(|error| ProbeError::from_probe(MediaFormat::Mp4, error))?;
    let (ftyp, moov) = top_level;
    // `ftyp` is a short list of four-byte brands. Only a small bounded prefix is needed to
    // recognize QuickTime's `qt  ` brand.
    let ftyp = ftyp
        .map(|(offset, size)| {
            read_region(
                input.file(),
                offset,
                size.min(MAX_FILE_TYPE_BYTES),
                file_len,
            )
        })
        .transpose()
        .map_err(|error| ProbeError::from_probe(MediaFormat::Mp4, error))?;
    let format = if ftyp.as_deref().is_some_and(is_quicktime_brand) {
        MediaFormat::Mov
    } else {
        MediaFormat::Mp4
    };
    let (moov_offset, moov_size) = moov
        .ok_or_else(|| invalid("ISO-BMFF movie box not found"))
        .map_err(|error| ProbeError::from_probe(format, error))?;
    let moov = read_region(input.file(), moov_offset, moov_size, file_len)
        .map_err(|error| ProbeError::from_probe(format, error))?;
    parse_movie(&moov, format).map_err(|error| ProbeError::from_probe(format, error))
}

fn is_quicktime_brand(ftyp: &[u8]) -> bool {
    // The payload starts with major_brand and minor_version, followed by zero or more compatible
    // brands. Treating every four-byte word as a candidate safely includes the major brand and
    // ignores the numeric minor version.
    ftyp.chunks_exact(BOX_TYPE_BYTES)
        .any(|brand| brand == b"qt  ")
}

fn parse_movie(data: &[u8], format: MediaFormat) -> io::Result<ProbeResult> {
    let mut media = ProbeResult::new(format);
    let mut movie_duration = None;
    let mut tracks = Vec::new();
    for child in Boxes::new(data) {
        let child = child?;
        match &child.kind {
            b"mvhd" => movie_duration = parse_duration(child.payload)?,
            b"trak" => tracks.push(parse_track(child.payload, INITIAL_BOX_DEPTH)?),
            _ => {}
        }
    }
    media.duration = movie_duration.and_then(|seconds| Duration::try_from_secs_f64(seconds).ok());

    for track in &tracks {
        // Handler types classify the media independently of its sample-entry codec. The four
        // subtitle values cover closed captions, subtitles, MPEG-4 subtitle systems, and QuickTime
        // text tracks respectively.
        match track.handler.as_ref() {
            Some(b"soun") => media.tracks.push(MediaTrack::Audio(audio_stream(track))),
            Some(b"vide") => media.tracks.push(MediaTrack::Video(video_stream(track))),
            Some(b"clcp" | b"sbtl" | b"subt" | b"text") => {
                media
                    .tracks
                    .push(MediaTrack::Subtitle(subtitle_stream(track)));
            }
            _ => {}
        }
    }
    Ok(media)
}

fn audio_stream(track: &Track) -> AudioTrack {
    AudioTrack {
        info: TrackInfo {
            is_enabled: track.enabled,
            language: track.language,
            ..TrackInfo::default()
        },
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

fn video_stream(track: &Track) -> VideoTrack {
    let frame_rate = match (track.average_sample_delta, track.timescale) {
        (Some(delta), timescale) if delta > 0.0 && timescale > 0 => {
            Some((timescale as f64 / delta) as f32)
        }
        _ => None,
    };
    VideoTrack {
        info: TrackInfo {
            is_enabled: track.enabled,
            language: track.language,
            ..TrackInfo::default()
        },
        codec: track.codec.as_ref().and_then(video_codec),
        profile: track.video_profile.clone(),
        width: pixel_dimension(track.width),
        height: pixel_dimension(track.height),
        resolution: video_resolution(track.width, track.height, None),
        frame_rate,
        dynamic_range: track.dynamic_range.clone(),
    }
}

fn subtitle_stream(track: &Track) -> SubtitleTrack {
    SubtitleTrack {
        info: TrackInfo {
            is_enabled: track.enabled,
            language: track.language,
            ..TrackInfo::default()
        },
        codec: track.codec.as_ref().and_then(subtitle_codec),
    }
}

fn parse_duration(data: &[u8]) -> io::Result<Option<f64>> {
    let version = *data
        .first()
        .ok_or_else(|| invalid("truncated movie header"))?;
    // `mvhd` version 1 has 64-bit creation/modification times and duration; version 0 uses 32-bit
    // fields. The timescale remains 32-bit in both.
    let (timescale_offset, duration_offset, duration_size) = if version == FULL_BOX_VERSION_1 {
        (
            MOVIE_HEADER_V1_TIMESCALE_OFFSET,
            MOVIE_HEADER_V1_DURATION_OFFSET,
            DURATION64_BYTES,
        )
    } else {
        (
            MOVIE_HEADER_V0_TIMESCALE_OFFSET,
            MOVIE_HEADER_V0_DURATION_OFFSET,
            DURATION32_BYTES,
        )
    };
    let timescale = u64::from(u32_be(data, timescale_offset)?);
    let duration = if duration_size == DURATION64_BYTES {
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
            b"mdia" => parse_media(child.payload, &mut track, depth + BOX_DEPTH_INCREMENT)?,
            _ => {}
        }
    }
    Ok(track)
}

fn parse_track_header(data: &[u8], track: &mut Track) -> io::Result<()> {
    // `tkhd` is a FullBox: enabled is flag bit 0, and the final width/height fields are unsigned
    // 16.16 fixed-point values regardless of box version.
    let flags = u32_be(data, 0)? & FULL_BOX_FLAGS_MASK;
    track.enabled = flags & TRACK_ENABLED_FLAG != 0;
    if data.len() >= TRACK_DIMENSIONS_BYTES {
        track.width = u64::from(u32_be(data, data.len() - TRACK_WIDTH_FROM_END)?)
            >> FIXED_16_16_FRACTION_BITS;
        track.height = u64::from(u32_be(data, data.len() - TRACK_HEIGHT_FROM_END)?)
            >> FIXED_16_16_FRACTION_BITS;
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
            // `hdlr` stores handler_type after the four-byte FullBox prefix and a four-byte
            // pre_defined field.
            b"hdlr" => track.handler = Some(fourcc(child.payload, HANDLER_TYPE_OFFSET)?),
            b"minf" => parse_children(child.payload, track, depth + BOX_DEPTH_INCREMENT)?,
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
            b"stbl" => parse_children(child.payload, track, depth + BOX_DEPTH_INCREMENT)?,
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
    // As in `mvhd`, version 1 widens creation/modification times and duration while timescale and
    // the packed language field retain their widths.
    let (timescale_offset, duration_offset, duration_size, language_offset) =
        if version == FULL_BOX_VERSION_1 {
            (
                MEDIA_HEADER_V1_TIMESCALE_OFFSET,
                MEDIA_HEADER_V1_DURATION_OFFSET,
                DURATION64_BYTES,
                MEDIA_HEADER_V1_LANGUAGE_OFFSET,
            )
        } else {
            (
                MEDIA_HEADER_V0_TIMESCALE_OFFSET,
                MEDIA_HEADER_V0_DURATION_OFFSET,
                DURATION32_BYTES,
                MEDIA_HEADER_V0_LANGUAGE_OFFSET,
            )
        };
    track.timescale = u64::from(u32_be(data, timescale_offset)?);
    track.duration = if duration_size == DURATION64_BYTES {
        u64_be(data, duration_offset)?
    } else {
        u64::from(u32_be(data, duration_offset)?)
    };
    track.language = data
        .get(language_offset..language_offset + PACKED_LANGUAGE_BYTES)
        .map(|_| u16_be(data, language_offset))
        .transpose()?
        .and_then(mp4_language);
    Ok(())
}

fn mp4_language(value: u16) -> Option<Language> {
    // ISO language codes pack three lowercase letters into 15 bits. Each 5-bit value is the ASCII
    // letter minus 0x60 (`a` => 1, `z` => 26); zero and 27..31 are not letters. Legacy QuickTime
    // numeric language codes do not use this packing and therefore intentionally fail this decoder.
    let mut identifier = String::with_capacity(PACKED_LANGUAGE_LETTERS);
    for shift in PACKED_LANGUAGE_SHIFTS {
        let letter = u8::try_from((value >> shift) & PACKED_LANGUAGE_CHARACTER_MASK).ok()?;
        if !(PACKED_LANGUAGE_MIN_CHARACTER..=PACKED_LANGUAGE_MAX_CHARACTER).contains(&letter) {
            return None;
        }
        identifier.push(char::from(PACKED_LANGUAGE_ASCII_BIAS + letter));
    }
    Language::from_identifier(&identifier)
}

fn parse_sample_description(data: &[u8], track: &mut Track) -> io::Result<()> {
    // `stsd` begins with a four-byte FullBox prefix and a four-byte entry count. The first sample
    // entry's type is the codec identifier retained by this metadata-only probe.
    let entries = data
        .get(SAMPLE_DESCRIPTION_ENTRIES_OFFSET..)
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
    // VisualSampleEntry has a 78-byte fixed payload before codec-specific child boxes. Width and
    // height are 16-bit integers at payload bytes 24 and 26.
    if entry.payload.len() < VISUAL_SAMPLE_ENTRY_BYTES {
        return Err(invalid("truncated video sample entry"));
    }
    track.width = u64::from(u16_be(entry.payload, VISUAL_SAMPLE_WIDTH_OFFSET)?);
    track.height = u64::from(u16_be(entry.payload, VISUAL_SAMPLE_HEIGHT_OFFSET)?);
    if matches!(&entry.kind, b"dvhe" | b"dvh1") {
        track.dynamic_range = Some(VideoDynamicRange::DolbyVision);
    }
    for child in Boxes::new(&entry.payload[VISUAL_SAMPLE_ENTRY_BYTES..]) {
        let child = child?;
        match &child.kind {
            b"avcC" if child.payload.len() >= AVC_CONFIG_MIN_BYTES => {
                // AVCDecoderConfigurationRecord byte 1 is AVCProfileIndication (`profile_idc`).
                track.video_profile = avc_profile(child.payload[DECODER_CONFIG_PROFILE_OFFSET]);
            }
            b"hvcC" if child.payload.len() >= HEVC_CONFIG_MIN_BYTES => {
                // HEVCDecoderConfigurationRecord stores general_profile_idc in byte 1's low five
                // bits and bitDepthLumaMinus8 in byte 17.
                let profile = child.payload[DECODER_CONFIG_PROFILE_OFFSET] & HEVC_PROFILE_IDC_MASK;
                let bit_depth = child
                    .payload
                    .get(HEVC_BIT_DEPTH_OFFSET)
                    .map(|value| HEVC_BASE_BIT_DEPTH + (value & HEVC_BIT_DEPTH_MASK));
                track.video_profile = hevc_profile(profile, bit_depth);
            }
            b"dvcC" | b"dvvC" => {
                track.dynamic_range = Some(VideoDynamicRange::DolbyVision);
            }
            b"colr"
                if child.payload.starts_with(b"nclx")
                    && child.payload.len() >= NCLX_COLOR_MIN_BYTES
                    // `nclx` transfer_characteristics 16 is SMPTE ST 2084 (PQ), the transfer
                    // function used by HDR10.
                    && u16_be(child.payload, NCLX_TRANSFER_CHARACTERISTICS_OFFSET)?
                        == NCLX_TRANSFER_PQ =>
            {
                track.dynamic_range = Some(VideoDynamicRange::HDR10);
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_audio_entry(entry: BoxView<'_>, track: &mut Track) -> io::Result<()> {
    // AudioSampleEntry version 0 has a 28-byte fixed payload. QuickTime versions 1 and 2 append 16
    // and 36 bytes respectively before child boxes; version 2 also carries a 32-bit channel count
    // at payload byte 40.
    if entry.payload.len() < AUDIO_SAMPLE_ENTRY_V0_BYTES {
        return Err(invalid("truncated audio sample entry"));
    }
    let version = u16_be(entry.payload, AUDIO_SAMPLE_ENTRY_VERSION_OFFSET)?;
    track.channels = u64::from(if version == AUDIO_SAMPLE_ENTRY_VERSION_2 {
        u32_be(entry.payload, AUDIO_SAMPLE_ENTRY_V2_CHANNELS_OFFSET)?
    } else {
        u32::from(u16_be(entry.payload, AUDIO_SAMPLE_ENTRY_CHANNELS_OFFSET)?)
    });
    let child_offset = match version {
        AUDIO_SAMPLE_ENTRY_VERSION_1 => AUDIO_SAMPLE_ENTRY_V1_CHILD_OFFSET,
        AUDIO_SAMPLE_ENTRY_VERSION_2 => AUDIO_SAMPLE_ENTRY_V2_CHILD_OFFSET,
        _ => AUDIO_SAMPLE_ENTRY_V0_BYTES,
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
    // DecoderSpecificInfo is nested inside the ES and DecoderConfig descriptors. Scan the
    // already-bounded `esds` payload for a well-formed tag 0x05 rather than duplicating the
    // complete MPEG-4 descriptor grammar.
    for tag_offset in ESDS_FULL_BOX_BYTES..data.len().saturating_sub(DESCRIPTOR_TAG_BYTES) {
        if data[tag_offset] != DECODER_SPECIFIC_INFO_TAG {
            continue;
        }
        let mut offset = tag_offset + DESCRIPTOR_TAG_BYTES;
        let mut size = 0_usize;
        let mut complete = false;
        for _ in 0..MAX_DESCRIPTOR_LENGTH_BYTES {
            let Some(byte) = data.get(offset).copied() else {
                break;
            };
            offset += DESCRIPTOR_TAG_BYTES;
            size = size
                .checked_shl(DESCRIPTOR_LENGTH_DATA_BITS)?
                .checked_add(usize::from(byte & DESCRIPTOR_LENGTH_DATA_MASK))?;
            if byte & DESCRIPTOR_LENGTH_CONTINUES == 0 {
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
        // The first five AudioSpecificConfig bits are audioObjectType: 2 = AAC LC, 5 = SBR, and 29
        // = PS (the latter two are HE-AAC).
        return match config >> AAC_OBJECT_TYPE_SHIFT {
            AAC_OBJECT_TYPE_LOW_COMPLEXITY => Some(AudioProfile::LowComplexity),
            AAC_OBJECT_TYPE_SBR | AAC_OBJECT_TYPE_PARAMETRIC_STEREO => {
                Some(AudioProfile::HighEfficiency)
            }
            _ => None,
        };
    }
    None
}

fn parse_time_to_sample(data: &[u8], track: &mut Track) -> io::Result<()> {
    // `stts` stores run-length pairs of (sample_count, sample_delta) after its four-byte FullBox
    // prefix and four-byte entry count. The weighted mean delta later converts the media timescale
    // into an average frame rate.
    let count = usize::try_from(u32_be(data, TIME_TO_SAMPLE_COUNT_OFFSET)?)
        .map_err(|_| invalid("sample timing count is too large"))?;
    let mut samples = 0_u64;
    let mut duration = 0_u64;
    for index in 0..count {
        let offset = TIME_TO_SAMPLE_ENTRIES_OFFSET + index * TIME_TO_SAMPLE_ENTRY_BYTES;
        let sample_count = u64::from(u32_be(data, offset)?);
        let delta = u64::from(u32_be(data, offset + TIME_TO_SAMPLE_DELTA_OFFSET)?);
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
    // `stsz` supplies one default byte size for every sample, or zero followed by a 32-bit size for
    // each sample. Summing sizes avoids reading `mdat`.
    let default_size = u64::from(u32_be(data, SAMPLE_SIZE_DEFAULT_OFFSET)?);
    let count = usize::try_from(u32_be(data, SAMPLE_SIZE_COUNT_OFFSET)?)
        .map_err(|_| invalid("sample size count is too large"))?;
    track.sample_bytes = if default_size > 0 {
        default_size.checked_mul(count as u64)
    } else {
        let mut total = 0_u64;
        for index in 0..count {
            total = total
                .checked_add(u64::from(u32_be(
                    data,
                    SAMPLE_SIZE_ENTRIES_OFFSET + index * SAMPLE_SIZE_ENTRY_BYTES,
                )?))
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
    // Duration is in media-timescale ticks, so bits * timescale / duration is the average number of
    // bits per second.
    let value = (bytes as u128)
        .checked_mul(u128::from(u8::BITS))?
        .checked_mul(u128::from(track.timescale))?
        / u128::from(track.duration);
    u32::try_from(value).ok()
}

crate::unit_tests!("mp4.test.rs");
