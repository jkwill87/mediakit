//! Verifies shared Windows media structure decoding.

use super::*;
use crate::meta::fields::{AudioCodec, AudioLayout};

#[test]
fn parses_extensible_wave_codec_layout_and_bit_rate() {
    let mut data = vec![0_u8; 40];
    data[0..2].copy_from_slice(&WAVE_FORMAT_EXTENSIBLE.to_le_bytes());
    data[WAVE_CHANNELS_OFFSET..WAVE_CHANNELS_OFFSET + 2].copy_from_slice(&8_u16.to_le_bytes());
    data[WAVE_AVERAGE_BYTES_OFFSET..WAVE_AVERAGE_BYTES_OFFSET + 4]
        .copy_from_slice(&96_000_u32.to_le_bytes());
    data[WAVE_CHANNEL_MASK_OFFSET..WAVE_CHANNEL_MASK_OFFSET + 4]
        .copy_from_slice(&0x0000_0008_u32.to_le_bytes());
    data[WAVE_SUBFORMAT_TAG_OFFSET..WAVE_SUBFORMAT_TAG_OFFSET + 2]
        .copy_from_slice(&WAVE_FORMAT_AC3.to_le_bytes());

    let stream = parse_wave_audio(&data, StreamInfo::default()).unwrap();
    assert_eq!(stream.codec, Some(AudioCodec::DolbyDigital));
    assert_eq!(
        stream.layout,
        Some(AudioLayout {
            full: 7,
            sub: 1,
            height: 0,
        })
    );
    assert_eq!(stream.bit_rate, Some(768_000));
}

#[test]
fn maps_windows_media_audio_formats_to_wma() {
    for format in [
        WAVE_FORMAT_WMAUDIO2,
        WAVE_FORMAT_WMAUDIO3,
        WAVE_FORMAT_WMAUDIO_LOSSLESS,
    ] {
        assert_eq!(wave_audio_codec(format), Some(AudioCodec::Wma));
    }
}

#[test]
fn parses_signed_bitmap_dimensions_and_optional_compression() {
    let mut data = vec![0_u8; BITMAP_INFO_COMPRESSION_BYTES];
    data[BITMAP_WIDTH_OFFSET..BITMAP_WIDTH_OFFSET + 4]
        .copy_from_slice(&(-1_920_i32).to_le_bytes());
    data[BITMAP_HEIGHT_OFFSET..BITMAP_HEIGHT_OFFSET + 4]
        .copy_from_slice(&(-1_080_i32).to_le_bytes());
    data[BITMAP_COMPRESSION_OFFSET..BITMAP_COMPRESSION_OFFSET + 4].copy_from_slice(b"H264");

    let bitmap = parse_bitmap_info(&data).unwrap();
    assert_eq!((bitmap.width, bitmap.height), (1_920, 1_080));
    assert_eq!(bitmap.compression, Some(*b"H264"));

    let bitmap = parse_bitmap_info(&data[..BITMAP_INFO_MIN_BYTES]).unwrap();
    assert_eq!(bitmap.compression, None);
}
