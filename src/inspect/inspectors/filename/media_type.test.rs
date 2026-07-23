//! Verifies filename inspection of media categories.

use super::*;
use crate::inspect::Inspector;

fn _detect_media_type(filename: &str) -> MediaType {
    FilenameInspector::new(filename)
        .inspect_television_ordering()
        .media_type()
}

#[test]
fn television_s01e01() {
    assert_eq!(
        _detect_media_type("Battlestar.Galactica.2003.S01E01.mp4"),
        MediaType::Television
    );
}

#[test]
fn television_01x01() {
    assert_eq!(
        _detect_media_type("Cyberpunk.Edgerunners.01x01.mp4"),
        MediaType::Television
    );
}

#[test]
fn television_s1e1() {
    assert_eq!(
        _detect_media_type("Firefly.S1E1.mp4"),
        MediaType::Television
    );
}

#[test]
fn television_multipart() {
    assert_eq!(
        _detect_media_type("Station.Eleven.S01E01E02.mp4"),
        MediaType::Television
    );
}

#[test]
fn movie_with_year() {
    assert_eq!(_detect_media_type("Alien.1979.mp4"), MediaType::Movie);
}

#[test]
fn movie_with_resolution() {
    assert_eq!(_detect_media_type("Inception.720p.mp4"), MediaType::Movie);
}

#[test]
fn movie_simple() {
    assert_eq!(_detect_media_type("Akira.mp4"), MediaType::Movie);
}

#[test]
fn explicit_movie_hint_overrides_episode_notation() {
    let media_type = FilenameInspector::new("The.Truman.Show.S01E01.mp4")
        .with_media_type_hint(MediaType::Movie)
        .analyze()
        .media_type();
    assert_eq!(media_type, MediaType::Movie);
}

#[test]
fn explicit_television_hint_overrides_automatic_movie() {
    let media_type = FilenameInspector::new("It.2017.mp4")
        .with_media_type_hint(MediaType::Television)
        .analyze()
        .media_type();
    assert_eq!(media_type, MediaType::Television);
}
