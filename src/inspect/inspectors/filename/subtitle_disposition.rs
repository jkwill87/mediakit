//! Inspects filename tokens for standalone-subtitle disposition metadata.

use super::FilenameInspector;
use super::subtitle_suffix_parser::ParsedSubtitleSuffix;
use crate::inspect::TokenIdentity;
use crate::meta::Tag;
use crate::meta::fields::SubtitleDisposition;

impl FilenameInspector {
    /// Selects retained disposition suffixes on standalone subtitle files.
    ///
    /// **Preconditions:**
    /// - Requires the file format to have been previously selected.
    pub(super) fn inspect_subtitle_disposition(mut self) -> Self {
        let Some(parsed) = ParsedSubtitleSuffix::parse(&self.path) else {
            return self;
        };
        let suffix_start = parsed.suffix_start();
        let Some(container_idx) = self
            .tokens
            .iter()
            .position(|token| matches!(token.tag, Some(Tag::FileFormat(_))))
        else {
            return self;
        };
        let suffix_end = self.tokens[container_idx].start;

        for token in &mut self.tokens {
            if token.start < suffix_start
                || token.end > suffix_end
                || token.tag.is_some()
                || !matches!(token.ident, TokenIdentity::Word)
            {
                continue;
            }
            let disposition = match token.template(&self.filename).to_ascii_lowercase().as_str() {
                "forced" => Some(SubtitleDisposition::Forced),
                "sdh" => Some(SubtitleDisposition::Sdh),
                "commentary" => Some(SubtitleDisposition::Commentary),
                _ => None,
            };
            if let Some(disposition) = disposition
                && parsed.dispositions.contains(&disposition)
            {
                token.tag = Some(Tag::SubtitleDisposition(disposition));
            }
        }
        self
    }
}

crate::unit_tests!("subtitle_disposition.test.rs");
