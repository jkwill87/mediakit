//! Inspects filename tokens for episode titles.

use super::FilenameInspector;
use crate::inspect::{Token, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::MediaType;

impl FilenameInspector {
    /// Selects the episode title for a television series.
    ///
    /// **Preconditions:**
    /// - Requires the episode ordering to have been previously selected.
    /// - Requires the series title to have been previously selected.
    pub(super) fn inspect_episode_title(self) -> Self {
        if self.media_type() != MediaType::Television {
            return self;
        }
        let last_ep_idx = match self
            .tokens
            .iter()
            .rposition(|t| matches!(&t.tag, Some(Tag::EpisodeNumber(_))))
        {
            Some(idx) => idx,
            None => return self,
        };
        let range_start = last_ep_idx + 1;
        if range_start >= self.tokens.len() {
            return self;
        }
        let range_end = self.tokens[range_start..]
            .iter()
            .position(|t| t.tag.is_some())
            .map_or(self.tokens.len(), |offset| range_start + offset);
        // trim trailing delimiters
        let mut range_end = range_end;
        while range_end > range_start
            && matches!(self.tokens[range_end - 1].ident, TokenIdentity::Delimiter)
        {
            range_end -= 1;
        }
        if range_end <= range_start {
            return self;
        }
        let title_text = self.tokens[range_start..range_end]
            .iter()
            .filter(|t| !matches!(t.ident, TokenIdentity::Delimiter))
            .map(|t| t.template(&self.filename))
            .collect::<Vec<&str>>()
            .join(" ");
        if title_text.is_empty() {
            return self;
        }
        if title_text
            .split_whitespace()
            .next()
            .is_some_and(looks_like_technical_suffix)
        {
            return self;
        }
        let token = Token {
            start: self.tokens[range_start].start,
            end: self.tokens[range_end - 1].end,
            ident: TokenIdentity::Word,
            tag: Some(Tag::EpisodeTitle(title_text)),
        };
        let mut tokens = self.tokens;
        tokens.splice(range_start..range_end, vec![token]);
        Self { tokens, ..self }
    }
}

fn looks_like_technical_suffix(value: &str) -> bool {
    let value = value.to_ascii_lowercase();
    if let Some((width, height)) = value.split_once('x') {
        return width.len() >= 3
            && height.len() >= 3
            && width.chars().all(|character| character.is_ascii_digit())
            && height.chars().all(|character| character.is_ascii_digit());
    }
    ["fps", "kbps", "mbps", "p", "i"].iter().any(|suffix| {
        value.strip_suffix(suffix).is_some_and(|number| {
            number.len() >= 2 && number.chars().all(|character| character.is_ascii_digit())
        })
    })
}

crate::unit_tests!("episode_title.test.rs");
