//! Inspects filename tokens for alternative titles.

use super::FilenameInspector;
use crate::inspect::{Token, TokenIdentity};
use crate::meta::Tag;

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

fn title_segments(filename: &str, start: usize, end: usize) -> Vec<(usize, usize)> {
    let source = &filename[start..end];
    let mut separators = Vec::new();
    let mut characters = source.char_indices().peekable();

    while let Some((offset, character)) = characters.next() {
        if !matches!(character, '-' | '+' | '/' | '\\' | '|') {
            continue;
        }

        let separator_start = start + offset;
        let mut separator_end = separator_start + character.len_utf8();
        while let Some((next_offset, next)) = characters.peek().copied() {
            if !matches!(next, '-' | '+' | '/' | '\\' | '|') {
                break;
            }
            characters.next();
            separator_end = start + next_offset + next.len_utf8();
        }

        let single_embedded_hyphen = character == '-'
            && separator_end == separator_start + 1
            && filename[..separator_start]
                .chars()
                .next_back()
                .is_some_and(char::is_alphanumeric)
            && filename[separator_end..end]
                .chars()
                .next()
                .is_some_and(char::is_alphanumeric);
        if !single_embedded_hyphen {
            separators.push((separator_start, separator_end));
        }
    }

    let mut segments = Vec::new();
    let mut segment_start = start;
    for (separator_start, separator_end) in separators {
        if let Some(bounds) = trimmed_bounds(filename, segment_start, separator_start) {
            segments.push(bounds);
        }
        segment_start = separator_end;
    }
    if let Some(bounds) = trimmed_bounds(filename, segment_start, end) {
        segments.push(bounds);
    }
    for bounds in segments.iter_mut().skip(1) {
        *bounds = trim_alternative_qualifiers(filename, *bounds);
    }
    segments
}

fn trim_alternative_qualifiers(filename: &str, (start, mut end): (usize, usize)) -> (usize, usize) {
    let value = &filename[start..end];
    if let Some(parenthesis) = value.find('(')
        && looks_like_edition(&value[parenthesis + 1..])
        && let Some((_, trimmed_end)) = trimmed_bounds(filename, start, start + parenthesis)
    {
        end = trimmed_end;
    }

    (start, end)
}

pub(super) fn looks_like_edition(value: &str) -> bool {
    value
        .split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .any(|word| {
            [
                "alternative",
                "collector",
                "criterion",
                "director",
                "edition",
                "extended",
                "imax",
                "limited",
                "remastered",
                "special",
                "theatrical",
                "uncut",
                "unrated",
            ]
            .iter()
            .any(|marker| word.eq_ignore_ascii_case(marker))
        })
}

impl FilenameInspector {
    /// Splits a strongly delimited secondary title from the primary title.
    ///
    /// **Preconditions:**
    /// - Requires the title to have been previously selected.
    pub(super) fn inspect_alternative_title(self) -> Self {
        let Some(title_idx) = self
            .tokens
            .iter()
            .position(|token| matches!(token.tag, Some(Tag::Title(_))))
        else {
            return self;
        };
        let target = &self.tokens[title_idx];
        let segments = title_segments(&self.filename, target.start, target.end);
        if segments.len() < 2 {
            return self;
        }

        let mut replacement = Vec::with_capacity(segments.len() * 2);
        let mut cursor = target.start;
        for (index, (segment_start, segment_end)) in segments.into_iter().enumerate() {
            if cursor < segment_start {
                replacement.push(Token {
                    start: cursor,
                    end: segment_start,
                    ident: TokenIdentity::Delimiter,
                    tag: None,
                });
            }
            let value = normalized_text(&self.filename[segment_start..segment_end]);
            replacement.push(Token {
                start: segment_start,
                end: segment_end,
                ident: TokenIdentity::Word,
                tag: Some(if index == 0 {
                    Tag::Title(value)
                } else {
                    Tag::AlternativeTitle(value)
                }),
            });
            cursor = segment_end;
        }
        if cursor < target.end {
            replacement.push(Token {
                start: cursor,
                end: target.end,
                ident: TokenIdentity::Delimiter,
                tag: None,
            });
        }
        let mut tokens = self.tokens;
        tokens.splice(title_idx..=title_idx, replacement);
        Self { tokens, ..self }
    }
}

crate::unit_tests!("alternative_title.test.rs");
