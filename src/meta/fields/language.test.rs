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
