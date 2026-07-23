//! Verifies shared stream defaults.

use super::*;

#[test]
fn stream_defaults_are_container_neutral() {
    let info = StreamInfo::default();
    assert!(info.is_enabled);
    assert!(!info.is_default);
    assert_eq!(info.language, None);

    assert_eq!(AudioStream::default().info, info);
    assert_eq!(VideoStream::default().info, info);
    assert_eq!(SubtitleStream::default().info, info);
    assert_eq!(SubtitleStream::default().codec, None);
}
