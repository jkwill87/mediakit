//! Verifies filename inspection of standalone-subtitle dispositions.

use super::*;
use crate::inspect::Inspector;

fn values(filename: &str, key: &str) -> Vec<String> {
    FilenameInspector::new(filename)
        .analyze()
        .tags()
        .into_iter()
        .filter(|tag| tag.key() == key)
        .map(Tag::value)
        .collect()
}
#[test]
fn tags_dispositions_without_exposing_track_numbers_or_creating_an_alternative_title() {
    let inspector = FilenameInspector::new("28.Weeks.Later.2007.en.2.forced.sdh.srt").analyze();
    assert_eq!(values(&inspector.filename, "subtitle_language"), ["en"]);
    assert_eq!(
        values(&inspector.filename, "subtitle_disposition"),
        ["forced", "sdh"]
    );
    assert!(
        inspector
            .tags()
            .into_iter()
            .all(|tag| !matches!(tag, Tag::AlternativeTitle(_)))
    );
}

#[test]
fn supports_compound_and_wrapped_subtitle_suffixes() {
    for filename in [
        "Wedding.Crashers.2005.pt-BR.forced.srt",
        "The.Departed.2006.en-forced.srt",
        "The.Fugitive.1993.[en][forced].srt",
    ] {
        let inspector = FilenameInspector::new(filename).analyze();
        assert_eq!(
            inspector
                .tags()
                .into_iter()
                .filter(|tag| tag.key() == "subtitle_disposition")
                .map(Tag::value)
                .collect::<Vec<_>>(),
            ["forced"],
            "{filename}"
        );
    }
}

#[test]
fn aggregates_distinct_and_repeated_subtitle_languages() {
    assert_eq!(values("Movie.eng.ita.srt", "subtitle_language"), ["multi"]);
    assert_eq!(values("Movie.eng.eng.srt", "subtitle_language"), ["multi"]);
}

#[test]
fn numeric_suffixes_are_excluded_from_identity_and_title_metadata() {
    let movie = FilenameInspector::new("Movie.en.2.srt").analyze();
    assert_eq!(movie.identity_stem(), Some("Movie"));
    assert!(movie
        .tags()
        .into_iter()
        .any(|tag| matches!(tag, Tag::Title(title) if title == "Movie")));
    assert!(
        movie
            .tags()
            .into_iter()
            .all(|tag| tag.key() != "subtitle_track_number")
    );

    let episode = FilenameInspector::new("Show.S01E01.Pilot.en.2.srt").analyze();
    assert!(episode.tags().into_iter().any(
        |tag| matches!(tag, Tag::EpisodeTitle(title) if title == "Pilot")
    ));
}
