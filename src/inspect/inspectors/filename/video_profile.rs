//! Inspects filename tokens for video codec-profile metadata.

use super::FilenameInspector;
use crate::meta::Tag;
use crate::meta::fields::{VideoCodec, VideoProfile};

impl FilenameInspector {
    /// Selects video encoding profiles associated with a detected video codec.
    ///
    /// **Preconditions:**
    /// - Requires the video codec to have been previously selected.
    pub(super) fn inspect_video_profile(mut self) -> Self {
        let codec = self
            .tokens
            .iter()
            .find_map(|token| match token.tag.as_ref() {
                Some(Tag::VideoCodec(codec)) => Some(codec.clone()),
                _ => None,
            });
        let Some(codec) = codec else {
            return self;
        };

        for token in &mut self.tokens {
            if token.tag.is_some() {
                continue;
            }
            let identifier = token.template(&self.filename).to_ascii_lowercase();
            let profile = match (&codec, identifier.as_str()) {
                (VideoCodec::H264, "bp" | "baseline") => VideoProfile::Baseline,
                (VideoCodec::H264, "ep" | "xp" | "extended") => VideoProfile::Extended,
                (VideoCodec::H264 | VideoCodec::H265, "mp" | "main") => VideoProfile::Main,
                (VideoCodec::H265, "main10" | "main-10") => VideoProfile::Main10,
                (VideoCodec::H264 | VideoCodec::H265, "hp" | "hip" | "high") => VideoProfile::High,
                (VideoCodec::H264 | VideoCodec::H265, "hi10" | "hi10p" | "high10") => {
                    VideoProfile::High10
                }
                (VideoCodec::H264, "hi422p") => VideoProfile::High422,
                (VideoCodec::H264, "hi444pp") => VideoProfile::High444Predictive,
                (VideoCodec::H264, "sc" | "svc") => VideoProfile::ScalableVideoCoding,
                (VideoCodec::H264, "avchd") => VideoProfile::Avchd,
                _ => continue,
            };
            token.tag = Some(Tag::VideoProfile(profile));
        }
        self
    }
}

crate::unit_tests!("video_profile.test.rs");
