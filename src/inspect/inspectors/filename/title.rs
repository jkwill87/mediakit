//! Inspects filename tokens for primary titles.

use super::FilenameInspector;
use crate::inspect::{Token, TokenIdentity};
use crate::meta::Tag;

impl FilenameInspector {
    /// Selects the title for a movie or television series.
    ///
    /// **Preconditions:**
    /// - Requires structural and technical metadata to have been previously selected.
    pub(super) fn inspect_title(self) -> Self {
        let mut tokens = self.tokens;
        let Some(range_start_idx) = tokens
            .iter()
            .position(|token| token.tag.is_none() && matches!(token.ident, TokenIdentity::Word))
        else {
            return Self { tokens, ..self };
        };
        let mut range_end_idx = tokens[range_start_idx..]
            .iter()
            .position(|token| token.tag.is_some())
            .map_or(tokens.len(), |offset| range_start_idx + offset);
        while range_end_idx > range_start_idx
            && matches!(tokens[range_end_idx - 1].ident, TokenIdentity::Delimiter)
        {
            range_end_idx -= 1;
        }
        if range_end_idx <= range_start_idx {
            return Self { tokens, ..self };
        }

        let title_text = tokens[range_start_idx..range_end_idx]
            .iter()
            .filter(|token| !matches!(token.ident, TokenIdentity::Delimiter))
            .map(|token| token.template(&self.filename))
            .collect::<Vec<&str>>()
            .join(" ");
        if !title_text.is_empty() {
            let title_token = Token {
                start: tokens[range_start_idx].start,
                end: tokens[range_end_idx - 1].end,
                ident: TokenIdentity::Word,
                tag: Some(Tag::Title(title_text)),
            };
            // remove all tokens before the marker and insert the title token
            tokens.splice(range_start_idx..range_end_idx, vec![title_token]);
        }
        Self { tokens, ..self }
    }
}

crate::unit_tests!("title.test.rs");
