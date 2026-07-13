//! Verifies filename inspection of video codec metadata.

use super::*;

fn _check_codec(codec: VideoCodec, needle: &'static str, expected: &'static str) {
    const TITLES: &[&str] = &[
        "Master.and.Commander.The.Far.Side.of.the.World.2003",
        "Jack.Jack.Attack.2005",
        "Man.on.the.Moon.1999",
        "The.Marvels.2023",
        "The.Nightmare.Before.Christmas.1993",
        "Brazil.1985",
        "Heavy.Metal.1981",
        "A.Beautiful.Mind.2001",
    ];
    let title = TITLES[needle.bytes().map(usize::from).sum::<usize>() % TITLES.len()];
    let path = format!("{title}.{needle}.mp4");
    let inspector = FilenameInspector::new(path).inspect_video_codec();
    let token = inspector
        .tokens
        .into_iter()
        .find(|t| matches!(&t.tag, Some(Tag::VideoCodec(c)) if *c == codec))
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
macro_rules! ensure_codec {
    ($fn_name:ident, $codec:expr, $needle:expr) => {
        #[test]
        fn $fn_name() {
            _check_codec($codec, $needle, $needle);
        }
    };
    ($fn_name:ident, $codec:expr, $needle:expr, $expected:expr) => {
        #[test]
        fn $fn_name() {
            _check_codec($codec, $needle, $expected);
        }
    };
}

macro_rules! reject_codec {
    ($fn_name:ident, $codec:expr, $needle:expr) => {
        #[test]
        #[should_panic]
        fn $fn_name() {
            _check_codec($codec, $needle, $needle);
        }
    };
}

// VideoCodec::H262
ensure_codec!(check_h262_1, VideoCodec::H262, "h262");
ensure_codec!(check_h262_2, VideoCodec::H262, "h.262");
ensure_codec!(check_h262_3, VideoCodec::H262, "mpeg2");
ensure_codec!(check_h262_4, VideoCodec::H262, "mpeg-2");
ensure_codec!(check_h262_5, VideoCodec::H262, "mpeg.2");

// VideoCodec::Av1
ensure_codec!(check_av1_1, VideoCodec::Av1, "av1");
ensure_codec!(check_av1_2, VideoCodec::Av1, "av-1");

// VideoCodec::H264
ensure_codec!(check_h264_1, VideoCodec::H264, "h264");
ensure_codec!(check_h264_2, VideoCodec::H264, "h.264");
ensure_codec!(check_h264_3, VideoCodec::H264, "x264");
ensure_codec!(check_h264_4, VideoCodec::H264, "x.264");
ensure_codec!(check_h264_5, VideoCodec::H264, "avc");
ensure_codec!(check_h264_6, VideoCodec::H264, "avc1");
reject_codec!(check_h264_7, VideoCodec::H264, "h264xx");

// VideoCodec::H265
ensure_codec!(check_h265_1, VideoCodec::H265, "h265");
ensure_codec!(check_h265_2, VideoCodec::H265, "h.265");
ensure_codec!(check_h265_3, VideoCodec::H265, "x265");
ensure_codec!(check_h265_4, VideoCodec::H265, "x.265");
ensure_codec!(check_h265_5, VideoCodec::H265, "hevc");
reject_codec!(check_h265_6, VideoCodec::H265, "h265xx");

// VideoCodec::Mpeg4Visual
ensure_codec!(check_mpeg4_1, VideoCodec::Mpeg4Visual, "mpeg4");
ensure_codec!(check_mpeg4_2, VideoCodec::Mpeg4Visual, "mp4v");
ensure_codec!(check_mpeg4_3, VideoCodec::Mpeg4Visual, "xvid");
ensure_codec!(check_mpeg4_4, VideoCodec::Mpeg4Visual, "divx");

// VideoCodec::Vc1
ensure_codec!(check_vc1_1, VideoCodec::Vc1, "vc1");
ensure_codec!(check_vc1_2, VideoCodec::Vc1, "vc.1");

// VideoCodec::Vp8
ensure_codec!(check_vp8_1, VideoCodec::Vp8, "vp8");
ensure_codec!(check_vp8_2, VideoCodec::Vp8, "vp.8");

// VideoCodec::Vp9
ensure_codec!(check_vp9_1, VideoCodec::Vp9, "vp9");
ensure_codec!(check_vp9_2, VideoCodec::Vp9, "vp.9");
