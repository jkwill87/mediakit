//! Verifies filename inspection of audio codec metadata.

use super::*;

fn _check_codec(codec: AudioCodec, needle: &'static str, expected: &'static str) {
    const TITLES: &[&str] = &[
        "Anastasia.1997",
        "Migration.2023",
        "Space.Jam.1996",
        "Garden.State.2004",
        "Sunshine.2007",
        "Forrest.Gump.1994",
        "Clueless.1995",
        "Training.Day.2001",
    ];
    let title = TITLES[needle.bytes().map(usize::from).sum::<usize>() % TITLES.len()];
    let path = format!("{title}.{needle}.mp4");
    let inspector = FilenameInspector::new(path).inspect_audio_codec();
    let token = inspector
        .tokens
        .into_iter()
        .find(|t| matches!(&t.tag, Some(Tag::AudioCodec(c)) if *c == codec))
        .expect("token not found");
    let start = title.len() + 1;
    let end = expected.len() + start;
    let excerpt = token.template(&inspector.filename);
    assert_eq!(excerpt, expected);
    assert_eq!(token.start, start);
    assert_eq!(token.end, end);
}

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

// AudioCodec::Aac
ensure_codec!(check_aac1, AudioCodec::Aac, "aac");
ensure_codec!(check_aac2, AudioCodec::Aac, "aac he");
ensure_codec!(check_aac3, AudioCodec::Aac, "aac.he");
ensure_codec!(check_aac4, AudioCodec::Aac, "aac:he");
ensure_codec!(check_aac5, AudioCodec::Aac, "aac-he");
ensure_codec!(check_aac7, AudioCodec::Aac, "aac lc");
ensure_codec!(check_aac8, AudioCodec::Aac, "aac.lc");
ensure_codec!(check_aac9, AudioCodec::Aac, "aac:lc");
ensure_codec!(check_aac10, AudioCodec::Aac, "aac-lc");
reject_codec!(check_aac11, AudioCodec::Aac, "aacxx");

// AudioCodec::Alac and AudioCodec::Opus
ensure_codec!(check_alac1, AudioCodec::Alac, "alac");
ensure_codec!(check_opus1, AudioCodec::Opus, "opus");

// AudioCodec::DolbyAtmos
ensure_codec!(check_dolby_atmos1, AudioCodec::DolbyAtmos, "atmos");
ensure_codec!(check_dolby_atmos2, AudioCodec::DolbyAtmos, "dolby atmos");
ensure_codec!(check_dolby_atmos3, AudioCodec::DolbyAtmos, "dolby.atmos");
ensure_codec!(check_dolby_atmos4, AudioCodec::DolbyAtmos, "dolby-atmos");
reject_codec!(check_dolby_atmos5, AudioCodec::DolbyAtmos, "dolby-atmosss");

// AudioCodec::DolbyDigitalPlus
ensure_codec!(
    check_dolby_digital_plus1,
    AudioCodec::DolbyDigitalPlus,
    "dolby-digital-plus"
);
ensure_codec!(
    check_dolby_digital_plus2,
    AudioCodec::DolbyDigitalPlus,
    "ddp"
);
ensure_codec!(
    check_dolby_digital_plus3,
    AudioCodec::DolbyDigitalPlus,
    "e-ac-3"
);
ensure_codec!(
    check_dolby_digital_plus4,
    AudioCodec::DolbyDigitalPlus,
    "eac-3"
);
ensure_codec!(
    check_dolby_digital_plus5,
    AudioCodec::DolbyDigitalPlus,
    "eac3"
);
ensure_codec!(
    check_dolby_digital_plus6,
    AudioCodec::DolbyDigitalPlus,
    "eac-3"
);
reject_codec!(
    check_dolby_digital_plus7,
    AudioCodec::DolbyDigitalPlus,
    "eac-3x"
);
ensure_codec!(
    check_dolby_digital_plus8,
    AudioCodec::DolbyDigitalPlus,
    "DDP5.1",
    "DDP"
);

// AudioCodec::DolbyTrueHD
ensure_codec!(check_dolby_true_hd1, AudioCodec::DolbyTrueHD, "true-hd");
ensure_codec!(check_dolby_true_hd2, AudioCodec::DolbyTrueHD, "truehd");
reject_codec!(check_dolby_true_hd3, AudioCodec::DolbyTrueHD, "true hd");

// AudioCodec::DolbyDigital
ensure_codec!(
    check_dolby_digital1,
    AudioCodec::DolbyDigital,
    "dolby-digital"
);
ensure_codec!(check_dolby_digital2, AudioCodec::DolbyDigital, "dolby");
ensure_codec!(check_dolby_digital3, AudioCodec::DolbyDigital, "dd");
ensure_codec!(check_dolby_digital4, AudioCodec::DolbyDigital, "ac3");
ensure_codec!(check_dolby_digital5, AudioCodec::DolbyDigital, "ac-3");
ensure_codec!(check_dolby_digital6, AudioCodec::DolbyDigital, "ac3d");
ensure_codec!(check_dolby_digital7, AudioCodec::DolbyDigital, "ac-3d");
ensure_codec!(check_dolby_digital8, AudioCodec::DolbyDigital, "ac3 hq");
ensure_codec!(check_dolby_digital9, AudioCodec::DolbyDigital, "ac3.hq");
ensure_codec!(check_dolby_digital10, AudioCodec::DolbyDigital, "ac3:hq");
ensure_codec!(check_dolby_digital11, AudioCodec::DolbyDigital, "ac3-hq");
ensure_codec!(check_dolby_digital12, AudioCodec::DolbyDigital, "ac3 ex");
ensure_codec!(check_dolby_digital13, AudioCodec::DolbyDigital, "ac3.ex");
ensure_codec!(check_dolby_digital14, AudioCodec::DolbyDigital, "ac3:ex");
ensure_codec!(check_dolby_digital15, AudioCodec::DolbyDigital, "ac3-ex");
reject_codec!(check_dolby_digital16, AudioCodec::DolbyDigital, "ac3xx");
reject_codec!(check_dolby_digital17, AudioCodec::DolbyDigital, "ac3 hqxx");
ensure_codec!(
    check_dolby_digital18,
    AudioCodec::DolbyDigital,
    "DD5.1",
    "DD"
);

// AudioCodec::DtsHD
ensure_codec!(check_dts_hd1, AudioCodec::DtsHD, "dtshd");
ensure_codec!(check_dts_hd2, AudioCodec::DtsHD, "dts-hd");
ensure_codec!(check_dts_hd3, AudioCodec::DtsHD, "dtsma");
ensure_codec!(check_dts_hd4, AudioCodec::DtsHD, "dts-ma");
reject_codec!(check_dts_hd5, AudioCodec::DtsHD, "dts-hdxx");

// AudioCodec::DtsX
ensure_codec!(check_dts_x1, AudioCodec::DtsX, "dtsx");
ensure_codec!(check_dts_x2, AudioCodec::DtsX, "dts:x");
ensure_codec!(check_dts_x3, AudioCodec::DtsX, "dts-x");
reject_codec!(check_dts_x4, AudioCodec::DtsX, "dts-xx");

// AudioCodec::Dts
ensure_codec!(check_dts1, AudioCodec::Dts, "dts");

// AudioCodec::Flac
ensure_codec!(check_flac1, AudioCodec::Flac, "flac");

// AudioCodec::Mp3
ensure_codec!(check_mp31, AudioCodec::Mp3, "mp3");
ensure_codec!(check_mp32, AudioCodec::Mp3, "lame");

// AudioCodec::Pcm
ensure_codec!(check_pcm1, AudioCodec::Pcm, "pcm");

// AudioCodec::Lpcm
ensure_codec!(check_lpcm1, AudioCodec::Lpcm, "lpcm");

// AudioCodec::Vorbis
ensure_codec!(check_vorbis1, AudioCodec::Vorbis, "vorbis");
