//! Verifies filename inspection of audio codec-profile metadata.

use super::*;
use crate::inspect::Inspector;

fn profiles(filename: &str) -> Vec<AudioProfile> {
    FilenameInspector::new(filename)
        .analyze()
        .tokens
        .into_iter()
        .filter_map(|token| match token.tag {
            Some(Tag::AudioProfile(profile)) => Some(profile),
            _ => None,
        })
        .collect()
}

#[test]
fn dts_hd_master_audio() {
    assert_eq!(
        profiles("Natural.Born.Killers.1994.DTS-HD.MA.5.1.mkv"),
        vec![AudioProfile::MasterAudio]
    );
}

#[test]
fn aac_high_efficiency() {
    assert_eq!(
        profiles("The.Crow.2024.AAC-HE.5.1.mkv"),
        vec![AudioProfile::HighEfficiency]
    );
}

#[test]
fn profile_requires_compatible_codec() {
    assert!(profiles("The.Game.1997.H.264.MA.mkv").is_empty());
}
