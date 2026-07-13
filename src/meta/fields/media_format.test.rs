//! Verifies media-format metadata behavior.

use super::*;

#[test]
fn recognized_extensions_round_trip_to_their_format() {
    for format in MediaFormat::ALL {
        assert_eq!(
            MediaFormat::from_extension(format.extension()),
            Some(format)
        );
        assert!(!format.mime_type().is_empty());
    }
    assert_eq!(MediaFormat::from_extension(".SRT"), Some(MediaFormat::Srt));
    assert_eq!(MediaFormat::from_extension("mpg"), Some(MediaFormat::Mpeg));
}

#[test]
fn subtitle_formats_are_classified_by_content() {
    let subtitles = MediaFormat::ALL
        .into_iter()
        .filter(|format| format.is_subtitle())
        .map(MediaFormat::extension)
        .collect::<Vec<_>>();
    assert_eq!(subtitles, ["ass", "idx", "srt", "ssa", "sub", "vtt"]);
}
