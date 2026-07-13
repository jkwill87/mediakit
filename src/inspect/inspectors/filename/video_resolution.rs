//! Inspects filename tokens for video resolution metadata.

use super::FilenameInspector;
use crate::meta::Tag;
use crate::meta::fields::VideoResolution;

impl FilenameInspector {
    /// Selects the video resolution for a movie or television series.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inpsect_video_resolution(self) -> Self {
        let mut tokens = self.tokens;
        for token in &mut tokens {
            let resolution = match token.template(&self.filename).to_lowercase().as_str() {
                "360i" => VideoResolution::Sd360i,
                "360p" => VideoResolution::Sd360p,
                "480i" => VideoResolution::Sd480i,
                "480p" => VideoResolution::Sd480P,
                "1080i" => VideoResolution::Hd1080i,
                "1080p" | "full-hd" | "fhd" => VideoResolution::Hd1080p,
                "720i" => VideoResolution::Hd720i,
                "720p" => VideoResolution::Hd720p,
                "2160p" | "uhd" | "uhd4k" | "4k" => VideoResolution::Uhd4k,
                "4320p" | "8k" => VideoResolution::Uhd8k,
                _ => continue,
            };
            token.tag = Some(Tag::VideoResolution(resolution));
        }
        Self { tokens, ..self }
    }
}

crate::unit_tests!("video_resolution.test.rs");
