//! Inspects filename tokens for external-track suffix metadata.

use super::FilenameInspector;
use super::track_suffix_parser::ParsedTrackSuffix;
use crate::inspect::{Token, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::{Language, TrackDisposition};

impl FilenameInspector {
    /// Tags language, track, and retained disposition suffixes on subtitle files.
    ///
    /// **Preconditions:**
    /// - Requires the file format to have been previously selected.
    pub(super) fn inspect_track_suffix(mut self) -> Self {
        let Some(parsed) = ParsedTrackSuffix::parse(&self.path) else {
            return self;
        };
        let suffix_start = parsed.base_stem_len();
        let identity_stem = parsed.identity_stem().map(str::to_owned);
        let generic_identity = parsed.is_generic();
        let track = parsed.metadata;
        self.metadata.format = Some(parsed.format);
        self.metadata
            .set_external_track(track.clone(), identity_stem, generic_identity);
        let Some(container_idx) = self
            .tokens
            .iter()
            .position(|token| matches!(token.tag, Some(Tag::FileFormat(_))))
        else {
            return self;
        };
        let suffix_end = self.tokens[container_idx].start;

        if let Some(language) = track.language
            && let Some((start, end)) = language_range(
                &self.filename,
                &self.tokens,
                suffix_start,
                suffix_end,
                language,
            )
        {
            self.tokens.splice(
                start..=end,
                [Token {
                    start: self.tokens[start].start,
                    end: self.tokens[end].end,
                    ident: TokenIdentity::Word,
                    tag: Some(Tag::SubtitleLanguage(language)),
                }],
            );
        }

        for token in &mut self.tokens {
            if token.start < suffix_start
                || token.end > suffix_end
                || token.tag.is_some()
                || !matches!(token.ident, TokenIdentity::Word)
            {
                continue;
            }
            let value = token.template(&self.filename);
            if let Ok(number) = value.parse::<u16>()
                && track.number == Some(number)
            {
                token.tag = Some(Tag::SubtitleTrack(number));
                continue;
            }
            let disposition = match value.to_ascii_lowercase().as_str() {
                "forced" => Some(TrackDisposition::Forced),
                "sdh" => Some(TrackDisposition::Sdh),
                "commentary" => Some(TrackDisposition::Commentary),
                _ => None,
            };
            if let Some(disposition) = disposition
                && track.dispositions.contains(&disposition)
            {
                token.tag = Some(Tag::SubtitleDisposition(disposition));
            }
        }
        self
    }
}

fn language_range(
    filename: &str,
    tokens: &[Token],
    suffix_start: usize,
    suffix_end: usize,
    expected: Language,
) -> Option<(usize, usize)> {
    let candidates = tokens
        .iter()
        .enumerate()
        .filter(|(_, token)| {
            token.start >= suffix_start
                && token.end <= suffix_end
                && matches!(token.ident, TokenIdentity::Word)
                && token.tag.is_none()
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    for word_count in (1..=candidates.len().min(6)).rev() {
        for range in candidates.windows(word_count) {
            let start = *range.first()?;
            let end = *range.last()?;
            let candidate = &filename[tokens[start].start..tokens[end].end];
            if candidate
                .split(|character: char| !character.is_alphanumeric())
                .any(|part| {
                    matches!(
                        part.to_ascii_lowercase().as_str(),
                        "forced" | "sdh" | "commentary"
                    )
                })
            {
                continue;
            }
            if Language::from_identifier(candidate) == Some(expected) {
                return Some((start, end));
            }
        }
    }
    None
}

crate::unit_tests!("track_suffix.test.rs");
