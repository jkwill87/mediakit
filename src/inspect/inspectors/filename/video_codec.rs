//! Inspects filename tokens for video codec metadata.

use super::{BOL, CASE_INSENSITIVE, EOL};
use crate::inspect::{FilenameInspector, Token, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::VideoCodec;
use crate::regexp::RegexVar;
use const_format::concatcp;
use std::str::FromStr;

impl FilenameInspector {
    /// Selects the video codec for a movie or television series.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_video_codec(self) -> Self {
        let mut re = RegexVar::new(concatcp!(
            CASE_INSENSITIVE,
            BOL,
            // AV1
            r"(?<av1>av[-.]?1)",
            // H262
            r"|(?<h262>h.?262|mpeg[-.]?2)",
            // H264
            r"|(?<h264>h.?264|x.?264|avc1?)",
            // H265
            r"|(?<h265>h.?265|x.?265|hevc)",
            // MPEG-4 Visual
            r"|(?<mpeg4_visual>mpeg[-.]?4|mp4v|xvid|divx)",
            // VC1
            r"|(?<vc1>vc.?1)",
            // VP8
            r"|(?<vp8>vp.?8)",
            // VP9
            r"|(?<vp9>vp.?9)",
            EOL,
        ));
        if !re.search(&self.filename) {
            return self;
        }

        let mut tokens = self.tokens;

        for regex_match in re.labeled_captures() {
            let value = regex_match.label.unwrap();
            let video_codec = VideoCodec::from_str(value).unwrap();
            let start = regex_match.start;
            let end = regex_match.end;
            let token = Token {
                start,
                end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::VideoCodec(video_codec)),
            };
            let start_token_idx = tokens.iter().position(|t| t.end > start).unwrap();
            let end_token_idx = tokens
                .iter()
                .rev()
                .position(|t| t.start < end)
                .map(|idx| tokens.len() - idx)
                .unwrap();
            tokens.splice(start_token_idx..end_token_idx, vec![token]);
        }
        Self { tokens, ..self }
    }
}

crate::unit_tests!("video_codec.test.rs");
