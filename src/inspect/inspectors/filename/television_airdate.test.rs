//! Verifies filename inspection of television air dates.

use super::*;
use crate::inspect::Inspector;
use crate::meta::fields::AirDate;

fn _detect_airdate(filename: &str) -> Option<AirDate> {
    let inspector = FilenameInspector::new(filename).inspect_television_air_date();
    inspector.tokens.iter().find_map(|t| match &t.tag {
        Some(Tag::AirDate(d)) => Some(*d),
        _ => None,
    })
}

#[test]
fn dash_separated() {
    let airdate = _detect_airdate("Nine.Puzzles.2020-03-15.mp4").unwrap();
    assert_eq!(airdate.year(), 2020);
    assert_eq!(airdate.month(), 3);
    assert_eq!(airdate.day(), 15);
}

#[test]
fn dash_separated_end_of_year() {
    let airdate = _detect_airdate("Survivor.2021-12-01.mp4").unwrap();
    assert_eq!(airdate.year(), 2021);
    assert_eq!(airdate.month(), 12);
    assert_eq!(airdate.day(), 1);
}

#[test]
fn dash_separated_with_trailing_tokens() {
    let airdate = _detect_airdate("Chopped.2020-01-20.720p.mp4").unwrap();
    assert_eq!(airdate.year(), 2020);
    assert_eq!(airdate.month(), 1);
    assert_eq!(airdate.day(), 20);
}

#[test]
fn skipped_for_movie() {
    let inspector = FilenameInspector::new("Sesame.Street.2020-03-15.mp4")
        .with_media_type_hint(MediaType::Movie)
        .inspect_television_air_date();
    let has_airdate = inspector
        .tokens
        .iter()
        .any(|t| matches!(&t.tag, Some(Tag::AirDate(_))));
    assert!(!has_airdate);
}

#[test]
fn no_date_present() {
    assert!(_detect_airdate("Orphan.Black.Natural.Selection.mp4").is_none());
}

#[test]
fn dot_separated() {
    let airdate = _detect_airdate("Rookie.Blue.2020.03.15.mp4").unwrap();
    assert_eq!(airdate.year(), 2020);
    assert_eq!(airdate.month(), 3);
    assert_eq!(airdate.day(), 15);
}

#[test]
fn full_pipeline_detects_date_based_episode() {
    let inspector = FilenameInspector::new("the.colbert.show.2010.10.01.avi").analyze();
    assert_eq!(inspector.media_type(), MediaType::Television);
    assert!(
        inspector
            .tags()
            .iter()
            .any(|tag| matches!(tag, Tag::AirDate(_)))
    );
}
