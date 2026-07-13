//! Verifies feature-gated natural-language detection through the public API.

#![cfg(feature = "with_whatlang")]

use mediakit::meta::fields::Language;

#[test]
fn detects_supported_languages_from_natural_text() {
    for (text, expected) in [
        (
            "The morning train arrived at the station while everyone waited patiently on the platform.",
            "en",
        ),
        (
            "Le train du matin est arrivé à la gare pendant que tout le monde attendait patiemment sur le quai.",
            "fr",
        ),
    ] {
        assert_eq!(
            Language::detect_from_text(text).map(|language| language.iso_639_1),
            Some(expected),
            "{text}"
        );
    }
}

#[test]
fn empty_text_has_no_detectable_language() {
    assert_eq!(Language::detect_from_text(""), None);
}
