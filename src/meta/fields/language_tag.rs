//! Defines aggregate language values emitted by inspection tags.

use super::Language;

/// A language value summarized across one inspection category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageTag {
    /// One normalized language was identified.
    Language(Language),
    /// Multiple language observations or an explicit multi marker were identified.
    Multi,
}

impl std::fmt::Display for LanguageTag {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Language(language) => language.fmt(formatter),
            Self::Multi => formatter.write_str("multi"),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for LanguageTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LanguageTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.eq_ignore_ascii_case("multi") {
            return Ok(Self::Multi);
        }
        Language::from_identifier(&value)
            .map(Self::Language)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown language tag: {value}")))
    }
}

crate::unit_tests!("language_tag.test.rs");
