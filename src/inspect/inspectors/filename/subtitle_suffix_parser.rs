//! Parses standalone-subtitle language and disposition suffixes.

use crate::meta::fields::{Language, LanguageTag, MediaFormat, SubtitleDisposition};
use std::ops::Range;
use std::path::Path;

/// External-track filename semantics used by the filename inspection pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ParsedSubtitleSuffix<'a> {
    pub(super) languages: Vec<LanguageTag>,
    pub(super) dispositions: Vec<SubtitleDisposition>,
    stem: Option<&'a str>,
    identity_range: Option<Range<usize>>,
    suffix_start: usize,
}

#[derive(Debug, Clone, Copy)]
enum Marker {
    Language(Language),
    Multi,
    Track,
    Disposition(SubtitleDisposition),
    Neutral,
}

impl<'a> ParsedSubtitleSuffix<'a> {
    pub(super) fn parse(path: &'a Path) -> Option<Self> {
        path.extension()
            .and_then(|extension| extension.to_str())
            .and_then(MediaFormat::from_extension)
            .filter(|format| format.is_subtitle())?;
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            return Some(Self {
                languages: Vec::new(),
                dispositions: Vec::new(),
                stem: None,
                identity_range: None,
                suffix_start: 0,
            });
        };

        let (mut boundary, mut markers) = peel_markers(stem);
        if boundary == 0 && is_ambiguous_bare_language_stem(stem, &markers) {
            boundary = stem.len();
            markers.clear();
        }
        let languages = markers
            .iter()
            .rev()
            .filter_map(|marker| match marker {
                Marker::Language(language) => Some(LanguageTag::Language(*language)),
                Marker::Multi => Some(LanguageTag::Multi),
                Marker::Track | Marker::Disposition(_) | Marker::Neutral => None,
            })
            .collect();
        let dispositions = markers
            .iter()
            .rev()
            .filter_map(|marker| match marker {
                Marker::Disposition(disposition) => Some(*disposition),
                Marker::Language(_) | Marker::Multi | Marker::Track | Marker::Neutral => None,
            })
            .fold(Vec::new(), |mut unique, disposition| {
                if !unique.contains(&disposition) {
                    unique.push(disposition);
                }
                unique
            });

        let base_prefix = stem[..boundary].trim_end_matches(is_suffix_separator);
        let base = base_prefix.trim();
        let base_start = base_prefix.len() - base_prefix.trim_start().len();
        let base_end = base_start + base.len();
        let generic = base.is_empty() || is_generic_label(base);
        let identity_range = (!generic).then_some(base_start..base_end);
        Some(Self {
            languages,
            dispositions,
            stem: Some(stem),
            identity_range,
            suffix_start: if generic { 0 } else { base_end },
        })
    }

    pub(super) fn identity_stem(&self) -> Option<&'a str> {
        self.stem?.get(self.identity_range.as_ref()?.clone())
    }

    pub(super) const fn suffix_start(&self) -> usize {
        self.suffix_start
    }
}

fn normalize_identity_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::to_lowercase)
        .filter(|character| character.is_alphanumeric())
        .collect()
}

fn marker_sequence(value: &str) -> Option<Vec<Marker>> {
    if let Some(marker) = numbered_qualifier(value) {
        return Some(vec![marker, Marker::Track]);
    }
    let whole = classify_path_marker(value);
    if let Some(marker @ (Marker::Disposition(_) | Marker::Neutral)) = whole {
        return Some(vec![marker]);
    }
    let split_markers = value
        .split(|character: char| !character.is_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(classify_path_marker)
        .collect::<Option<Vec<_>>>();
    if let Some(markers) = split_markers {
        if markers.len() > 1
            && markers
                .iter()
                .any(|marker| matches!(marker, Marker::Disposition(_) | Marker::Neutral))
        {
            return Some(markers);
        }
        return whole
            .map(|marker| vec![marker])
            .or_else(|| (markers.len() > 1).then_some(markers));
    }
    whole.map(|marker| vec![marker])
}

fn classify_path_marker(value: &str) -> Option<Marker> {
    classify_marker(trim_marker_wrappers(value)).or_else(|| {
        let value = trim_marker_wrappers(value);
        if is_track_index(value) {
            Some(Marker::Track)
        } else {
            None
        }
    })
}

fn is_ambiguous_bare_language_stem(stem: &str, markers: &[Marker]) -> bool {
    is_ambiguous_bare_language_code(&stem.to_ascii_lowercase())
        && matches!(markers, [Marker::Language(_)])
}

fn is_ambiguous_bare_language_code(value: &str) -> bool {
    matches!(
        value,
        "ar" | "da" | "de" | "el" | "he" | "is" | "it" | "la" | "no"
    )
}

fn peel_markers(stem: &str) -> (usize, Vec<Marker>) {
    let mut cursor = stem.len();
    let mut boundary = stem.len();
    let mut markers = Vec::new();
    loop {
        cursor = trim_suffix_separators(stem, cursor);
        if cursor == 0 {
            boundary = 0;
            break;
        }
        let start = stem[..cursor]
            .char_indices()
            .rev()
            .find_map(|(index, character)| {
                is_primary_separator(character).then_some(index + character.len_utf8())
            })
            .unwrap_or(0);
        let original_segment = &stem[start..cursor];
        let raw_segment = original_segment.trim();
        if let Some((base_len, wrapped)) = attached_wrapper_markers(raw_segment) {
            let mut accepted = true;
            for marker in wrapped.into_iter().rev() {
                let at_start = start == 0 && base_len == 0;
                let Some(marker) = reconcile_marker(marker, &markers, at_start) else {
                    accepted = false;
                    break;
                };
                markers.push(marker);
            }
            if accepted {
                boundary = start + base_len;
                cursor = boundary;
                continue;
            }
        }
        let segment = trim_marker_wrappers(raw_segment);
        let segment_offset = original_segment.find(segment).unwrap_or_default();
        if let Some(marker) = numbered_qualifier(segment) {
            markers.push(Marker::Track);
            markers.push(marker);
            boundary = start;
            cursor = start;
            continue;
        }
        if let Some((base, marker)) = hyphen_qualifier_suffix(segment)
            && let Some(marker) = reconcile_marker(marker, &markers, false)
        {
            markers.push(marker);
            boundary = start + segment_offset + base.len();
            cursor = boundary;
            continue;
        }
        if let Some((base, suffix)) = segment.rsplit_once('-')
            && !base.is_empty()
        {
            let suffix_marker = classify_suffix_marker(suffix);
            if matches!(suffix_marker, Some(Marker::Language(_)))
                && (matches!(
                    classify_suffix_marker(base),
                    Some(Marker::Disposition(_) | Marker::Neutral)
                ))
                && let Some(marker) =
                    suffix_marker.and_then(|marker| reconcile_marker(marker, &markers, false))
            {
                markers.push(marker);
                boundary = start + segment_offset + base.len();
                cursor = boundary;
                continue;
            }
            if let Some(marker) = classify_marker(segment) {
                let Some(marker) = reconcile_marker(marker, &markers, start == 0) else {
                    break;
                };
                markers.push(marker);
                boundary = start;
                cursor = start;
                continue;
            }
            if let Some(marker) =
                suffix_marker.and_then(|marker| reconcile_marker(marker, &markers, false))
            {
                markers.push(marker);
                boundary = start + segment_offset + base.len();
                cursor = boundary;
                continue;
            }
        }
        if let Some(marker) = classify_suffix_marker(segment) {
            let protect_leading_title =
                start == 0 && !is_explicit_leading_qualifier(segment, &markers);
            let Some(marker) = reconcile_marker(marker, &markers, protect_leading_title) else {
                break;
            };
            markers.push(marker);
            boundary = start;
            cursor = start;
            continue;
        }
        if is_track_index(segment)
            && (start > 0 || !markers.is_empty() || previous_suffix_is_language(stem, start))
        {
            markers.push(Marker::Track);
            boundary = start;
            cursor = start;
            continue;
        }
        if let Some((compound_start, marker)) = compound_suffix_marker(stem, start, cursor) {
            let Some(marker) = reconcile_marker(marker, &markers, compound_start == 0) else {
                break;
            };
            markers.push(marker);
            boundary = compound_start;
            cursor = compound_start;
            continue;
        }
        break;
    }
    (boundary, markers)
}

fn is_explicit_leading_qualifier(value: &str, markers: &[Marker]) -> bool {
    matches!(classify_suffix_marker(value), Some(Marker::Disposition(_)))
        && value.chars().any(|character| character.is_alphabetic())
        && value
            .chars()
            .filter(|character| character.is_alphabetic())
            .all(char::is_lowercase)
        && markers
            .iter()
            .any(|marker| matches!(marker, Marker::Language(_)))
}

fn reconcile_marker(marker: Marker, markers: &[Marker], at_start: bool) -> Option<Marker> {
    match marker {
        Marker::Language(language) => {
            if at_start
                && !markers.is_empty()
                && is_ambiguous_bare_language_code(language.iso_639_1)
            {
                return None;
            }
            Some(Marker::Language(language))
        }
        Marker::Disposition(_) if at_start => None,
        Marker::Multi if at_start => None,
        Marker::Neutral if at_start => None,
        Marker::Multi | Marker::Track | Marker::Disposition(_) | Marker::Neutral => Some(marker),
    }
}

fn hyphen_qualifier_suffix(value: &str) -> Option<(&str, Marker)> {
    for (index, _) in value.rmatch_indices('-') {
        let marker = classify_suffix_marker(&value[index + 1..]);
        if let Some(marker @ (Marker::Disposition(_) | Marker::Neutral)) = marker {
            return Some((&value[..index], marker));
        }
    }
    None
}

fn attached_wrapper_markers(value: &str) -> Option<(usize, Vec<Marker>)> {
    let closing = value.chars().next_back()?;
    let opening = match closing {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => return None,
    };
    let start = value.rfind(opening)?;
    let inner = &value[start + opening.len_utf8()..value.len() - closing.len_utf8()];
    let markers = marker_sequence(inner)?;
    Some((start, markers))
}

fn classify_suffix_marker(value: &str) -> Option<Marker> {
    classify_marker(trim_marker_wrappers(value))
}

fn is_track_index(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 3
        && value.chars().all(|character| character.is_ascii_digit())
}

fn numbered_qualifier(value: &str) -> Option<Marker> {
    let value = trim_marker_wrappers(value);
    let digit_start = value.char_indices().find(|(index, _)| {
        value[*index..]
            .chars()
            .all(|character| character.is_ascii_digit())
    })?;
    let (qualifier, track) = value.split_at(digit_start.0);
    if qualifier.is_empty() || !is_track_index(track) {
        return None;
    }
    let marker = classify_marker(qualifier)?;
    matches!(marker, Marker::Disposition(_) | Marker::Neutral).then_some(marker)
}

fn previous_suffix_is_language(stem: &str, start: usize) -> bool {
    let previous_end = trim_suffix_separators(stem, start);
    if previous_end == 0 {
        return false;
    }
    let previous_start = stem[..previous_end]
        .char_indices()
        .rev()
        .find_map(|(index, character)| {
            is_primary_separator(character).then_some(index + character.len_utf8())
        })
        .unwrap_or(0);
    matches!(
        classify_suffix_marker(&stem[previous_start..previous_end]),
        Some(Marker::Language(_) | Marker::Multi)
    )
}

fn compound_suffix_marker(stem: &str, start: usize, cursor: usize) -> Option<(usize, Marker)> {
    let previous_end = trim_suffix_separators(stem, start);
    if previous_end == 0 {
        return None;
    }
    let previous_start = stem[..previous_end]
        .char_indices()
        .rev()
        .find_map(|(index, character)| {
            is_primary_separator(character).then_some(index + character.len_utf8())
        })
        .unwrap_or(0);
    let marker = classify_marker(trim_marker_wrappers(&stem[previous_start..cursor]))?;
    matches!(marker, Marker::Disposition(_) | Marker::Neutral).then_some((previous_start, marker))
}

fn trim_marker_wrappers(value: &str) -> &str {
    value.trim_matches(|character: char| {
        character.is_whitespace() || matches!(character, '(' | ')' | '[' | ']' | '{' | '}')
    })
}

fn classify_marker(value: &str) -> Option<Marker> {
    let normalized = value.trim().to_ascii_lowercase().replace([' ', '_'], "-");
    let disposition = match normalized.as_str() {
        "forced" => Some(SubtitleDisposition::Forced),
        "sdh" => Some(SubtitleDisposition::Sdh),
        "commentary" => Some(SubtitleDisposition::Commentary),
        _ => None,
    };
    disposition
        .map(Marker::Disposition)
        .or_else(|| (normalized == "multi").then_some(Marker::Multi))
        .or_else(|| {
            matches!(
                normalized.as_str(),
                "sub"
                    | "subs"
                    | "subtitle"
                    | "subtitles"
                    | "fansub"
                    | "hardsub"
                    | "customsub"
                    | "utf8"
                    | "utf-8"
                    | "orig"
                    | "original"
                    | "full"
                    | "hearing-impaired"
                    | "hearingimpaired"
                    | "cc"
                    | "closed-caption"
                    | "closed-captions"
                    | "closedcaption"
                    | "default"
                    | "foreign"
                    | "sign"
                    | "signs"
                    | "song"
                    | "songs"
                    | "lyrics"
            )
            .then_some(Marker::Neutral)
        })
        .or_else(|| Language::from_identifier(&normalized).map(Marker::Language))
}

fn trim_suffix_separators(value: &str, mut end: usize) -> usize {
    while let Some(character) = value[..end].chars().next_back() {
        if !is_suffix_separator(character) {
            break;
        }
        end -= character.len_utf8();
    }
    end
}

const fn is_primary_separator(character: char) -> bool {
    matches!(character, '.' | '_' | ' ')
}

const fn is_suffix_separator(character: char) -> bool {
    is_primary_separator(character) || character == '-'
}

fn is_generic_label(value: &str) -> bool {
    matches!(
        normalize_identity_text(value).as_str(),
        "sub" | "subs" | "subtitle" | "subtitles" | "caption" | "captions"
    )
}

crate::unit_tests!("subtitle_suffix_parser.test.rs");
