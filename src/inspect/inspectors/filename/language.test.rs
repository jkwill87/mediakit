//! Verifies consolidated filename language inspection.

use super::*;
use crate::inspect::Inspector;

fn language_tags(filename: &str) -> Vec<(String, LanguageTag)> {
    let inspector = FilenameInspector::new(filename).analyze();
    inspector
        .tags()
        .into_iter()
        .filter_map(|tag| match tag {
            Tag::AudioLanguage(language) => Some(("audio".to_string(), *language)),
            Tag::SubtitleLanguage(language) => Some(("subtitle".to_string(), *language)),
            _ => None,
        })
        .collect()
}

fn language_code(language: LanguageTag) -> Option<&'static str> {
    match language {
        LanguageTag::Language(language) => Some(language.iso_639_1),
        LanguageTag::Multi => None,
    }
}

#[test]
fn audio_language_after_episode_ordering() {
    let tags = language_tags("Key.and.Peele.S01E02.eng.1080p.WEB-DL.x265-FLAME.mkv");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].0, "audio");
    assert_eq!(language_code(tags[0].1), Some("en"));
}

#[test]
fn multiple_audio_languages() {
    let tags = language_tags("Mr.Robot.S01E02.ita.eng.2160p.WEB.H265-FLAME.mkv");
    assert_eq!(tags, [("audio".to_owned(), LanguageTag::Multi)]);
}

#[test]
fn languages_before_the_technical_suffix_are_aggregated() {
    assert_eq!(
        language_tags("Movie.ita.eng.1080p.mkv"),
        [("audio".to_owned(), LanguageTag::Multi)]
    );
    assert_eq!(
        language_tags("Movie.ita.eng.fre.1080p.mkv"),
        [("audio".to_owned(), LanguageTag::Multi)]
    );
    assert_eq!(
        language_tags("Movie.eng.eng.1080p.mkv"),
        [("audio".to_owned(), LanguageTag::Multi)]
    );
}

#[test]
fn contiguous_languages_become_one_positioned_multi_token() {
    let inspector = FilenameInspector::new("Movie.ita.eng.1080p.mkv").analyze();
    let token_languages = inspector
        .tokens()
        .iter()
        .filter_map(|token| match token.tag {
            Some(Tag::AudioLanguage(language)) => {
                Some((token.template(inspector.filename()), language))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(token_languages, [("ita.eng", LanguageTag::Multi)]);

    let token_tags = inspector
        .tokens()
        .iter()
        .filter_map(|token| token.tag.as_ref())
        .map(|tag| (tag.key(), tag.value()))
        .collect::<Vec<_>>();
    let tags = inspector
        .tags()
        .into_iter()
        .map(|tag| (tag.key(), tag.value()))
        .collect::<Vec<_>>();
    assert_eq!(tags, token_tags);
}

#[test]
fn separated_language_candidates_retain_the_leftmost_language() {
    assert_eq!(
        language_tags("Movie.2024.eng.AAC.ita.1080p.mkv"),
        [(
            "audio".to_owned(),
            LanguageTag::Language(Language::from_identifier("eng").unwrap())
        )]
    );
}

#[test]
fn scene_multi_is_a_technical_marker_but_title_and_group_words_are_not() {
    let filename =
        "Star.Wars.The.Mandalorian.And.Grogu.2026.2160p.WEB-DL.DV.HDR10+.MULTi-Ben.The.Men.mkv";
    let inspected = FilenameInspector::new(filename).analyze();
    let actual = inspected
        .tags()
        .into_iter()
        .map(|tag| (tag.key(), tag.value()))
        .collect::<Vec<_>>();
    assert_eq!(
        actual,
        [
            ("title", "Star Wars The Mandalorian And Grogu".to_owned()),
            ("year", "2026".to_owned()),
            ("video_resolution", "4k".to_owned()),
            ("release_source", "webdl".to_owned()),
            ("video_dynamic_range", "dolby_vision".to_owned()),
            ("video_dynamic_range", "hdr10".to_owned()),
            ("audio_language", "multi".to_owned()),
            ("release_group", "Ben.The.Men".to_owned()),
            ("file_format", "mkv".to_owned()),
        ]
    );

    assert!(language_tags("Multi.2024.1080p.WEB-DL.H264-GROUP.mkv").is_empty());
    assert!(language_tags("English.2024.mkv").is_empty());
    assert!(language_tags("Movie.2024.1080p.WEB-DL.H264-Multi.mkv").is_empty());
    assert_eq!(
        language_tags("Movie.2024.eng.MULTi-GROUP.mkv"),
        [("audio".to_owned(), LanguageTag::Multi)]
    );
    assert_eq!(
        language_tags("Movie.2024.eng.AAC.MULTi-GROUP.mkv"),
        [("audio".to_owned(), LanguageTag::Multi)]
    );
}

#[test]
fn subtitle_language_before_extension() {
    let tags = language_tags("Sliders.S01E02.Fever.fr.srt");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].0, "subtitle");
    assert_eq!(language_code(tags[0].1), Some("fr"));
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
        assert_eq!(language_code(tags[0].1), Some(expected), "{filename}");
    }
}

#[test]
fn subtitle_language_blocks_use_the_same_normalization_rules() {
    assert_eq!(
        language_tags("Movie.ita.eng.fre.srt"),
        [("subtitle".to_owned(), LanguageTag::Multi)]
    );
    assert_eq!(
        language_tags("Movie.eng.eng.srt"),
        [("subtitle".to_owned(), LanguageTag::Multi)]
    );
    assert_eq!(
        language_tags("Movie.eng.multi.srt"),
        [("subtitle".to_owned(), LanguageTag::Multi)]
    );
    assert_eq!(
        language_tags("Movie.eng.forced.ita.srt"),
        [(
            "subtitle".to_owned(),
            LanguageTag::Language(Language::from_identifier("eng").unwrap())
        )]
    );
}

#[test]
fn analyze_contains_exactly_one_language_inspection_pass() {
    let pipeline = include_str!("mod.rs");
    assert_eq!(pipeline.matches(".inspect_language()").count(), 1);
    assert!(!pipeline.contains("inspect_audio_language"));
    assert!(!pipeline.contains("inspect_subtitle_suffix"));
}

#[test]
fn movie_subtitle_language_is_not_absorbed_into_title() {
    let filename = "Mean.Girls.2004.en.srt";
    let inspector = FilenameInspector::new(filename).analyze();
    let tags = language_tags(filename);
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].0, "subtitle");
    assert_eq!(language_code(tags[0].1), Some("en"));
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
