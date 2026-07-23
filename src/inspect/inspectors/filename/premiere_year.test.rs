//! Verifies filename inspection of premiere years.

use super::*;
use crate::inspect::Inspector;

fn _run_pipeline(filename: &str) -> FilenameInspector {
    FilenameInspector::new(filename)
        .inspect_file_format()
        .inspect_television_ordering()
        .inspect_title()
        .inspect_premiere_year()
}

fn _detect_year(filename: &str) -> Option<u16> {
    let inspector = _run_pipeline(filename);
    inspector.tokens.iter().find_map(|t| match &t.tag {
        Some(Tag::PremiereYear(y)) => Some(*y),
        _ => None,
    })
}

fn _detect_title(filename: &str) -> String {
    let inspector = _run_pipeline(filename);
    inspector
        .tokens
        .iter()
        .find_map(|t| match &t.tag {
            Some(Tag::Title(title)) => Some(title.clone()),
            _ => None,
        })
        .expect("title not found")
}

#[test]
fn year_extracted() {
    assert_eq!(_detect_year("Unhinged.2020.mp4"), Some(2020));
}

#[test]
fn year_absorbed_by_trailing_tokens() {
    assert_eq!(_detect_year("American.Psycho.2000.720p.mp4"), Some(2000));
}

#[test]
fn year_recent() {
    assert_eq!(
        _detect_year("One.Battle.After.Another.2025.mp4"),
        Some(2025)
    );
}

#[test]
fn year_lower_bound() {
    assert_eq!(_detect_year("Casablanca.1900.mp4"), Some(1900));
}

#[test]
fn year_upper_bound() {
    assert_eq!(_detect_year("Citizen.Kane.2099.mp4"), Some(2099));
}

#[test]
fn title_split_from_year() {
    assert_eq!(_detect_title("Black.Bear.2020.mp4"), "Black Bear");
}

#[test]
fn title_unchanged_with_trailing() {
    assert_eq!(_detect_title("Armageddon.1998.720p.mp4"), "Armageddon");
}

#[test]
fn no_year_present() {
    assert!(_detect_year("Flow.mp4").is_none());
}

#[test]
fn no_year_only_resolution() {
    assert!(_detect_year("Avatar.The.Way.of.Water.720p.mp4").is_none());
}

#[test]
fn year_below_min() {
    assert!(_detect_year("Mulan.1899.mp4").is_none());
}

#[test]
fn year_above_max() {
    assert!(_detect_year("Contact.2100.mp4").is_none());
}

#[test]
fn parenthesized_year() {
    assert_eq!(_detect_year("Possessor (2020).mp4"), Some(2020));
}

#[test]
fn edition_after_year_is_not_an_alternative_title() {
    let inspector = _run_pipeline("A Beautiful Mind 2001 Ultimate Extended Edition.mkv");
    assert!(
        inspector
            .tags()
            .iter()
            .all(|tag| !matches!(tag, Tag::AlternativeTitle(_)))
    );
}

#[test]
fn descriptive_text_after_year_remains_an_alternative_title() {
    let inspector = _run_pipeline("London.2012.Olympics.CTV.Preview.Show.HDTV.mkv");
    assert!(inspector.tags().iter().any(
        |tag| matches!(tag, Tag::AlternativeTitle(value) if value == "Olympics CTV Preview Show HDTV")
    ));
}
