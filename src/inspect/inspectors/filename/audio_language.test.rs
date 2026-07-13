//! Verifies filename inspection of audio language metadata.

use super::*;
use crate::inspect::Inspector;

fn language_tags(filename: &str) -> Vec<(String, Language)> {
    FilenameInspector::new(filename)
        .analyze()
        .tokens
        .into_iter()
        .filter_map(|token| match token.tag {
            Some(Tag::AudioLanguage(language)) => Some(("audio".to_string(), language)),
            Some(Tag::SubtitleLanguage(language)) => Some(("subtitle".to_string(), language)),
            _ => None,
        })
        .collect()
}

#[test]
fn audio_language_after_episode_ordering() {
    let tags = language_tags("Key.and.Peele.S01E02.eng.1080p.WEB-DL.x265-FLAME.mkv");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].0, "audio");
    assert_eq!(tags[0].1.iso_639_1, "en");
}

#[test]
fn multiple_audio_languages() {
    let tags = language_tags("Mr.Robot.S01E02.ita.eng.2160p.WEB.H265-FLAME.mkv");
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].1.iso_639_1, "it");
    assert_eq!(tags[1].1.iso_639_1, "en");
}

#[test]
fn subtitle_language_before_extension() {
    let tags = language_tags("Sliders.S01E02.Fever.fr.srt");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].0, "subtitle");
    assert_eq!(tags[0].1.iso_639_1, "fr");
}

#[test]
fn subtitle_language_accepts_regional_tags_and_release_affixes() {
    for (filename, expected) in [
        ("Spider-Man.Far.From.Home.2019.pt-BR.srt", "pt"),
        ("Rental.Family.2025.sub-eng.ass", "en"),
        ("Looper.2012.fre.vtt", "fr"),
        ("Eraserhead.1977.Dutch.ssa", "nl"),
    ] {
        let tags = language_tags(filename);
        assert_eq!(tags.len(), 1, "{filename}");
        assert_eq!(tags[0].0, "subtitle", "{filename}");
        assert_eq!(tags[0].1.iso_639_1, expected, "{filename}");
    }
}

#[test]
fn movie_subtitle_language_is_not_absorbed_into_title() {
    let filename = "Mean.Girls.2004.en.srt";
    let inspector = FilenameInspector::new(filename).analyze();
    let tags = language_tags(filename);
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].0, "subtitle");
    assert_eq!(tags[0].1.iso_639_1, "en");
    assert!(
        inspector
            .tags()
            .iter()
            .any(|tag| matches!(tag, Tag::Title(title) if title == "Mean Girls"))
    );
}

#[test]
fn ordinary_subtitle_title_is_not_a_language() {
    assert!(language_tags("Nancy.Drew.S01E01.WEBRip.x264-ION10.srt").is_empty());
}

#[test]
fn ambiguous_bare_codes_remain_titles_and_only_suffix_languages_are_tagged() {
    for filename in [
        "Ar.srt", "Da.srt", "De.srt", "El.srt", "He.srt", "Is.srt", "It.srt", "La.srt", "No.srt",
    ] {
        let inspector = FilenameInspector::new(filename).analyze();
        assert!(language_tags(filename).is_empty(), "{filename}");
        assert!(
            inspector
                .tags()
                .iter()
                .any(|tag| matches!(tag, Tag::Title(_))),
            "{filename}"
        );
    }

    let inspector = FilenameInspector::new("It.en.srt").analyze();
    assert_eq!(language_tags("It.en.srt").len(), 1);
    assert!(
        inspector
            .tags()
            .iter()
            .any(|tag| matches!(tag, Tag::Title(title) if title == "It"))
    );
}
