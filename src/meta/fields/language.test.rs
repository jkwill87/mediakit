//! Verifies language metadata behavior.

use super::*;

#[test]
fn resolves_extended_languages_and_bibliographic_aliases() {
    for (identifier, expected) in [
        ("cat", "ca"),
        ("Dutch", "nl"),
        ("fre", "fr"),
        ("ger", "de"),
        ("rum", "ro"),
        ("mac", "mk"),
        ("may", "ms"),
        ("slo", "sk"),
        ("khk", "mn"),
        ("nob", "nb"),
        ("tel", "te"),
        ("español", "es"),
        ("vfi", "fr"),
        ("flemish", "nl"),
        ("brazilian", "pt"),
        ("català", "ca"),
        ("latin-american-spanish", "es"),
        ("castilian-spanish", "es"),
        ("french-canadian", "fr"),
        ("traditional-chinese", "zh"),
    ] {
        assert_eq!(
            Language::from_identifier(identifier).map(|language| language.iso_639_1),
            Some(expected),
            "{identifier}"
        );
    }
    assert_eq!(
        Language::from_identifier("en-forced").map(|language| language.iso_639_1),
        Some("en")
    );
}

#[test]
fn resolves_release_aliases_affixes_and_primary_language_tags() {
    for (identifier, expected) in [
        ("pt-BR", "pt"),
        ("ptbr", "pt"),
        ("en-US", "en"),
        ("de-CH-1901", "de"),
        ("sl-rozaj", "sl"),
        ("en-US-u-ca-gregory", "en"),
        ("zh-Hant-TW", "zh"),
        ("subeng", "en"),
        ("engsub", "en"),
        ("vff", "fr"),
        ("ua", "uk"),
        ("cn", "zh"),
        ("jp", "ja"),
    ] {
        assert_eq!(
            Language::from_identifier(identifier).map(|language| language.iso_639_1),
            Some(expected),
            "{identifier}"
        );
    }
}

#[test]
fn resolves_every_embedded_track_language_from_sample_file() {
    let identifiers = [
        "en", "en", "bg", "ca", "cs", "da", "de", "el", "es", "es", "et", "fi", "fr",
        "he", "hr", "hu", "id", "is", "it", "khk", "lt", "lv", "mk", "ms", "nb", "nl",
        "pl", "pt", "pt", "ro", "sk", "sl", "sr", "sv", "th", "tr", "zh", "zh", "zh",
        "es", "pt",
    ];

    assert_eq!(identifiers.len(), 41);
    for identifier in identifiers {
        assert!(
            Language::from_identifier(identifier).is_some(),
            "{identifier}"
        );
    }
    assert_eq!(
        Language::from_identifier("khk").map(|language| language.iso_639_1),
        Some("mn")
    );
}
