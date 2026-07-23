//! Decodes Windows media structures shared by ASF and AVI.
//!
//! Both containers embed `WAVEFORMATEX` audio descriptions and `BITMAPINFOHEADER` video
//! descriptions. Keeping those layouts here prevents their offsets, extensible-format handling,
//! and normalization rules from drifting apart.

use super::audio_layout;
use super::binary::{fourcc, i32_le, invalid, u16_le, u32_le};
use crate::meta::fields::AudioCodec;
use crate::meta::streams::{AudioStream, StreamInfo};
use std::io;

/// Minimum encoded size of `WAVEFORMATEX` through `nBlockAlign`.
pub(in crate::probe) const WAVE_FORMAT_MIN_BYTES: usize = 16;
/// Minimum `BITMAPINFOHEADER` size needed for signed dimensions.
pub(in crate::probe) const BITMAP_INFO_MIN_BYTES: usize = 12;
/// Minimum `BITMAPINFOHEADER` size needed through the compression FourCC.
pub(in crate::probe) const BITMAP_INFO_COMPRESSION_BYTES: usize = 20;

/// `WAVEFORMATEX.nChannels` offset.
const WAVE_CHANNELS_OFFSET: usize = 2;
/// `WAVEFORMATEX.nAvgBytesPerSec` offset.
const WAVE_AVERAGE_BYTES_OFFSET: usize = 8;
/// `WAVEFORMATEXTENSIBLE` format tag.
const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;
/// Minimum encoded size of `WAVEFORMATEXTENSIBLE` used by these probes.
const WAVE_EXTENSIBLE_MIN_BYTES: usize = 40;
/// `WAVEFORMATEXTENSIBLE.dwChannelMask` offset.
const WAVE_CHANNEL_MASK_OFFSET: usize = 20;
/// Offset of the format tag embedded at the start of the SubFormat GUID.
const WAVE_SUBFORMAT_TAG_OFFSET: usize = 24;

/// `BITMAPINFOHEADER.biWidth` offset.
const BITMAP_WIDTH_OFFSET: usize = 4;
/// `BITMAPINFOHEADER.biHeight` offset.
const BITMAP_HEIGHT_OFFSET: usize = 8;
/// `BITMAPINFOHEADER.biCompression` FourCC offset.
const BITMAP_COMPRESSION_OFFSET: usize = 16;

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

/// Parsed `BITMAPINFOHEADER` fields shared by the video probes.
pub(in crate::probe) struct BitmapInfo {
    /// Magnitude of the signed coded width.
    pub(in crate::probe) width: u64,
    /// Magnitude of the signed coded height.
    pub(in crate::probe) height: u64,
    /// Compression FourCC when the input includes that field.
    pub(in crate::probe) compression: Option<[u8; 4]>,
}

/// Parses the Windows bitmap dimensions and optional compression identifier.
pub(in crate::probe) fn parse_bitmap_info(data: &[u8]) -> io::Result<BitmapInfo> {
    if data.len() < BITMAP_INFO_MIN_BYTES {
        return Err(invalid("truncated BITMAPINFOHEADER"));
    }
    Ok(BitmapInfo {
        width: u64::from(i32_le(data, BITMAP_WIDTH_OFFSET)?.unsigned_abs()),
        height: u64::from(i32_le(data, BITMAP_HEIGHT_OFFSET)?.unsigned_abs()),
        compression: (data.len() >= BITMAP_INFO_COMPRESSION_BYTES)
            .then(|| fourcc(data, BITMAP_COMPRESSION_OFFSET))
            .transpose()?,
    })
}

/// Converts a Windows wave-format structure into normalized audio metadata.
pub(in crate::probe) fn parse_wave_audio(data: &[u8], info: StreamInfo) -> io::Result<AudioStream> {
    if data.len() < WAVE_FORMAT_MIN_BYTES {
        return Err(invalid("truncated WAVEFORMATEX"));
    }
    let mut format_tag = u16_le(data, 0)?;
    let channels = u64::from(u16_le(data, WAVE_CHANNELS_OFFSET)?);
    let average_bytes = u32_le(data, WAVE_AVERAGE_BYTES_OFFSET)?;
    let mut channel_mask = None;
    if format_tag == WAVE_FORMAT_EXTENSIBLE && data.len() >= WAVE_EXTENSIBLE_MIN_BYTES {
        channel_mask = Some(u32_le(data, WAVE_CHANNEL_MASK_OFFSET)?);
        format_tag = u16_le(data, WAVE_SUBFORMAT_TAG_OFFSET)?;
    }
    Ok(AudioStream {
        info,
        codec: wave_audio_codec(format_tag),
        profile: None,
        layout: audio_layout(channels, channel_mask),
        bit_rate: average_bytes.checked_mul(u8::BITS),
    })
}

/// Maps `WAVEFORMATEX.wFormatTag` registry values to normalized codecs.
const fn wave_audio_codec(format_tag: u16) -> Option<AudioCodec> {
    match format_tag {
        WAVE_FORMAT_PCM | WAVE_FORMAT_IEEE_FLOAT => Some(AudioCodec::Pcm),
        WAVE_FORMAT_MPEGLAYER3 => Some(AudioCodec::Mp3),
        WAVE_FORMAT_AAC | WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC | WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC => {
            Some(AudioCodec::Aac)
        }
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

crate::unit_tests!("windows_media.test.rs");
