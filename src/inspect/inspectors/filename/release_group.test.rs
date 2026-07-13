//! Verifies filename inspection of release groups.

use super::*;
use crate::inspect::Inspector;

fn _detect_release_group(filename: &str) -> Option<String> {
    let inspector = FilenameInspector::new(filename).analyze();
    inspector.tokens.iter().find_map(|t| match &t.tag {
        Some(Tag::ReleaseGroup(g)) => Some(g.clone()),
        _ => None,
    })
}

#[test]
fn tv_show_group() {
    let group = _detect_release_group("Peacemaker.S01E01.720p-LOL.mp4");
    assert!(group.is_some());
    assert!(group.unwrap().contains("LOL"));
}

#[test]
fn tv_show_multipart_group() {
    let group = _detect_release_group("Schitts.Creek.S01E01E02.720p-ARCHiViST.mp4");
    assert!(group.is_some());
    assert!(group.unwrap().contains("ARCHiViST"));
}

#[test]
fn tv_show_with_source_group() {
    let group = _detect_release_group("Curb.Your.Enthusiasm.S01E01.BluRay-FGT.mkv");
    assert!(group.is_some());
    assert!(group.unwrap().contains("FGT"));
}

#[test]
fn no_hyphen_no_group() {
    assert!(_detect_release_group("Rome.S01E01.720p.mkv").is_none());
}

#[test]
fn no_metadata_no_group() {
    assert!(_detect_release_group("The.Lost.Boys.1987.mp4").is_none());
}

#[test]
fn episode_range_not_group() {
    assert!(_detect_release_group("Misfits.S01E01-02.mp4").is_none());
}

#[test]
fn resolution_is_valid_group_marker() {
    assert_eq!(
        _detect_release_group("The.Warriors.1979.720p-ARCHiViST.mkv"),
        Some("ARCHiViST".to_string())
    );
}

#[test]
fn group_before_sample_marker() {
    assert_eq!(
        _detect_release_group("bobs.burgers.s01e04.1080p.ac3.rargb.sample.mkv"),
        Some("rargb".to_string())
    );
}

#[test]
fn leading_bracketed_group_is_excluded_from_title() {
    let inspector =
        FilenameInspector::new("[HorribleSubs] Garo - Vanishing Line - 01.mkv").analyze();
    assert!(
        inspector
            .tags()
            .iter()
            .any(|tag| matches!(tag, Tag::ReleaseGroup(value) if value == "HorribleSubs"))
    );
    assert!(
        inspector
            .tags()
            .iter()
            .any(|tag| matches!(tag, Tag::Title(value) if value == "Garo"))
    );
}
