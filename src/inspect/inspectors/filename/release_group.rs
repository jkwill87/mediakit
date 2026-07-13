//! Inspects filename tokens for release groups.

use super::FilenameInspector;
use crate::inspect::{Token, TokenIdentity};
use crate::meta::Tag;

impl FilenameInspector {
    /// Selects a release group enclosed in leading square brackets.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_leading_release_group(mut self) -> Self {
        let Some(group_end) = self
            .filename
            .strip_prefix('[')
            .and_then(|value| value.find(']'))
        else {
            return self;
        };
        let start = 1;
        let end = start + group_end;
        if start == end {
            return self;
        }
        let Some(start_token_idx) = self.tokens.iter().position(|token| token.end > start) else {
            return self;
        };
        let Some(end_token_idx) = self.tokens.iter().rposition(|token| token.start < end) else {
            return self;
        };
        let value = self.filename[start..end].to_owned();
        self.tokens.splice(
            start_token_idx..=end_token_idx,
            [Token {
                start,
                end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::ReleaseGroup(value)),
            }],
        );
        self
    }

    /// Selects the release group for a movie or television series.
    ///
    /// **Preconditions:**
    /// - Requires technical metadata to have been previously selected when the
    ///   group appears immediately before a `sample` marker.
    pub(super) fn inspect_release_group(mut self) -> Self {
        let start_token_idx = match self.tokens.iter().rev().skip(2).position(|token| {
            matches!(token.ident, TokenIdentity::Delimiter) && token.template(&self.filename) == "-"
        }) {
            Some(idx) => self.tokens.len() - idx - 2,
            None => {
                let Some(sample_idx) = self.tokens.iter().position(|token| {
                    token.tag.is_none()
                        && matches!(token.ident, TokenIdentity::Word)
                        && token
                            .template(&self.filename)
                            .eq_ignore_ascii_case("sample")
                }) else {
                    return self;
                };
                let Some(group_idx) = self.tokens[..sample_idx].iter().rposition(|token| {
                    token.tag.is_none() && matches!(token.ident, TokenIdentity::Word)
                }) else {
                    return self;
                };
                let has_technical_marker = self.tokens[..group_idx].iter().any(|token| {
                    matches!(
                        token.tag,
                        Some(Tag::ReleaseSource(_))
                            | Some(Tag::AudioCodec(_))
                            | Some(Tag::AudioProfile(_))
                            | Some(Tag::AudioLayout(_))
                            | Some(Tag::VideoCodec(_))
                            | Some(Tag::VideoProfile(_))
                            | Some(Tag::VideoDynamicRange(_))
                            | Some(Tag::VideoResolution(_))
                    )
                });
                if !has_technical_marker {
                    return self;
                }
                let value = self.tokens[group_idx].template(&self.filename).to_string();
                self.tokens[group_idx].tag = Some(Tag::ReleaseGroup(value));
                return self;
            }
        };
        let end_token_idx = self.tokens.len() - 2;
        if self.tokens[start_token_idx..end_token_idx]
            .iter()
            .any(|token| token.tag.is_some())
        {
            return self;
        }
        let start = self.tokens[start_token_idx].start;
        let end = self.tokens[end_token_idx].start;
        let release_group_text = self.filename[start..end].to_string();
        let release_group_token = Token {
            start,
            end,
            ident: TokenIdentity::Word,
            tag: Some(Tag::ReleaseGroup(release_group_text)),
        };
        self.tokens
            .splice(start_token_idx..end_token_idx, vec![release_group_token]);
        self
    }
}

crate::unit_tests!("release_group.test.rs");
