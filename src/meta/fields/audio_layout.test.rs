//! Verifies audio-layout metadata behavior.

use super::*;

#[test]
fn to_string_stereo() {
    let layout = AudioLayout {
        full: 2,
        sub: 0,
        height: 0,
    };
    assert_eq!(layout.to_string(), "2.0");
}

#[test]
fn to_string_surround_5_1() {
    let layout = AudioLayout {
        full: 5,
        sub: 1,
        height: 0,
    };
    assert_eq!(layout.to_string(), "5.1");
}

#[test]
fn to_string_surround_7_1() {
    let layout = AudioLayout {
        full: 7,
        sub: 1,
        height: 0,
    };
    assert_eq!(layout.to_string(), "7.1");
}

#[test]
fn to_string_surround_5_1_2() {
    let layout = AudioLayout {
        full: 5,
        sub: 1,
        height: 2,
    };
    assert_eq!(layout.to_string(), "5.1.2");
}
