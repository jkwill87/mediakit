//! Verifies filename inspection of primary titles.

use super::*;

fn _detect_title(filename: &str) -> String {
    let inspector = FilenameInspector::new(filename)
        .inspect_file_format()
        .inspect_television_ordering()
        .inspect_title();
    inspector
        .tokens
        .iter()
        .find_map(|t| match &t.tag {
            Some(Tag::Title(title)) => Some(title.clone()),
            _ => None,
        })
        .expect("title not found")
}

#[test]
fn movie_dot_separated() {
    assert_eq!(_detect_title("The.Terminator.mp4"), "The Terminator");
}

#[test]
fn movie_multi_word() {
    assert_eq!(
        _detect_title("Harry.Potter.and.the.Order.of.the.Phoenix.mp4"),
        "Harry Potter and the Order of the Phoenix"
    );
}

#[test]
fn movie_single_word() {
    assert_eq!(_detect_title("Bloodsport.mp4"), "Bloodsport");
}

#[test]
fn tv_show_with_season() {
    assert_eq!(_detect_title("Foundation.S01E01.mp4"), "Foundation");
}

#[test]
fn tv_show_single_word() {
    assert_eq!(_detect_title("Columbo.S01E01.mp4"), "Columbo");
}

#[test]
fn tv_show_with_resolution() {
    assert_eq!(_detect_title("Farscape.S04E03E04.720p.mp4"), "Farscape");
}

#[test]
fn space_separated() {
    assert_eq!(
        _detect_title("There Will Be Blood.mp4"),
        "There Will Be Blood"
    );
}

#[test]
fn hyphen_separated() {
    assert_eq!(
        _detect_title("No-Country-for-Old-Men.mp4"),
        "No Country for Old Men"
    );
}
