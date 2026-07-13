//! Verifies audio-codec metadata behavior.

use super::*;
use std::str::FromStr;

#[test]
fn aac_to_string() {
    assert_eq!(AudioCodec::Aac.to_string(), "aac");
}

#[test]
fn aac_from_str() {
    assert_eq!(AudioCodec::from_str("aac").unwrap(), AudioCodec::Aac);
}

#[test]
fn modern_container_codecs_round_trip() {
    assert_eq!(AudioCodec::Alac.to_string(), "alac");
    assert_eq!(AudioCodec::from_str("alac").unwrap(), AudioCodec::Alac);
    assert_eq!(AudioCodec::Opus.to_string(), "opus");
    assert_eq!(AudioCodec::from_str("opus").unwrap(), AudioCodec::Opus);
}

#[test]
fn dolby_atmos_to_string() {
    assert_eq!(AudioCodec::DolbyAtmos.to_string(), "dolby_atmos");
}

#[test]
fn dolby_atmos_from_str() {
    assert_eq!(
        AudioCodec::from_str("dolby_atmos").unwrap(),
        AudioCodec::DolbyAtmos
    );
}

#[test]
fn dolby_digital_to_string() {
    assert_eq!(AudioCodec::DolbyDigital.to_string(), "dolby_digital");
}

#[test]
fn dolby_digital_from_str() {
    assert_eq!(
        AudioCodec::from_str("dolby_digital").unwrap(),
        AudioCodec::DolbyDigital
    );
}

#[test]
fn dolby_digital_plus_to_string() {
    assert_eq!(
        AudioCodec::DolbyDigitalPlus.to_string(),
        "dolby_digital_plus"
    );
}

#[test]
fn dolby_digital_plus_from_str() {
    assert_eq!(
        AudioCodec::from_str("dolby_digital_plus").unwrap(),
        AudioCodec::DolbyDigitalPlus
    );
}

#[test]
fn dolby_true_hd_to_string() {
    assert_eq!(AudioCodec::DolbyTrueHD.to_string(), "dolby_true_hd");
}

#[test]
fn dolby_true_hd_from_str() {
    assert_eq!(
        AudioCodec::from_str("dolby_true_hd").unwrap(),
        AudioCodec::DolbyTrueHD
    );
}

#[test]
fn dts_to_string() {
    assert_eq!(AudioCodec::Dts.to_string(), "dts");
}

#[test]
fn dts_from_str() {
    assert_eq!(AudioCodec::from_str("dts").unwrap(), AudioCodec::Dts);
}

#[test]
fn dts_hd_to_string() {
    assert_eq!(AudioCodec::DtsHD.to_string(), "dts_hd");
}

#[test]
fn dts_hd_from_str() {
    assert_eq!(AudioCodec::from_str("dts_hd").unwrap(), AudioCodec::DtsHD);
}

#[test]
fn dts_x_to_string() {
    assert_eq!(AudioCodec::DtsX.to_string(), "dts_x");
}

#[test]
fn dts_x_from_str() {
    assert_eq!(AudioCodec::from_str("dts_x").unwrap(), AudioCodec::DtsX);
}

#[test]
fn flac_to_string() {
    assert_eq!(AudioCodec::Flac.to_string(), "flac");
}

#[test]
fn flac_from_str() {
    assert_eq!(AudioCodec::from_str("flac").unwrap(), AudioCodec::Flac);
}

#[test]
fn lpcm_to_string() {
    assert_eq!(AudioCodec::Lpcm.to_string(), "lpcm");
}

#[test]
fn lpcm_from_str() {
    assert_eq!(AudioCodec::from_str("lpcm").unwrap(), AudioCodec::Lpcm);
}

#[test]
fn mp3_to_string() {
    assert_eq!(AudioCodec::Mp3.to_string(), "mp3");
}

#[test]
fn mp3_from_str() {
    assert_eq!(AudioCodec::from_str("mp3").unwrap(), AudioCodec::Mp3);
}

#[test]
fn pcm_to_string() {
    assert_eq!(AudioCodec::Pcm.to_string(), "pcm");
}

#[test]
fn pcm_from_str() {
    assert_eq!(AudioCodec::from_str("pcm").unwrap(), AudioCodec::Pcm);
}

#[test]
fn vorbis_to_string() {
    assert_eq!(AudioCodec::Vorbis.to_string(), "vorbis");
}

#[test]
fn vorbis_from_str() {
    assert_eq!(AudioCodec::from_str("vorbis").unwrap(), AudioCodec::Vorbis);
}
