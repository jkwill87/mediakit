//! Verifies filename inspection of external-track suffix metadata.

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
fn tags_language_track_and_dispositions_without_creating_an_alternative_title() {
    let inspector = FilenameInspector::new("28.Weeks.Later.2007.en.2.forced.sdh.srt").analyze();
    assert_eq!(values(&inspector.filename, "subtitle_language"), ["en"]);
    assert_eq!(values(&inspector.filename, "subtitle_track"), ["2"]);
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
