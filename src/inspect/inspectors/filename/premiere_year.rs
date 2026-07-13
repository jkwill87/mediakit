//! Inspects filename tokens for premiere years.

use super::alternative_title::looks_like_edition;
use crate::inspect::{FilenameInspector, Token, TokenIdentity};
use crate::meta::Tag;
use crate::utils::validate_year;

fn normalized_text(value: &str) -> String {
    value
        .split(|character: char| !character.is_alphanumeric())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn trimmed_bounds(filename: &str, start: usize, end: usize) -> Option<(usize, usize)> {
    let value = &filename[start..end];
    let leading = value
        .char_indices()
        .find(|(_, character)| character.is_alphanumeric())?
        .0;
    let trailing = value
        .char_indices()
        .rev()
        .find(|(_, character)| character.is_alphanumeric())
        .map(|(idx, character)| idx + character.len_utf8())?;
    Some((start + leading, start + trailing))
}

fn last_year(filename: &str, start: usize, end: usize) -> Option<(usize, usize, u16)> {
    let bytes = filename.as_bytes();
    (start..end.saturating_sub(3)).rev().find_map(|year_start| {
        let year_end = year_start + 4;
        if year_end > end
            || !bytes[year_start..year_end].iter().all(u8::is_ascii_digit)
            || year_start
                .checked_sub(1)
                .is_some_and(|idx| bytes[idx].is_ascii_digit())
            || (year_end < bytes.len() && bytes[year_end].is_ascii_digit())
        {
            return None;
        }
        let year = filename[year_start..year_end].parse::<u16>().ok()?;
        validate_year(year).ok()?;
        trimmed_bounds(filename, start, year_start).map(|_| (year_start, year_end, year))
    })
}

fn push_delimiter(tokens: &mut Vec<Token>, start: usize, end: usize) {
    if start < end {
        tokens.push(Token {
            start,
            end,
            ident: TokenIdentity::Delimiter,
            tag: None,
        });
    }
}

impl FilenameInspector {
    /// Selects a standalone premiere year from the parsed title span.
    ///
    /// Text following the year is retained as an alternate title, which covers
    /// common edition and secondary-title filename forms without discarding its
    /// source span.
    ///
    /// **Preconditions:**
    /// - Requires the title to have been previously selected.
    pub(super) fn inspect_premiere_year(self) -> Self {
        let Some(title_idx) = self
            .tokens
            .iter()
            .position(|token| matches!(token.tag, Some(Tag::Title(_))))
        else {
            return self;
        };
        let target_start = self.tokens[title_idx].start;
        let target_end = self.tokens[title_idx].end;
        let Some((year_start, year_end, year)) =
            last_year(&self.filename, target_start, target_end)
        else {
            let mut tokens = self.tokens;
            if let Some(token) = tokens.iter_mut().find(|token| {
                token.tag.is_none()
                    && token
                        .template(&self.filename)
                        .parse::<u16>()
                        .is_ok_and(|year| validate_year(year).is_ok())
            }) {
                let year = token
                    .template(&self.filename)
                    .parse::<u16>()
                    .expect("year was validated above");
                token.tag = Some(Tag::PremiereYear(year));
            }
            return Self { tokens, ..self };
        };
        let Some((title_start, title_end)) =
            trimmed_bounds(&self.filename, target_start, year_start)
        else {
            return self;
        };

        let mut replacement = Vec::with_capacity(6);
        push_delimiter(&mut replacement, target_start, title_start);
        replacement.push(Token {
            start: title_start,
            end: title_end,
            ident: TokenIdentity::Word,
            tag: Some(Tag::Title(normalized_text(
                &self.filename[title_start..title_end],
            ))),
        });
        push_delimiter(&mut replacement, title_end, year_start);
        replacement.push(Token {
            start: year_start,
            end: year_end,
            ident: TokenIdentity::Word,
            tag: Some(Tag::PremiereYear(year)),
        });

        if let Some((alternative_start, alternative_end)) =
            trimmed_bounds(&self.filename, year_end, target_end)
        {
            push_delimiter(&mut replacement, year_end, alternative_start);
            replacement.push(Token {
                start: alternative_start,
                end: alternative_end,
                ident: TokenIdentity::Word,
                tag: (!looks_like_edition(&self.filename[alternative_start..alternative_end]))
                    .then(|| {
                        Tag::AlternativeTitle(normalized_text(
                            &self.filename[alternative_start..alternative_end],
                        ))
                    }),
            });
            push_delimiter(&mut replacement, alternative_end, target_end);
        } else {
            push_delimiter(&mut replacement, year_end, target_end);
        }

        let mut tokens = self.tokens;
        tokens.splice(title_idx..=title_idx, replacement);
        Self { tokens, ..self }
    }
}

crate::unit_tests!("premiere_year.test.rs");
