//! Verifies filename inspection of season and episode ordering.

use super::*;

fn tag_values(filename: &str) -> (Vec<u16>, Vec<u16>) {
    let inspector = FilenameInspector::new(filename).inspect_television_ordering();
    let mut seasons = Vec::new();
    let mut episodes = Vec::new();
    for tag in inspector.tags() {
        match tag {
            Tag::SeasonNumber(value) => seasons.push(*value),
            Tag::EpisodeNumber(value) => episodes.push(*value),
            _ => {}
        }
    }
    (seasons, episodes)
}

#[test]
fn separated_season_and_episode_marker() {
    assert_eq!(
        tag_values("Reservation Dogs S2 (Ep 6).mkv"),
        (vec![2], vec![6])
    );
}

#[test]
fn explicit_episode_marker_without_season() {
    assert_eq!(tag_values("From - Ep 23 - Exodus.mkv"), (vec![], vec![23]));
}

#[test]
fn strongly_delimited_bare_episode() {
    assert_eq!(
        tag_values("Fear.Factor.House.of.Fear.-.The.Final.Endgame.-.10.1080p.mkv"),
        (vec![], vec![10])
    );
}

#[test]
fn resolution_is_not_episode_ordering() {
    assert_eq!(tag_values("Flow.1280x720.mkv"), (vec![], vec![]));
}
use crate::inspect::Inspector;

fn episode_numbers(filename: &str) -> Vec<u16> {
    let inspector = FilenameInspector::new(filename).inspect_television_ordering();
    inspector
        .tokens
        .iter()
        .filter_map(|t| match &t.tag {
            Some(Tag::EpisodeNumber(n)) => Some(*n),
            _ => None,
        })
        .collect()
}

fn season_number(filename: &str) -> Option<u16> {
    let inspector = FilenameInspector::new(filename).inspect_television_ordering();
    inspector.tokens.iter().find_map(|t| match &t.tag {
        Some(Tag::SeasonNumber(n)) => Some(*n),
        _ => None,
    })
}

#[test]
fn single_s01e01() {
    let filename = "Shrinking.S01E01.Coin.Flip.mp4";
    assert_eq!(episode_numbers(filename), vec![1]);
    assert_eq!(season_number(filename), Some(1));
}

#[test]
fn single_01x01() {
    let filename = "cowboy.bebop.01x01.mp4";
    assert_eq!(episode_numbers(filename), vec![1]);
    assert_eq!(season_number(filename), Some(1));
}

#[test]
fn single_no_leading_zeros() {
    assert_eq!(episode_numbers("Avenue.5.S1E1.mp4"), vec![1]);
    assert_eq!(season_number("Avenue.5.S1E1.mp4"), Some(1));
}

#[test]
fn season_zero() {
    assert_eq!(episode_numbers("american.gods.s00e01.mp4"), vec![1]);
    assert_eq!(season_number("american.gods.s00e01.mp4"), Some(0));
}

#[test]
fn single_three_digit_episode() {
    assert_eq!(episode_numbers("Adventure.Time.S01E100.mp4"), vec![100]);
}

#[test]
fn single_ep_notation() {
    let filename = "Forensic.Files.II.S01EP03.mp4";
    assert_eq!(episode_numbers(filename), vec![3]);
    assert_eq!(season_number(filename), Some(1));
}

#[test]
fn multipart_consecutive_e() {
    assert_eq!(
        episode_numbers("Parks.and.Recreation.S07E01E02.mp4"),
        vec![1, 2]
    );
}

#[test]
fn multipart_three_consecutive_e() {
    assert_eq!(
        episode_numbers("Family.Guy.S20E01E02E03.mp4"),
        vec![1, 2, 3]
    );
}

#[test]
fn multipart_hyphen_e() {
    assert_eq!(episode_numbers("Shark.Tank.S16E01-E02.mp4"), vec![1, 2]);
}

#[test]
fn multipart_hyphen_e_range() {
    assert_eq!(episode_numbers("Birdgirl.S01E01-E03.mp4"), vec![1, 3]);
}

#[test]
fn multipart_hyphen_bare() {
    assert_eq!(episode_numbers("Poker.Face.S01E01-02.mp4"), vec![1, 2]);
}

#[test]
fn multipart_01x01_hyphen_bare() {
    assert_eq!(episode_numbers("clone.high.02x01-02.mp4"), vec![1, 2]);
}

#[test]
fn full_pipeline_no_release_group() {
    let inspector = FilenameInspector::new("the expanse s01e01-02.mp4").analyze();
    let episodes: Vec<u16> = inspector
        .tokens
        .iter()
        .filter_map(|t| match &t.tag {
            Some(Tag::EpisodeNumber(n)) => Some(*n),
            _ => None,
        })
        .collect();
    assert_eq!(episodes, vec![1, 2]);
    let has_release_group = inspector
        .tokens
        .iter()
        .any(|t| matches!(&t.tag, Some(Tag::ReleaseGroup(_))));
    assert!(!has_release_group);
}

#[test]
fn full_pipeline_multipart_with_release_group() {
    let inspector =
        FilenameInspector::new("Game.of.Thrones.S01E01E02.720p-ARCHiViST.mp4").analyze();
    let episodes: Vec<u16> = inspector
        .tokens
        .iter()
        .filter_map(|t| match &t.tag {
            Some(Tag::EpisodeNumber(n)) => Some(*n),
            _ => None,
        })
        .collect();
    assert_eq!(episodes, vec![1, 2]);
}
