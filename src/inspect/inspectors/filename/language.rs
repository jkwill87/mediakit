//! Inspects filename tokens for audio and subtitle language metadata.

use super::subtitle_suffix_parser::ParsedSubtitleSuffix;
use crate::inspect::{FilenameInspector, Token, TokenIdentity};
use crate::meta::Tag;
use crate::meta::fields::{Language, LanguageTag};
use crate::utils::validate_year;

#[derive(Clone, Copy)]
enum LanguageKind {
    Audio,
    Subtitle,
}

impl FilenameInspector {
    /// Selects and normalizes filename language markers in one pass.
    ///
    /// **Preconditions:**
    /// - Requires the file format plus structural and technical metadata to have been selected.
    pub(super) fn inspect_language(mut self) -> Self {
        if self
            .file_format()
            .is_some_and(|format| format.is_subtitle())
        {
            inspect_subtitle_languages(&mut self);
            normalize_languages(&mut self.tokens, LanguageKind::Subtitle);
        } else {
            inspect_audio_languages(&mut self);
            normalize_languages(&mut self.tokens, LanguageKind::Audio);
        }
        self
    }
}

fn inspect_audio_languages(inspector: &mut FilenameInspector) {
    let Some(marker_idx) = inspector.tokens.iter().position(|token| {
        matches!(
            token.tag,
            Some(Tag::AirDate(_))
                | Some(Tag::SeasonNumber(_))
                | Some(Tag::EpisodeNumber(_))
                | Some(Tag::PremiereYear(_))
                | Some(Tag::ReleaseSource(_))
                | Some(Tag::AudioCodec(_))
                | Some(Tag::AudioProfile(_))
                | Some(Tag::AudioLayout(_))
                | Some(Tag::VideoCodec(_))
                | Some(Tag::VideoProfile(_))
                | Some(Tag::VideoDynamicRange(_))
                | Some(Tag::VideoResolution(_))
        ) || (token.tag.is_none()
            && matches!(token.ident, TokenIdentity::Word)
            && token
                .template(&inspector.filename)
                .parse::<u16>()
                .is_ok_and(|year| validate_year(year).is_ok()))
    }) else {
        return;
    };

    // A language block can immediately precede an already-tagged technical marker. A raw year is
    // intentionally excluded so a title-leading language word is not inferred from the year alone.
    if inspector.tokens[marker_idx].tag.is_some() {
        for token in inspector.tokens[..marker_idx].iter_mut().rev() {
            if !matches!(token.ident, TokenIdentity::Word) {
                continue;
            }
            if token.tag.is_some() {
                break;
            }
            let Some(language) = Language::from_identifier(token.template(&inspector.filename))
            else {
                break;
            };
            token.tag = Some(Tag::AudioLanguage(LanguageTag::Language(language)));
        }
    }

    let format_idx = inspector
        .tokens
        .iter()
        .position(|token| matches!(token.tag, Some(Tag::FileFormat(_))))
        .unwrap_or(inspector.tokens.len());
    let release_group_idx = inspector.tokens[marker_idx + 1..format_idx]
        .iter()
        .rposition(|token| {
            token.tag.is_none()
                && matches!(token.ident, TokenIdentity::Delimiter)
                && token.template(&inspector.filename) == "-"
        })
        .map(|idx| marker_idx + 1 + idx);
    let scan_end = release_group_idx.unwrap_or(format_idx);

    for token in &mut inspector.tokens[marker_idx + 1..scan_end] {
        if token.tag.is_some() || !matches!(token.ident, TokenIdentity::Word) {
            continue;
        }
        let text = token.template(&inspector.filename);
        let language = if text.eq_ignore_ascii_case("multi") {
            Some(LanguageTag::Multi)
        } else {
            Language::from_identifier(text).map(LanguageTag::Language)
        };
        if let Some(language) = language {
            token.tag = Some(Tag::AudioLanguage(language));
        }
    }
}

fn inspect_subtitle_languages(inspector: &mut FilenameInspector) {
    let Some(parsed) = ParsedSubtitleSuffix::parse(&inspector.path) else {
        return;
    };
    let suffix_start = parsed.suffix_start();
    let languages = parsed.languages;
    let Some(container_idx) = inspector
        .tokens
        .iter()
        .position(|token| matches!(token.tag, Some(Tag::FileFormat(_))))
    else {
        return;
    };
    let suffix_end = inspector.tokens[container_idx].start;

    for language in languages {
        let range = match language {
            LanguageTag::Language(language) => language_range(
                &inspector.filename,
                &inspector.tokens,
                suffix_start,
                suffix_end,
                language,
            ),
            LanguageTag::Multi => inspector
                .tokens
                .iter()
                .enumerate()
                .find(|(_, token)| {
                    token.start >= suffix_start
                        && token.end <= suffix_end
                        && token.tag.is_none()
                        && matches!(token.ident, TokenIdentity::Word)
                        && token
                            .template(&inspector.filename)
                            .eq_ignore_ascii_case("multi")
                })
                .map(|(index, _)| (index, index)),
        };
        if let Some((start, end)) = range {
            inspector.tokens.splice(
                start..=end,
                [Token {
                    start: inspector.tokens[start].start,
                    end: inspector.tokens[end].end,
                    ident: TokenIdentity::Word,
                    tag: Some(Tag::SubtitleLanguage(language)),
                }],
            );
        }
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

fn normalize_languages(tokens: &mut Vec<Token>, kind: LanguageKind) {
    let candidates = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| language_tag(token, kind).map(|language| (index, language)))
        .collect::<Vec<_>>();
    if candidates.len() <= 1 {
        return;
    }

    if let Some(explicit_position) = candidates
        .iter()
        .position(|(_, language)| *language == LanguageTag::Multi)
    {
        let mut block_start = explicit_position;
        while block_start > 0
            && same_language_block(
                tokens,
                candidates[block_start - 1].0,
                candidates[block_start].0,
            )
        {
            block_start -= 1;
        }
        let mut block_end = explicit_position;
        while block_end + 1 < candidates.len()
            && same_language_block(tokens, candidates[block_end].0, candidates[block_end + 1].0)
        {
            block_end += 1;
        }
        let start = candidates[block_start].0;
        let end = candidates[block_end].0;
        clear_language_tags(tokens, kind);
        collapse_to_multi(tokens, start, end, kind);
        return;
    }

    if candidates
        .windows(2)
        .all(|pair| same_language_block(tokens, pair[0].0, pair[1].0))
    {
        let start = candidates[0].0;
        let end = candidates[candidates.len() - 1].0;
        clear_language_tags(tokens, kind);
        collapse_to_multi(tokens, start, end, kind);
        return;
    }

    for (index, _) in candidates.iter().skip(1) {
        tokens[*index].tag = None;
    }
}

fn same_language_block(tokens: &[Token], left: usize, right: usize) -> bool {
    tokens[left + 1..right]
        .iter()
        .all(|token| token.tag.is_none() && matches!(token.ident, TokenIdentity::Delimiter))
}

fn clear_language_tags(tokens: &mut [Token], kind: LanguageKind) {
    for token in tokens {
        if language_tag(token, kind).is_some() {
            token.tag = None;
        }
    }
}

fn collapse_to_multi(tokens: &mut Vec<Token>, start: usize, end: usize, kind: LanguageKind) {
    let tag = match kind {
        LanguageKind::Audio => Tag::AudioLanguage(LanguageTag::Multi),
        LanguageKind::Subtitle => Tag::SubtitleLanguage(LanguageTag::Multi),
    };
    if start == end {
        tokens[start].tag = Some(tag);
        return;
    }
    let token = Token {
        start: tokens[start].start,
        end: tokens[end].end,
        ident: TokenIdentity::Word,
        tag: Some(tag),
    };
    tokens.splice(start..=end, [token]);
}

const fn language_tag(token: &Token, kind: LanguageKind) -> Option<LanguageTag> {
    match (&token.tag, kind) {
        (Some(Tag::AudioLanguage(language)), LanguageKind::Audio)
        | (Some(Tag::SubtitleLanguage(language)), LanguageKind::Subtitle) => Some(*language),
        _ => None,
    }
}

crate::unit_tests!("language.test.rs");
