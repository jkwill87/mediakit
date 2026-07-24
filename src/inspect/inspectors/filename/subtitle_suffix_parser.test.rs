//! Verifies standalone-subtitle suffix parsing.

use super::*;
use std::path::PathBuf;

fn language_code(language: LanguageTag) -> Option<&'static str> {
    match language {
        LanguageTag::Language(language) => Some(language.iso_639_1),
        LanguageTag::Multi => None,
    }
}

#[test]
fn supports_python_formats_and_mediakit_native_formats() {
    for name in [
        "300.2007.en.srt",
        "28.Days.Later.2002.en.idx",
        "Rango.2011.en.sub",
        "Unhinged.2020.en.ass",
        "Wonka.2023.en.ssa",
        "Gladiator.2000.en.vtt",
        "Anora.2024.en.SRT",
    ] {
        assert!(
            ParsedSubtitleSuffix::parse(Path::new(name)).is_some(),
            "{name}"
        );
    }
}

#[test]
fn preserves_language_markers_for_pipeline_normalization() {
    let english = LanguageTag::Language(Language::from_identifier("eng").unwrap());
    let italian = LanguageTag::Language(Language::from_identifier("ita").unwrap());

    assert_eq!(
        ParsedSubtitleSuffix::parse(Path::new("Movie.eng.eng.srt"))
            .unwrap()
            .languages,
        [english, english]
    );
    assert_eq!(
        ParsedSubtitleSuffix::parse(Path::new("Movie.ita.multi.eng.srt"))
            .unwrap()
            .languages,
        [italian, LanguageTag::Multi, english]
    );
    assert!(
        ParsedSubtitleSuffix::parse(Path::new("Multi.srt"))
            .unwrap()
            .languages
            .is_empty()
    );
}

#[test]
fn parses_languages_aliases_tags_and_dispositions() {
    for (name, language, dispositions) in [
        ("The.Bodyguard.1992.eng.srt", "en", vec![]),
        ("Backrooms.2026.English.srt", "en", vec![]),
        ("The.Super.Mario.Galaxy.Movie.2026.pt-BR.srt", "pt", vec![]),
        ("Friday.1995.fr-CA.srt", "fr", vec![]),
        ("Hocus.Pocus.1993.en-CA.srt", "en", vec![]),
        ("Good.Time.2017.es-IT.srt", "es", vec![]),
        ("City.Slickers.1991.de-CH-1901.srt", "de", vec![]),
        ("Wall.Street.1987.sl-rozaj.srt", "sl", vec![]),
        ("Deliverance.1972.en-US-u-ca-gregory.srt", "en", vec![]),
        ("Hamlet.2009.eng.2.srt", "en", vec![]),
        (
            "Clueless.1995.fre.forced.srt",
            "fr",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Misery.1990.forced.fre.srt",
            "fr",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Chappie.2015.en-forced.srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Frozen.2013.forced-en.srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Bambi.1942-forced-en.srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        ("Signs.2002.en-sdh.srt", "en", vec![SubtitleDisposition::Sdh]),
        (
            "Deadpool.2016.en.1.forced.srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Candyman.2021.en (forced).srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Jumanji.1995.en[forced].srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Luca.2021.en(forced).srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Psycho.1960.[en][forced].srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Brazil.1985.[en.forced].srt",
            "en",
            vec![SubtitleDisposition::Forced],
        ),
        (
            "Blackhat.2015.en.forced.sdh.forced.srt",
            "en",
            vec![SubtitleDisposition::Forced, SubtitleDisposition::Sdh],
        ),
        ("forced.eng.srt", "en", vec![SubtitleDisposition::Forced]),
        ("sdh.eng.srt", "en", vec![SubtitleDisposition::Sdh]),
        (
            "commentary.eng.srt",
            "en",
            vec![SubtitleDisposition::Commentary],
        ),
        ("Oldboy.2003.hi.srt", "hi", vec![]),
    ] {
        let parsed = ParsedSubtitleSuffix::parse(Path::new(name)).unwrap();
        assert_eq!(
            parsed.languages.first().copied().and_then(language_code),
            Some(language),
            "{name}"
        );
        assert_eq!(parsed.dispositions, dispositions, "{name}");
    }
}

#[test]
fn unsupported_disposition_words_are_ignored_without_losing_language() {
    for name in [
        "Mrs.Doubtfire.1993.en.cc.srt",
        "The.Muppets.2011.en.hearing_impaired.srt",
        "Spies.in.Disguise.2019.en.default.srt",
        "Emily.the.Criminal.2022.en.foreign.srt",
        "The.Lego.Batman.Movie.2017.en.signs.srt",
        "Cry-Baby.1990.en.songs.srt",
    ] {
        let parsed = ParsedSubtitleSuffix::parse(Path::new(name)).unwrap();
        assert_eq!(
            parsed.languages.first().copied().and_then(language_code),
            Some("en"),
            "{name}"
        );
        assert!(parsed.dispositions.is_empty(), "{name}");
    }
}

#[test]
fn ignores_release_qualifiers_while_retaining_language_and_numbered_dispositions() {
    for (name, association, dispositions) in [
        ("Quiz.Lady.2023.en.UTF8.srt", "quizlady2023", vec![]),
        (
            "Scott.Pilgrim.vs.the.World.2010.en.orig.srt",
            "scottpilgrimvstheworld2010",
            vec![],
        ),
        (
            "The.Princess.and.the.Frog.2009.en.full.srt",
            "theprincessandthefrog2009",
            vec![],
        ),
        (
            "Doctor.Strange.2016.en.commentary2.srt",
            "doctorstrange2016",
            vec![SubtitleDisposition::Commentary],
        ),
        (
            "Strange.Days.1995.en.cc1.srt",
            "strangedays1995",
            vec![],
        ),
    ] {
        let parsed = ParsedSubtitleSuffix::parse(Path::new(name)).unwrap();
        assert_eq!(
            parsed.languages.first().copied().and_then(language_code),
            Some("en"),
            "{name}"
        );
        assert_eq!(parsed.dispositions, dispositions, "{name}");
        assert_eq!(
            parsed.identity_stem().map(normalize_identity_text),
            Some(association.to_owned()),
            "{name}"
        );
    }
}

#[test]
fn disposition_words_can_still_be_media_titles() {
    for (name, language) in [("Signs.srt", None), ("Signs.en.srt", Some("en"))] {
        let parsed = ParsedSubtitleSuffix::parse(Path::new(name)).unwrap();
        assert_eq!(
            parsed.languages.first().copied().and_then(language_code),
            language,
            "{name}"
        );
        assert_eq!(parsed.dispositions, [], "{name}");
        assert_eq!(parsed.identity_stem(), Some("Signs"), "{name}");
    }
}

#[test]
fn retains_identity_stems_after_peeling_suffixes() {
    for (name, association) in [
        (
            "Once.Upon.a.Time.in.America.1984.en.srt",
            "onceuponatimeinamerica1984",
        ),
        ("Toy Story 1995.eng.srt", "toystory1995"),
        ("Infinity_Pool_2023.subeng.srt", "infinitypool2023"),
        ("Labyrinth-1986.eng-sub.srt", "labyrinth1986"),
    ] {
        let parsed = ParsedSubtitleSuffix::parse(Path::new(name)).unwrap();
        assert_eq!(
            parsed.identity_stem().map(normalize_identity_text),
            Some(association.to_owned()),
            "{name}"
        );
    }
}

#[test]
fn generic_language_names_have_no_identity_stem() {
    for name in ["Eng.srt", "English.idx", "subeng.sub"] {
        let parsed = ParsedSubtitleSuffix::parse(Path::new(name)).unwrap();
        assert_eq!(parsed.identity_stem(), None, "{name}");
    }
}

#[test]
fn ambiguous_bare_language_codes_remain_media_titles() {
    for name in [
        "Ar.srt", "Da.srt", "De.srt", "El.srt", "He.srt", "Is.srt", "It.srt", "La.srt", "No.srt",
        "123.srt",
    ] {
        let parsed = ParsedSubtitleSuffix::parse(Path::new(name)).unwrap();
        assert_eq!(parsed.languages.first().copied(), None, "{name}");
        assert!(parsed.identity_stem().is_some(), "{name}");
    }
}

#[test]
fn numeric_suffix_markers_are_removed_from_the_identity() {
    for (name, expected) in [
        (
            "Paws.of.Fury.The.Legend.of.Hank.2022.en.1.srt",
            "Paws.of.Fury.The.Legend.of.Hank.2022",
        ),
        ("Perfect.Blue.1998.en.002.forced.srt", "Perfect.Blue.1998"),
        ("Analyze.That.2002.eng.[3].sub", "Analyze.That.2002"),
    ] {
        let parsed = ParsedSubtitleSuffix::parse(Path::new(name)).unwrap();
        assert_eq!(parsed.identity_stem(), Some(expected), "{name}");
    }
    assert_eq!(
        ParsedSubtitleSuffix::parse(Path::new("123.srt"))
            .unwrap()
            .identity_stem(),
        Some("123")
    );
}

#[cfg(unix)]
#[test]
fn non_utf8_stems_never_produce_an_identity_stem() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let path = PathBuf::from(OsString::from_vec(b"rango-\xff.en.srt".to_vec()));
    let parsed = ParsedSubtitleSuffix::parse(&path).unwrap();

    assert_eq!(parsed.identity_stem(), None);
}
