//! Verifies probe-owned track defaults and common access.

use super::*;

#[test]
fn track_defaults_are_container_neutral() {
    let info = TrackInfo::default();
    assert!(info.is_enabled);
    assert!(!info.is_default);
    assert_eq!(info.language, None);

    assert_eq!(AudioTrack::default().info, info);
    assert_eq!(VideoTrack::default().info, info);
    assert_eq!(SubtitleTrack::default().info, info);
    assert_eq!(SubtitleTrack::default().codec, None);
    assert_eq!(Track::Audio(AudioTrack::default()).info(), &info);
}
