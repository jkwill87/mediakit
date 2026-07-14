//! Bounded, metadata-only parsers for supported container families.
//!
//! Each parser reads only the structural metadata needed to populate a
//! [`MediaInfo`](super::MediaInfo): container headers, stream descriptions,
//! and timing tables. Media payloads are never decoded. File offsets and
//! lengths are checked before reads, and individual metadata allocations are
//! capped by [`binary::MAX_METADATA_BYTES`].
//!
//! Container modules keep their byte-order and framing rules local. The
//! helpers in this module normalize identifiers shared by several families,
//! including FourCC codec codes, WAVE format tags, channel masks, codec
//! profiles, and display-resolution classes.

mod asf;
mod avi;
mod binary;
mod matroska;
mod mp4;
mod mpeg_ts;

pub(super) use asf::parse as parse_asf;
pub(super) use avi::parse as parse_avi;
pub(super) use matroska::parse as parse_matroska;
pub(super) use mp4::parse as parse_mp4;
pub(super) use mpeg_ts::{matches as matches_mpeg_ts, parse as parse_mpeg_ts};

use super::MediaInfo as Probe;
use crate::meta::fields::{AudioCodec, AudioLayout, VideoCodec, VideoProfile, VideoResolution};

/// `WAVEFORMATEXTENSIBLE` format tag used by ASF and AVI audio headers.
pub(super) const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;

/// Windows speaker-mask bit for the low-frequency-effects channel.
const SPEAKER_LOW_FREQUENCY: u32 = 0x0000_0008;
/// Windows speaker-position bits normalized as height channels.
const SPEAKER_HEIGHT_MASK: u32 = 0x0002_C000 | 0x0FC0_0000;
/// Channel count conventionally interpreted as a 5.1 layout without a mask.
const SURROUND_5_1_CHANNELS: u8 = 6;
/// Full-range channels in an inferred 5.1 layout.
const SURROUND_5_1_FULL_CHANNELS: u8 = 5;
/// Channel count conventionally interpreted as a 7.1 layout without a mask.
const SURROUND_7_1_CHANNELS: u8 = 8;
/// Full-range channels in an inferred 7.1 layout.
const SURROUND_7_1_FULL_CHANNELS: u8 = 7;
/// Low-frequency channels in inferred 5.1 and 7.1 layouts.
const SURROUND_LFE_CHANNELS: u8 = 1;

/// Nominal width of the 8K UHD raster.
const UHD_8K_WIDTH: u64 = 7_680;
/// Nominal height of the 8K UHD raster.
const UHD_8K_HEIGHT: u64 = 4_320;
/// Nominal width of the 4K UHD raster.
const UHD_4K_WIDTH: u64 = 3_840;
/// Nominal height of the 4K UHD raster.
const UHD_4K_HEIGHT: u64 = 2_160;
/// Nominal width of the 1080-line HD raster.
const HD_1080_WIDTH: u64 = 1_920;
/// Nominal height of the 1080-line HD raster.
const HD_1080_HEIGHT: u64 = 1_080;
/// Nominal width of the 720-line HD raster.
const HD_720_WIDTH: u64 = 1_280;
/// Nominal height of the 720-line HD raster.
const HD_720_HEIGHT: u64 = 720;
/// Nominal width used as the 480-line SD boundary.
const SD_480_WIDTH: u64 = 720;
/// Nominal height used as the 480-line SD boundary.
const SD_480_HEIGHT: u64 = 480;
/// Nominal width used as the 360-line SD boundary.
const SD_360_WIDTH: u64 = 640;
/// Nominal height used as the 360-line SD boundary.
const SD_360_HEIGHT: u64 = 360;

/// Microsoft PCM `WAVEFORMATEX` tag.
const WAVE_FORMAT_PCM: u16 = 0x0001;
/// Microsoft IEEE floating-point `WAVEFORMATEX` tag.
const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;
/// Microsoft MPEG Layer-3 `WAVEFORMATEX` tag.
const WAVE_FORMAT_MPEGLAYER3: u16 = 0x0055;
/// Generic AAC `WAVEFORMATEX` tag.
const WAVE_FORMAT_AAC: u16 = 0x00FF;
/// Nokia MPEG ADTS AAC `WAVEFORMATEX` tag.
const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: u16 = 0x1600;
/// Vodafone MPEG ADTS AAC `WAVEFORMATEX` tag.
const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: u16 = 0x1602;
/// Windows Media Audio Standard `WAVEFORMATEX` tag.
const WAVE_FORMAT_WMAUDIO2: u16 = 0x0161;
/// Windows Media Audio Professional `WAVEFORMATEX` tag.
const WAVE_FORMAT_WMAUDIO3: u16 = 0x0162;
/// Windows Media Audio Lossless `WAVEFORMATEX` tag.
const WAVE_FORMAT_WMAUDIO_LOSSLESS: u16 = 0x0163;
/// Common AC-3 `WAVEFORMATEX` tag used by multimedia containers.
const WAVE_FORMAT_AC3: u16 = 0x2000;
/// Common DTS `WAVEFORMATEX` tag used by multimedia containers.
const WAVE_FORMAT_DTS: u16 = 0x2001;
/// FLAC `WAVEFORMATEX` tag.
const WAVE_FORMAT_FLAC: u16 = 0xF1AC;
/// Opus `WAVEFORMATEX` tag.
const WAVE_FORMAT_OPUS: u16 = 0x704F;

/// H.264 Baseline `profile_idc`.
const AVC_PROFILE_BASELINE: u8 = 66;
/// H.264 Main `profile_idc`.
const AVC_PROFILE_MAIN: u8 = 77;
/// H.264 Extended `profile_idc`.
const AVC_PROFILE_EXTENDED: u8 = 88;
/// H.264 High `profile_idc`.
const AVC_PROFILE_HIGH: u8 = 100;
/// H.264 High 10 `profile_idc`.
const AVC_PROFILE_HIGH_10: u8 = 110;
/// H.264 High 4:2:2 `profile_idc`.
const AVC_PROFILE_HIGH_422: u8 = 122;
/// H.264 High 4:4:4 Predictive `profile_idc`.
const AVC_PROFILE_HIGH_444_PREDICTIVE: u8 = 244;
/// HEVC Main general-profile identifier.
const HEVC_PROFILE_MAIN: u8 = 1;
/// HEVC Main 10 general-profile identifier.
const HEVC_PROFILE_MAIN_10: u8 = 2;
/// Bit depth above which an HEVC stream is treated as Main 10.
const HEVC_MAIN_10_MIN_BIT_DEPTH: u8 = 8;

fn fourcc_string(value: &[u8; 4]) -> Option<String> {
    let value = std::str::from_utf8(value)
        .ok()?
        .trim_matches(char::from(0))
        .trim();
    (!value.is_empty()).then(|| value.to_owned())
}

/// Converts a channel count and optional Windows speaker mask into the
/// library's `full.sub.height` representation.
///
/// When a `WAVEFORMATEXTENSIBLE` mask is available, bit `0x8` identifies the
/// low-frequency-effects channel and the selected upper-speaker bits identify
/// height channels. Without a mask, conventional 5.1 and 7.1 layouts are
/// inferred from the channel count; other counts are treated as full-range.
fn audio_layout(channels: u64, channel_mask: Option<u32>) -> Option<AudioLayout> {
    let channels = u8::try_from(channels).ok().filter(|value| *value > 0)?;
    if let Some(mask) = channel_mask {
        let sub = u8::from(mask & SPEAKER_LOW_FREQUENCY != 0);
        let height = u8::try_from((mask & SPEAKER_HEIGHT_MASK).count_ones()).ok()?;
        return Some(AudioLayout {
            full: channels.saturating_sub(sub).saturating_sub(height),
            sub,
            height,
        });
    }
    Some(match channels {
        SURROUND_5_1_CHANNELS => AudioLayout {
            full: SURROUND_5_1_FULL_CHANNELS,
            sub: SURROUND_LFE_CHANNELS,
            height: 0,
        },
        SURROUND_7_1_CHANNELS => AudioLayout {
            full: SURROUND_7_1_FULL_CHANNELS,
            sub: SURROUND_LFE_CHANNELS,
            height: 0,
        },
        full => AudioLayout {
            full,
            sub: 0,
            height: 0,
        },
    })
}

/// Classifies stored pixel dimensions using the nominal raster boundary for
/// each resolution tier.
///
/// Either dimension may establish the tier because anamorphic and cropped
/// encodes do not always retain the corresponding standard width and height.
fn video_resolution(width: u64, height: u64, interlaced: Option<bool>) -> Option<VideoResolution> {
    let interlaced = interlaced.unwrap_or(false);
    if width >= UHD_8K_WIDTH || height >= UHD_8K_HEIGHT {
        return Some(VideoResolution::Uhd8k);
    }
    if width >= UHD_4K_WIDTH || height >= UHD_4K_HEIGHT {
        return Some(VideoResolution::Uhd4k);
    }
    if width >= HD_1080_WIDTH || height >= HD_1080_HEIGHT {
        return Some(if interlaced {
            VideoResolution::Hd1080i
        } else {
            VideoResolution::Hd1080p
        });
    }
    if width >= HD_720_WIDTH || height >= HD_720_HEIGHT {
        return Some(if interlaced {
            VideoResolution::Hd720i
        } else {
            VideoResolution::Hd720p
        });
    }
    if width >= SD_480_WIDTH || height >= SD_480_HEIGHT {
        return Some(if interlaced {
            VideoResolution::Sd480i
        } else {
            VideoResolution::Sd480P
        });
    }
    if width >= SD_360_WIDTH || height >= SD_360_HEIGHT {
        return Some(if interlaced {
            VideoResolution::Sd360i
        } else {
            VideoResolution::Sd360p
        });
    }
    None
}

/// Maps common ISO-BMFF, QuickTime, AVI, and ASF FourCC aliases to a video
/// codec.
fn video_codec(fourcc: &[u8; 4]) -> Option<VideoCodec> {
    let upper = fourcc.map(|value| value.to_ascii_uppercase());
    match &upper {
        b"AV01" => Some(VideoCodec::Av1),
        b"MPG2" | b"M2V1" | b"MPEG" => Some(VideoCodec::H262),
        b"AVC1" | b"AVC3" | b"H264" | b"X264" => Some(VideoCodec::H264),
        b"HVC1" | b"HEV1" | b"HEVC" | b"H265" | b"DVHE" | b"DVH1" => Some(VideoCodec::H265),
        b"MP4V" | b"XVID" | b"DIVX" | b"DX50" => Some(VideoCodec::Mpeg4Visual),
        b"VC-1" | b"DVC1" | b"WVC1" | b"WMV3" => Some(VideoCodec::Vc1),
        b"VP08" | b"VP80" => Some(VideoCodec::Vp8),
        b"VP09" | b"VP90" => Some(VideoCodec::Vp9),
        _ => None,
    }
}

/// Maps ISO-BMFF and QuickTime audio sample-entry FourCC values to a codec.
const fn audio_codec(fourcc: &[u8; 4]) -> Option<AudioCodec> {
    match fourcc {
        b"mp4a" | b"MP4A" => Some(AudioCodec::Aac),
        b"ac-3" | b"AC-3" => Some(AudioCodec::DolbyDigital),
        b"ec-3" | b"EC-3" => Some(AudioCodec::DolbyDigitalPlus),
        b"mlpa" | b"MLPA" => Some(AudioCodec::DolbyTrueHD),
        b"dtsc" | b"DTSC" => Some(AudioCodec::Dts),
        b"dtsh" | b"DTSH" | b"dtsl" | b"DTSL" => Some(AudioCodec::DtsHD),
        b"dtsx" | b"DTSX" => Some(AudioCodec::DtsX),
        b"fLaC" | b"FLAC" => Some(AudioCodec::Flac),
        b"alac" | b"ALAC" => Some(AudioCodec::Alac),
        b"Opus" | b"OPUS" => Some(AudioCodec::Opus),
        b".mp3" | b"MP3 " => Some(AudioCodec::Mp3),
        b"lpcm" | b"LPCM" => Some(AudioCodec::Lpcm),
        b"sowt" | b"twos" | b"raw " | b"in24" | b"in32" => Some(AudioCodec::Pcm),
        _ => None,
    }
}

/// Maps `WAVEFORMATEX.wFormatTag` values to a codec.
///
/// The hexadecimal values are the registrations from Microsoft's multimedia
/// format-tag registry. `0xFFFE` (`WAVE_FORMAT_EXTENSIBLE`) is resolved by the
/// ASF and AVI parsers before this function is called.
const fn wave_audio_codec(format_tag: u16) -> Option<AudioCodec> {
    match format_tag {
        WAVE_FORMAT_PCM | WAVE_FORMAT_IEEE_FLOAT => Some(AudioCodec::Pcm),
        WAVE_FORMAT_MPEGLAYER3 => Some(AudioCodec::Mp3),
        WAVE_FORMAT_AAC
        | WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC
        | WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC => Some(AudioCodec::Aac),
        WAVE_FORMAT_WMAUDIO2 | WAVE_FORMAT_WMAUDIO3 | WAVE_FORMAT_WMAUDIO_LOSSLESS => {
            Some(AudioCodec::DolbyDigitalPlus)
        }
        WAVE_FORMAT_AC3 => Some(AudioCodec::DolbyDigital),
        WAVE_FORMAT_DTS => Some(AudioCodec::Dts),
        WAVE_FORMAT_FLAC => Some(AudioCodec::Flac),
        WAVE_FORMAT_OPUS => Some(AudioCodec::Opus),
        _ => None,
    }
}

/// Maps H.264 `profile_idc` values from an AVC decoder configuration record.
const fn avc_profile(profile: u8) -> Option<VideoProfile> {
    match profile {
        AVC_PROFILE_BASELINE => Some(VideoProfile::Baseline),
        AVC_PROFILE_MAIN => Some(VideoProfile::Main),
        AVC_PROFILE_EXTENDED => Some(VideoProfile::Extended),
        AVC_PROFILE_HIGH => Some(VideoProfile::High),
        AVC_PROFILE_HIGH_10 => Some(VideoProfile::High10),
        AVC_PROFILE_HIGH_422 => Some(VideoProfile::High422),
        AVC_PROFILE_HIGH_444_PREDICTIVE => Some(VideoProfile::High444Predictive),
        _ => None,
    }
}

/// Maps the HEVC general profile space used by decoder configuration records.
///
/// Main 10 is also inferred when the signalled luma bit depth exceeds eight,
/// which covers files whose profile field is incomplete but whose bit-depth
/// constraint is usable.
fn hevc_profile(profile: u8, bit_depth: Option<u8>) -> Option<VideoProfile> {
    if profile == HEVC_PROFILE_MAIN_10
        || bit_depth.is_some_and(|depth| depth > HEVC_MAIN_10_MIN_BIT_DEPTH)
    {
        Some(VideoProfile::Main10)
    } else if profile == HEVC_PROFILE_MAIN {
        Some(VideoProfile::Main)
    } else {
        None
    }
}
