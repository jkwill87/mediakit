//! Verifies filename inspection of video codec-profile metadata.

use super::*;
use crate::inspect::Inspector;

fn profiles(filename: &str) -> Vec<VideoProfile> {
    FilenameInspector::new(filename)
        .analyze()
        .tokens
        .into_iter()
        .filter_map(|token| match token.tag {
            Some(Tag::VideoProfile(profile)) => Some(profile),
            _ => None,
        })
        .collect()
}

#[test]
fn h264_high() {
    assert_eq!(
        profiles("Five.Nights.at.Freddys.2023.H.264.High.mkv"),
        vec![VideoProfile::High]
    );
}

#[test]
fn h265_main_10() {
    assert_eq!(
        profiles("City.of.God.2002.HEVC.Main10.mkv"),
        vec![VideoProfile::Main10]
    );
}

#[test]
fn profile_requires_compatible_codec() {
    assert!(profiles("Fight.Club.1999.Main10.mkv").is_empty());
}
