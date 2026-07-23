//! Verifies subtitle-codec metadata behavior.

use super::*;
use std::str::FromStr;

#[test]
fn canonical_values_round_trip() {
    let cases = [
        (SubtitleCodec::Arib, "arib"),
        (SubtitleCodec::Ass, "ass"),
        (SubtitleCodec::Bitmap, "bitmap"),
        (SubtitleCodec::Cea608, "cea_608"),
        (SubtitleCodec::Cea708, "cea_708"),
        (SubtitleCodec::Dvb, "dvb_subtitle"),
        (SubtitleCodec::HdmvText, "hdmv_text"),
        (SubtitleCodec::Kate, "kate"),
        (SubtitleCodec::Pgs, "pgs"),
        (SubtitleCodec::PlainText, "text"),
        (SubtitleCodec::Srt, "srt"),
        (SubtitleCodec::Ssa, "ssa"),
        (SubtitleCodec::SubtitleGraphics, "subtitle_graphics"),
        (SubtitleCodec::Teletext, "teletext"),
        (SubtitleCodec::TimedText, "timed_text"),
        (SubtitleCodec::Ttml, "ttml"),
        (SubtitleCodec::VobSub, "vobsub"),
        (SubtitleCodec::WebVtt, "webvtt"),
    ];

    for (codec, canonical) in cases {
        assert_eq!(codec.to_string(), canonical);
        assert_eq!(SubtitleCodec::from_str(canonical).unwrap(), codec);
    }
}

#[test]
fn unknown_value_is_rejected() {
    assert!(SubtitleCodec::from_str("unknown").is_err());
}
