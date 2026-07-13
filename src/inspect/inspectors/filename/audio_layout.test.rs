//! Verifies filename inspection of audio channel-layout metadata.

use super::*;

fn _check_layout(needle: &str, expected: &str, full: u8, sub: u8, height: u8) {
    const TITLES: &[&str] = &[
        "Kung.Fu.Panda.2008",
        "Monsters.University.2013",
        "Planes.Trains.and.Automobiles.1987",
        "The.Tigger.Movie.2000",
        "Analyze.This.1999",
        "Point.Break.1991",
        "Double.Jeopardy.1999",
        "Dark.City.1998",
    ];
    let title = TITLES[needle.bytes().map(usize::from).sum::<usize>() % TITLES.len()];
    let path = format!("{title}.{needle}.mp4");
    let inspector = FilenameInspector::new(path).inspect_audio_layout();
    let token = inspector
        .tokens
        .into_iter()
        .find(|t| matches!(&t.tag, Some(Tag::AudioLayout(_))))
        .expect("token not found");
    let start = title.len() + 1;
    let end = expected.len() + start;
    let excerpt = token.template(&inspector.filename);
    assert_eq!(excerpt, expected);
    assert_eq!(token.start, start);
    assert_eq!(token.end, end);
    if let Some(Tag::AudioLayout(layout)) = &token.tag {
        assert_eq!(layout.full, full, "full mismatch");
        assert_eq!(layout.sub, sub, "sub mismatch");
        assert_eq!(layout.height, height, "height mismatch");
    }
}

macro_rules! ensure_layout {
    ($fn_name:ident, $needle:expr, $full:expr, $sub:expr, $height:expr) => {
        #[test]
        fn $fn_name() {
            _check_layout($needle, $needle, $full, $sub, $height);
        }
    };
}

macro_rules! reject_layout {
    ($fn_name:ident, $needle:expr) => {
        #[test]
        #[should_panic]
        fn $fn_name() {
            _check_layout($needle, $needle, 0, 0, 0);
        }
    };
}

// Full format
ensure_layout!(check_5_1, "5.1", 5, 1, 0);
ensure_layout!(check_7_1, "7.1", 7, 1, 0);
ensure_layout!(check_5_1_2, "5.1.2", 5, 1, 2);
ensure_layout!(check_7_1_2, "7.1.2", 7, 1, 2);
ensure_layout!(check_7_1_4, "7.1.4", 7, 1, 4);

// Mono
ensure_layout!(check_mono1, "mono", 1, 0, 0);
ensure_layout!(check_mono2, "1ch", 1, 0, 0);

// Stereo
ensure_layout!(check_stereo1, "stereo", 2, 0, 0);
ensure_layout!(check_stereo2, "2ch", 2, 0, 0);

// 5.1 surround aliases
ensure_layout!(check_surround_5_1a, "5ch", 5, 1, 0);
ensure_layout!(check_surround_5_1b, "6ch", 5, 1, 0);
ensure_layout!(check_surround_5_1c, "5.1ch", 5, 1, 0);

// 7.1 surround aliases
ensure_layout!(check_surround_7_1a, "7ch", 7, 1, 0);
ensure_layout!(check_surround_7_1b, "8ch", 7, 1, 0);
ensure_layout!(check_surround_7_1c, "7.1ch", 7, 1, 0);

// Compound codec+layout (no delimiter between codec and layout)
#[test]
fn check_5_1_after_codec() {
    let path = "The.Lost.World.Jurassic.Park.1997.DDP5.1.mp4";
    let inspector = FilenameInspector::new(path).inspect_audio_layout();
    let token = inspector
        .tokens
        .into_iter()
        .find(|t| matches!(&t.tag, Some(Tag::AudioLayout(_))))
        .expect("token not found");
    assert_eq!(token.template(&inspector.filename), "5.1");
    assert_eq!(token.start, 37);
    assert_eq!(token.end, 40);
}

// Reject
reject_layout!(check_reject_3ch, "3ch");
reject_layout!(check_reject_monox, "monox");
