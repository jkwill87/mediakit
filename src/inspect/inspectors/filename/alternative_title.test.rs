//! Verifies filename inspection of alternative titles.

use super::*;
use crate::inspect::Inspector;

fn title_values(filename: &str) -> (Option<String>, Vec<String>) {
    let inspector = FilenameInspector::new(filename).analyze();
    let mut title = None;
    let mut alternatives = Vec::new();
    for tag in inspector.tags() {
        match tag {
            Tag::Title(value) => title = Some(value.clone()),
            Tag::AlternativeTitle(value) => alternatives.push(value.clone()),
            _ => {}
        }
    }
    (title, alternatives)
}

#[test]
fn episode_secondary_series_title() {
    let inspector = FilenameInspector::new(
        "Malcolm in the Middle - Life's Still Unfair S01EP03 (2000) (1080p).mp4",
    )
    .analyze();
    let tags = inspector.tags();
    assert!(
        tags.iter()
            .any(|tag| matches!(tag, Tag::Title(value) if value == "Malcolm in the Middle"))
    );
    assert!(
        tags.iter().any(
            |tag| matches!(tag, Tag::AlternativeTitle(value) if value == "Life s Still Unfair")
        )
    );
}

#[test]
fn splits_multiple_strongly_delimited_titles() {
    for (filename, title, alternatives) in [
        (
            "Barry - Chapter One - Make Your Mark S01E01 1080p HDTV x265-FLAME.mkv",
            "Barry",
            &["Chapter One", "Make Your Mark"][..],
        ),
        (
            "Taskmaster - The Final - Supping From The Fountain S20E10 1080p WEB-DL H264-ARCHiViST.mkv",
            "Taskmaster",
            &["The Final", "Supping From The Fountain"][..],
        ),
        (
            "Fear Factor - House of Fear - The Final Endgame S01E10 1080p WEB-DL H264-FLAME.mkv",
            "Fear Factor",
            &["House of Fear", "The Final Endgame"][..],
        ),
    ] {
        let (actual_title, actual_alternatives) = title_values(filename);
        assert_eq!(actual_title.as_deref(), Some(title), "{filename}");
        assert_eq!(actual_alternatives, alternatives, "{filename}");
    }
}

#[test]
fn separates_alternative_titles_from_edition_qualifiers() {
    for (filename, title, alternatives) in [
        (
            "Star Wars: Episode IV - A New Hope (2004) Special Edition.MKV",
            "Star Wars Episode IV",
            &["A New Hope"][..],
        ),
        (
            "Queen - A Kind of Magic (Alternative Extended Version) 2CD 2014",
            "Queen",
            &["A Kind of Magic"][..],
        ),
    ] {
        let (actual_title, actual_alternatives) = title_values(filename);
        assert_eq!(actual_title.as_deref(), Some(title), "{filename}");
        assert_eq!(actual_alternatives, alternatives, "{filename}");
    }
}

#[test]
fn supports_guessit_title_separator_forms() {
    for (filename, title, alternative) in [
        (
            "OSS_117--Cairo,_Nest_of_Spies.mkv",
            "OSS 117",
            "Cairo Nest of Spies",
        ),
        ("Dexter.+.Original.Sin.S01E01.mkv", "Dexter", "Original Sin"),
        (
            "Joe.Pera.Talks.with.You.|.Relaxing.Old.Footage.S01E01.mkv",
            "Joe Pera Talks with You",
            "Relaxing Old Footage",
        ),
    ] {
        let (actual_title, alternatives) = title_values(filename);
        assert_eq!(actual_title.as_deref(), Some(title), "{filename}");
        assert_eq!(alternatives, [alternative], "{filename}");
    }
}

#[test]
fn keeps_single_embedded_hyphens_in_the_primary_title() {
    assert_eq!(
        title_values("Spider-Man - No Way Home 2021.mkv"),
        (
            Some("Spider Man".to_owned()),
            vec!["No Way Home".to_owned()]
        )
    );
}

#[test]
fn handles_episode_alternatives_around_supported_ordering_forms() {
    for (filename, title, alternative, season, episode) in [
        (
            "Fear Factor House of Fear.-.The Final Endgame.-.10.(1280x720.HEVC.AAC)",
            "Fear Factor House of Fear",
            "The Final Endgame",
            None,
            10,
        ),
        (
            "[HorribleSubs] Garo - Vanishing Line - 01 [1080p].mkv",
            "Garo",
            "Vanishing Line",
            None,
            1,
        ),
        (
            "The Power of Suggestion - Mind Field S2 (Ep 6) 1440p.H264.mp4",
            "The Power of Suggestion",
            "Mind Field",
            Some(2),
            6,
        ),
        (
            "My Bromance 2 - 5 Years Later S01E01 1080p WEB-DL.mkv",
            "My Bromance 2",
            "5 Years Later",
            Some(1),
            1,
        ),
    ] {
        let inspector = FilenameInspector::new(filename).analyze();
        let tags = inspector.tags();
        assert!(
            tags.iter()
                .any(|tag| matches!(tag, Tag::Title(value) if value == title)),
            "{filename}: {tags:?}"
        );
        assert!(
            tags.iter()
                .any(|tag| matches!(tag, Tag::AlternativeTitle(value) if value == alternative)),
            "{filename}: {tags:?}"
        );
        assert!(
            tags.iter()
                .any(|tag| matches!(tag, Tag::EpisodeNumber(value) if *value == episode)),
            "{filename}: {tags:?}"
        );
        if let Some(season) = season {
            assert!(
                tags.iter()
                    .any(|tag| matches!(tag, Tag::SeasonNumber(value) if *value == season)),
                "{filename}: {tags:?}"
            );
        }
    }
}

#[test]
fn preserves_episode_title_after_explicit_episode_marker() {
    let inspector =
        FilenameInspector::new("Series/Kaamelott/Kaamelott - Livre V - Ep 23 - Le Forfait.avi")
            .analyze();
    let tags = inspector.tags();
    assert!(
        tags.iter()
            .any(|tag| matches!(tag, Tag::Title(value) if value == "Kaamelott"))
    );
    assert!(
        tags.iter()
            .any(|tag| matches!(tag, Tag::AlternativeTitle(value) if value == "Livre V"))
    );
    assert!(tags.iter().any(|tag| matches!(tag, Tag::EpisodeNumber(23))));
    assert!(
        tags.iter()
            .any(|tag| matches!(tag, Tag::EpisodeTitle(value) if value == "Le Forfait"))
    );
}
