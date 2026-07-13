//! Verifies filename inspection of video dynamic-range metadata.

use super::*;

fn _check_dynamic_range(range: VideoDynamicRange, needle: &'static str, expected: &'static str) {
    const TITLES: &[&str] = &[
        "Independence.Day.1996",
        "Who.Framed.Roger.Rabbit.1988",
        "Blackhat.2015",
        "Lord.of.War.2005",
        "True.Romance.1993",
        "Uncle.Buck.1989",
        "Cube.1998",
        "Mad.Max.Fury.Road.2015",
    ];
    let title = TITLES[needle.bytes().map(usize::from).sum::<usize>() % TITLES.len()];
    let path = format!("{title}.{needle}.mp4");
    let inspector = FilenameInspector::new(path).inspect_video_dynamic_range();
    let token = inspector
        .tokens
        .into_iter()
        .find(|t| matches!(&t.tag, Some(Tag::VideoDynamicRange(r)) if *r == range))
        .expect("token not found");
    let start = title.len() + 1;
    let end = expected.len() + start;
    let excerpt = token.template(&inspector.filename);
    assert_eq!(excerpt, expected);
    assert_eq!(token.start, start);
    assert_eq!(token.end, end);
}

macro_rules! ensure_dynamic_range {
    ($fn_name:ident, $range:expr, $needle:expr) => {
        #[test]
        fn $fn_name() {
            _check_dynamic_range($range, $needle, $needle);
        }
    };
}

macro_rules! reject_dynamic_range {
    ($fn_name:ident, $range:expr, $needle:expr) => {
        #[test]
        #[should_panic]
        fn $fn_name() {
            _check_dynamic_range($range, $needle, $needle);
        }
    };
}

// VideoDynamicRange::SDR
ensure_dynamic_range!(check_sdr1, VideoDynamicRange::SDR, "sdr");
ensure_dynamic_range!(check_sdr2, VideoDynamicRange::SDR, "SDR");

// VideoDynamicRange::HDR10
ensure_dynamic_range!(check_hdr10_1, VideoDynamicRange::HDR10, "hdr");
ensure_dynamic_range!(check_hdr10_2, VideoDynamicRange::HDR10, "hdr10");
ensure_dynamic_range!(check_hdr10_3, VideoDynamicRange::HDR10, "10bit");
ensure_dynamic_range!(check_hdr10_4, VideoDynamicRange::HDR10, "HDR");

// VideoDynamicRange::HDR12
ensure_dynamic_range!(check_hdr12_1, VideoDynamicRange::HDR12, "hdr12");

// VideoDynamicRange::DolbyVision
ensure_dynamic_range!(
    check_dolby_vision1,
    VideoDynamicRange::DolbyVision,
    "dolbyvision"
);
ensure_dynamic_range!(check_dolby_vision2, VideoDynamicRange::DolbyVision, "dv");

// Reject
reject_dynamic_range!(check_reject_hdr11, VideoDynamicRange::HDR10, "hdr11");
