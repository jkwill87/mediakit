//! Verifies filename inspection of episode titles.

use super::*;
use crate::inspect::Inspector;

fn _detect_episode_title(filename: &str) -> Option<String> {
    let inspector = FilenameInspector::new(filename).analyze();
    inspector.tokens.iter().find_map(|t| match &t.tag {
        Some(Tag::EpisodeTitle(title)) => Some(title.clone()),
        _ => None,
    })
}

#[test]
fn single_word() {
    assert_eq!(
        _detect_episode_title("Breaking.Bad.S05E16.Felina.720p.BluRay.x264-DEMAND.mkv"),
        Some("Felina".to_string()),
    );
}

#[test]
fn multi_word() {
    assert_eq!(
        _detect_episode_title(
            "Aqua.Teen.Hunger.Force.S01E01.Rabbot.1080p.BluRay.x264-ARCHiViST.mkv"
        ),
        Some("Rabbot".to_string()),
    );
}

#[test]
fn with_part_number() {
    assert_eq!(
        _detect_episode_title(
            "Fear.Factor.House.of.Fear.S01E11.48.Hours.of.Fear.Part.1.1080p.WEB.H264-FLAME.mkv"
        ),
        Some("48 Hours of Fear Part 1".to_string()),
    );
}

#[test]
fn no_episode_title() {
    assert_eq!(
        _detect_episode_title("The.Office.S01E01.HDTV.x264-LOL.mkv"),
        None,
    );
}

#[test]
fn multi_episode_no_title() {
    assert_eq!(
        _detect_episode_title("Smiling.Friends.S01E01E02.720p-ARCHiViST.mp4"),
        None,
    );
}

#[test]
fn multi_episode_with_title() {
    assert_eq!(
        _detect_episode_title("Snowpiercer.S01E01E02.First.the.Weather.Changed.720p-ARCHiViST.mp4"),
        Some("First the Weather Changed".to_string()),
    );
}

#[test]
fn movie_skipped() {
    assert_eq!(
        _detect_episode_title("Bambi.1942.1080p.BluRay.x264-ARCHiViST.mkv"),
        None,
    );
}

#[test]
fn parenthesized_technical_suffix_is_not_an_episode_title() {
    for filename in [
        "Euphoria - 02 (1280x720 HEVC AAC).mkv",
        "Fallout S2 (Ep 6) (1440p 24fps H264).mp4",
    ] {
        assert_eq!(_detect_episode_title(filename), None, "{filename}");
    }
}
