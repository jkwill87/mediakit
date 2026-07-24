//! Verifies aggregate language tag formatting and serialization.

use super::*;

#[test]
fn normal_and_multi_values_format_canonically() {
    let english = Language::from_identifier("eng").unwrap();
    assert_eq!(LanguageTag::Language(english).to_string(), "en");
    assert_eq!(LanguageTag::Multi.to_string(), "multi");
}

#[cfg(feature = "serde")]
#[test]
fn normal_and_multi_values_serialize_as_tag_values() {
    let english = LanguageTag::Language(Language::from_identifier("eng").unwrap());
    for (value, encoded) in [(english, "\"en\""), (LanguageTag::Multi, "\"multi\"")] {
        assert_eq!(serde_json::to_string(&value).unwrap(), encoded);
        assert_eq!(serde_json::from_str::<LanguageTag>(encoded).unwrap(), value);
    }
}
