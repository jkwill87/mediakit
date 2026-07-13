//! Inspects filename tokens for video dynamic-range metadata.

use super::FilenameInspector;
use crate::meta::Tag;
use crate::meta::fields::VideoDynamicRange;

impl FilenameInspector {
    /// Selects the video dynamic range for a movie or television series.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_video_dynamic_range(self) -> Self {
        let start_idx = self
            .tokens
            .iter()
            .rposition(|token| match &token.tag {
                // video format never appears before ...
                Some(Tag::Title(_)) => true,
                Some(Tag::PremiereYear(_)) => true,
                Some(Tag::AirDate { .. }) => true,
                Some(Tag::SeasonNumber(_)) => true,
                Some(Tag::ReleaseSource(_)) => true,
                _ => false,
            })
            .unwrap_or(0);
        let mut tokens = self.tokens;
        for token in tokens.iter_mut().skip(start_idx) {
            let format = match token.template(&self.filename).to_lowercase().as_str() {
                "sdr" => VideoDynamicRange::SDR,
                "hdr" | "hdr10" | "10bit" => VideoDynamicRange::HDR10,
                "hdr12" => VideoDynamicRange::HDR12,
                "dolbyvision" | "dv" => VideoDynamicRange::DolbyVision,
                _ => continue,
            };
            token.tag = Some(Tag::VideoDynamicRange(format));
        }
        Self { tokens, ..self }
    }
}

crate::unit_tests!("video_dynamic_range.test.rs");
