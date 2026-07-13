//! Verifies bounded AVI container parsing.

use super::*;
use crate::meta::fields::{AudioCodec, VideoCodec};

#[test]
fn maps_avi_video_and_wave_audio_codecs() {
    assert_eq!(video_codec(b"XVID"), Some(VideoCodec::Mpeg4Visual));
    assert_eq!(wave_audio_codec(0x2000), Some(AudioCodec::DolbyDigital));
}
