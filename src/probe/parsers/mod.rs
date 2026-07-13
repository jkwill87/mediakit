//! Organizes bounded parsers for supported container families.

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

fn audio_layout(channels: u64, channel_mask: Option<u32>) -> Option<AudioLayout> {
    let channels = u8::try_from(channels).ok().filter(|value| *value > 0)?;
    if let Some(mask) = channel_mask {
        let sub = u8::from(mask & 0x8 != 0);
        let height_mask = 0x0002_C000 | 0x0FC0_0000;
        let height = u8::try_from((mask & height_mask).count_ones()).ok()?;
        return Some(AudioLayout {
            full: channels.saturating_sub(sub).saturating_sub(height),
            sub,
            height,
        });
    }
    Some(match channels {
        6 => AudioLayout {
            full: 5,
            sub: 1,
            height: 0,
        },
        8 => AudioLayout {
            full: 7,
            sub: 1,
            height: 0,
        },
        full => AudioLayout {
            full,
            sub: 0,
            height: 0,
        },
    })
}

fn video_resolution(width: u64, height: u64, interlaced: Option<bool>) -> Option<VideoResolution> {
    let interlaced = interlaced.unwrap_or(false);
    if width >= 7_680 || height >= 4_320 {
        return Some(VideoResolution::Uhd8k);
    }
    if width >= 3_840 || height >= 2_160 {
        return Some(VideoResolution::Uhd4k);
    }
    if width >= 1_920 || height >= 1_080 {
        return Some(if interlaced {
            VideoResolution::Hd1080i
        } else {
            VideoResolution::Hd1080p
        });
    }
    if width >= 1_280 || height >= 720 {
        return Some(if interlaced {
            VideoResolution::Hd720i
        } else {
            VideoResolution::Hd720p
        });
    }
    if width >= 720 || height >= 480 {
        return Some(if interlaced {
            VideoResolution::Sd480i
        } else {
            VideoResolution::Sd480P
        });
    }
    if width >= 640 || height >= 360 {
        return Some(if interlaced {
            VideoResolution::Sd360i
        } else {
            VideoResolution::Sd360p
        });
    }
    None
}

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

const fn wave_audio_codec(format_tag: u16) -> Option<AudioCodec> {
    match format_tag {
        0x0001 | 0x0003 => Some(AudioCodec::Pcm),
        0x0055 => Some(AudioCodec::Mp3),
        0x00FF | 0x1600 | 0x1602 => Some(AudioCodec::Aac),
        0x0161..=0x0163 => Some(AudioCodec::DolbyDigitalPlus),
        0x2000 => Some(AudioCodec::DolbyDigital),
        0x2001 => Some(AudioCodec::Dts),
        0xF1AC => Some(AudioCodec::Flac),
        0x704F => Some(AudioCodec::Opus),
        _ => None,
    }
}

const fn avc_profile(profile: u8) -> Option<VideoProfile> {
    match profile {
        66 => Some(VideoProfile::Baseline),
        77 => Some(VideoProfile::Main),
        88 => Some(VideoProfile::Extended),
        100 => Some(VideoProfile::High),
        110 => Some(VideoProfile::High10),
        122 => Some(VideoProfile::High422),
        244 => Some(VideoProfile::High444Predictive),
        _ => None,
    }
}

fn hevc_profile(profile: u8, bit_depth: Option<u8>) -> Option<VideoProfile> {
    if profile == 2 || bit_depth.is_some_and(|depth| depth > 8) {
        Some(VideoProfile::Main10)
    } else if profile == 1 {
        Some(VideoProfile::Main)
    } else {
        None
    }
}
