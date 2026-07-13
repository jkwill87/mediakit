//! Verifies filename inspection of media file formats.

use super::*;
use crate::meta::Tag;

fn check_format(filename: &str, expected: &str) {
    let inspector = FilenameInspector::new(filename).inspect_file_format();
    let format = inspector
        .tokens
        .iter()
        .find_map(|t| match &t.tag {
            Some(Tag::FileFormat(format)) => Some(format.extension()),
            _ => None,
        })
        .expect("file format not found");
    assert_eq!(format, expected);
}

#[test]
fn check_mp4() {
    check_format("The.Fox.and.the.Hound.1981.mp4", "mp4");
}

#[test]
fn check_mkv() {
    check_format("Red.Rooms.2023.mkv", "mkv");
}

#[test]
fn check_avi() {
    check_format("No.Country.for.Old.Men.2007.avi", "avi");
}

#[test]
fn check_m4v() {
    check_format("Ex.Machina.2015.m4v", "m4v");
}

#[test]
fn check_uppercase() {
    check_format("Canadian.Bacon.1995.MKV", "mkv");
}

#[test]
fn check_mixed_case() {
    check_format("Jackpot.2024.Mp4", "mp4");
}

#[test]
fn check_complex_filename() {
    check_format("The.Day.the.Earth.Blew.Up.2024.720p.mp4", "mp4");
}

#[test]
fn check_four_char_ext() {
    check_format("Anora.2024.mpeg", "mpeg");
}

#[test]
fn check_two_char_ext() {
    check_format("Smile.2022.ts", "ts");
}

#[test]
fn check_subtitle_ext() {
    check_format("Millennium.S01E02.en.srt", "srt");
}

#[test]
#[should_panic]
fn reject_unknown_ext() {
    check_format("Heretic.2024.nope", "nope");
}

#[test]
#[should_panic]
fn reject_no_extension() {
    check_format("Fresh", "");
}
