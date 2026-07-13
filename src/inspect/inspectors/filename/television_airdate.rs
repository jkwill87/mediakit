//! Inspects filename tokens for television air dates.

use super::FilenameInspector;
use crate::inspect::{Token, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::{AirDate, MediaType};

impl FilenameInspector {
    /// Selects the episode air date for a television series.
    ///
    /// Recognizes `YYYY-MM-DD`, `YYYY.MM.DD`, `YYYY/MM/DD`, and `YYYY MM DD` formatting.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_television_air_date(self) -> Self {
        const WINDOW_SIZE: usize = 5;
        // An explicit movie hint prevents date-shaped movie titles from being
        // interpreted as television air dates.
        if self.media_type_hint == Some(MediaType::Movie) {
            return self;
        }
        let mut tokens = self.tokens;
        for start_token_idx in 0..(tokens.len().saturating_sub(WINDOW_SIZE - 1)) {
            let end_token_idx = start_token_idx + WINDOW_SIZE;
            if end_token_idx > tokens.len() {
                break;
            }
            let start = tokens[start_token_idx].start;
            let end = tokens[end_token_idx - 1].end;
            let date_str = &self.filename[start..end];
            if let Ok(airdate) = AirDate::parse(date_str) {
                let tag = Some(Tag::AirDate(airdate));
                let ident = TokenIdentity::Word;
                let date_token = Token {
                    start,
                    end: start + date_str.len(),
                    ident,
                    tag,
                };
                tokens.splice(start_token_idx..end_token_idx, vec![date_token]);
            }
        }
        Self { tokens, ..self }
    }
}

crate::unit_tests!("television_airdate.test.rs");
