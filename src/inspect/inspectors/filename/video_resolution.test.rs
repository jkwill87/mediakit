//! Verifies filename inspection of video resolution metadata.

use super::*;

fn _check_resolution(resolution: VideoResolution, needle: &'static str, expected: &'static str) {
    const TITLES: &[&str] = &[
        "Psycho.1960",
        "Hotel.Transylvania.2.2015",
        "Death.on.the.Nile.2022",
        "Memories.of.Murder.2003",
        "Suicide.Squad.2016",
        "Halloween.Kills.2021",
        "Oldboy.2003",
        "Candyman.2021",
    ];
    let title = TITLES[needle.bytes().map(usize::from).sum::<usize>() % TITLES.len()];
    let path = format!("{title}.{needle}.mp4");
    let inspector = FilenameInspector::new(path).inpsect_video_resolution();
    let token = inspector
        .tokens
        .into_iter()
        .find(|t| matches!(&t.tag, Some(Tag::VideoResolution(r)) if *r == resolution))
        .expect("token not found");
    let start = title.len() + 1;
    let end = expected.len() + start;
    let excerpt = token.template(&inspector.filename);
    assert_eq!(excerpt, expected);
    assert_eq!(token.start, start);
    assert_eq!(token.end, end);
}

macro_rules! ensure_resolution {
    ($fn_name:ident, $resolution:expr, $needle:expr) => {
        #[test]
        fn $fn_name() {
            _check_resolution($resolution, $needle, $needle);
        }
    };
}

macro_rules! reject_resolution {
    ($fn_name:ident, $resolution:expr, $needle:expr) => {
        #[test]
        #[should_panic]
        fn $fn_name() {
            _check_resolution($resolution, $needle, $needle);
        }
    };
}

// VideoResolution::Sd360i
ensure_resolution!(check_360i, VideoResolution::Sd360i, "360i");

// VideoResolution::Sd360p
ensure_resolution!(check_360p, VideoResolution::Sd360p, "360p");

// VideoResolution::Sd480i
ensure_resolution!(check_480i, VideoResolution::Sd480i, "480i");

// VideoResolution::Sd480P
ensure_resolution!(check_480p, VideoResolution::Sd480P, "480p");

// VideoResolution::Hd720i
ensure_resolution!(check_720i, VideoResolution::Hd720i, "720i");

// VideoResolution::Hd720p
ensure_resolution!(check_720p, VideoResolution::Hd720p, "720p");

// VideoResolution::Hd1080i
ensure_resolution!(check_1080i, VideoResolution::Hd1080i, "1080i");

// VideoResolution::Hd1080p
ensure_resolution!(check_1080p, VideoResolution::Hd1080p, "1080p");
ensure_resolution!(check_fhd, VideoResolution::Hd1080p, "fhd");

// VideoResolution::Uhd4k
ensure_resolution!(check_2160p, VideoResolution::Uhd4k, "2160p");
ensure_resolution!(check_4k, VideoResolution::Uhd4k, "4k");
ensure_resolution!(check_uhd, VideoResolution::Uhd4k, "uhd");
ensure_resolution!(check_uhd4k, VideoResolution::Uhd4k, "uhd4k");

// VideoResolution::Uhd8k
ensure_resolution!(check_4320p, VideoResolution::Uhd8k, "4320p");
ensure_resolution!(check_8k, VideoResolution::Uhd8k, "8k");

// Reject — tokenizer splits "full-hd" into ["full", "-", "hd"], neither matches
reject_resolution!(check_reject_full_hd, VideoResolution::Hd1080p, "full-hd");
reject_resolution!(check_reject_720, VideoResolution::Hd720p, "720");
reject_resolution!(check_reject_1080, VideoResolution::Hd1080p, "1080");
