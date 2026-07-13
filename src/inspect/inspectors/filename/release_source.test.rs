//! Verifies filename inspection of release sources.

use super::*;

fn _check_source(source: ReleaseSource, needle: &'static str, expected: &'static str) {
    const TITLES: &[&str] = &[
        "Final.Destination.2000",
        "Austin.Powers.International.Man.of.Mystery.1997",
        "The.Croods.A.New.Age.2020",
        "DodgeBall.A.True.Underdog.Story.2004",
        "Miss.Congeniality.2000",
        "Resident.Evil.Extinction.2007",
        "Napoleon.Dynamite.2004",
        "The.Hills.Have.Eyes.2006",
    ];
    let title = TITLES[needle.bytes().map(usize::from).sum::<usize>() % TITLES.len()];
    let path = format!("{title}.{needle}.mp4");
    let inspector = FilenameInspector::new(path).inspect_release_source();
    let token = inspector
        .tokens
        .into_iter()
        .find(|t| matches!(&t.tag, Some(Tag::ReleaseSource(s)) if *s == source))
        .expect("token not found");
    let start = title.len() + 1;
    let end = expected.len() + start;
    let excerpt = token.template(&inspector.filename);
    assert_eq!(excerpt, expected);
    assert_eq!(token.start, start);
    assert_eq!(token.end, end);
}

#[expect(
    unused_macro_rules,
    reason = "the shared test macro supports optional explicit expectations"
)]
macro_rules! ensure_source {
    ($fn_name:ident, $source:expr, $needle:expr) => {
        #[test]
        fn $fn_name() {
            _check_source($source, $needle, $needle);
        }
    };
    ($fn_name:ident, $source:expr, $needle:expr, $expected:expr) => {
        #[test]
        fn $fn_name() {
            _check_source($source, $needle, $expected);
        }
    };
}

macro_rules! reject_source {
    ($fn_name:ident, $source:expr, $needle:expr) => {
        #[test]
        #[should_panic]
        fn $fn_name() {
            _check_source($source, $needle, $needle);
        }
    };
}

// ReleaseSource::BluRay
ensure_source!(check_bluray1, ReleaseSource::BluRay, "bluray");
ensure_source!(check_bluray2, ReleaseSource::BluRay, "blu");
ensure_source!(check_bluray3, ReleaseSource::BluRay, "bd");
ensure_source!(check_bluray4, ReleaseSource::BluRay, "br");
ensure_source!(check_bluray5, ReleaseSource::BluRay, "bluray-rip");
ensure_source!(check_bluray6, ReleaseSource::BluRay, "bdrip");
ensure_source!(check_bluray7, ReleaseSource::BluRay, "bluray.rip");
reject_source!(check_bluray8, ReleaseSource::BluRay, "blurayx");

// ReleaseSource::Dvd
ensure_source!(check_dvd1, ReleaseSource::Dvd, "dvd");
ensure_source!(check_dvd2, ReleaseSource::Dvd, "dvdrip");
ensure_source!(check_dvd3, ReleaseSource::Dvd, "dvd-rip");
ensure_source!(check_dvd4, ReleaseSource::Dvd, "dvd.rip");
reject_source!(check_dvd5, ReleaseSource::Dvd, "dvdxx");

// ReleaseSource::HDtv
ensure_source!(check_hdtv1, ReleaseSource::HDtv, "hdtv");
ensure_source!(check_hdtv2, ReleaseSource::HDtv, "hdtvrip");
ensure_source!(check_hdtv3, ReleaseSource::HDtv, "hdtv-rip");
ensure_source!(check_hdtv4, ReleaseSource::HDtv, "hdtv.rip");

// ReleaseSource::Telecine
ensure_source!(check_telecine1, ReleaseSource::Telecine, "telecine");
ensure_source!(check_telecine2, ReleaseSource::Telecine, "tc");

// ReleaseSource::WebRip
ensure_source!(check_webrip1, ReleaseSource::WebRip, "webrip");
ensure_source!(check_webrip2, ReleaseSource::WebRip, "web-rip");
ensure_source!(check_webrip3, ReleaseSource::WebRip, "web.rip");

// ReleaseSource::WebDl
ensure_source!(check_webdl1, ReleaseSource::WebDl, "webdl");
ensure_source!(check_webdl2, ReleaseSource::WebDl, "web-dl");
ensure_source!(check_webdl3, ReleaseSource::WebDl, "web.dl");
ensure_source!(check_webdl4, ReleaseSource::WebDl, "web");
reject_source!(check_webdl5, ReleaseSource::WebDl, "webdlx");

#[test]
fn source_abbreviations_do_not_match_inside_title_words() {
    for title in ["My Bromance", "A Webstory", "TCM Classics"] {
        let inspector =
            FilenameInspector::new(format!("{title}.S01E01.mkv")).inspect_release_source();
        assert!(
            inspector
                .tokens
                .iter()
                .all(|token| !matches!(token.tag, Some(Tag::ReleaseSource(_)))),
            "release source detected inside {title:?}"
        );
    }
}
