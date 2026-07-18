//! Defines normalized language metadata and identifier lookup.

/// Language represents the spoken language of a media file or the language of a subtitle.
#[derive(Debug, Eq, Clone, Copy)]
pub struct Language {
    /// The full human-readable name of the language.
    pub name: &'static str,
    /// The two-letter ISO 639-1 language code.
    pub iso_639_1: &'static str,
    /// The three-letter ISO 639-3 language code.
    pub iso_639_3: &'static str,
}

const LANG_ARABIC: Language = Language {
    name: "arabic",
    iso_639_1: "ar",
    iso_639_3: "ara",
};
const LANG_BULGARIAN: Language = Language {
    name: "bulgarian",
    iso_639_1: "bg",
    iso_639_3: "bul",
};
const LANG_CHINESE: Language = Language {
    name: "chinese",
    iso_639_1: "zh",
    iso_639_3: "zho",
};
const LANG_CATALAN: Language = Language {
    name: "catalan",
    iso_639_1: "ca",
    iso_639_3: "cat",
};
const LANG_CROATIAN: Language = Language {
    name: "croatian",
    iso_639_1: "hr",
    iso_639_3: "hrv",
};
const LANG_CZECH: Language = Language {
    name: "czech",
    iso_639_1: "cs",
    iso_639_3: "ces",
};
const LANG_DANISH: Language = Language {
    name: "danish",
    iso_639_1: "da",
    iso_639_3: "dan",
};
const LANG_DUTCH: Language = Language {
    name: "dutch",
    iso_639_1: "nl",
    iso_639_3: "nld",
};
const LANG_ESTONIAN: Language = Language {
    name: "estonian",
    iso_639_1: "et",
    iso_639_3: "est",
};
const LANG_ENGLISH: Language = Language {
    name: "english",
    iso_639_1: "en",
    iso_639_3: "eng",
};
const LANG_FRENCH: Language = Language {
    name: "french",
    iso_639_1: "fr",
    iso_639_3: "fra",
};
const LANG_GERMAN: Language = Language {
    name: "german",
    iso_639_1: "de",
    iso_639_3: "deu",
};
const LANG_GREEK: Language = Language {
    name: "greek",
    iso_639_1: "el",
    iso_639_3: "ell",
};
const LANG_HEBREW: Language = Language {
    name: "hebrew",
    iso_639_1: "he",
    iso_639_3: "heb",
};
const LANG_HINDI: Language = Language {
    name: "hindi",
    iso_639_1: "hi",
    iso_639_3: "hin",
};
const LANG_HUNGARIAN: Language = Language {
    name: "hungarian",
    iso_639_1: "hu",
    iso_639_3: "hun",
};
const LANG_INDONESIAN: Language = Language {
    name: "indonesian",
    iso_639_1: "id",
    iso_639_3: "ind",
};
const LANG_FINNISH: Language = Language {
    name: "finnish",
    iso_639_1: "fi",
    iso_639_3: "fin",
};
const LANG_ICELANDIC: Language = Language {
    name: "icelandic",
    iso_639_1: "is",
    iso_639_3: "isl",
};
const LANG_ITALIAN: Language = Language {
    name: "italian",
    iso_639_1: "it",
    iso_639_3: "ita",
};
const LANG_JAPANESE: Language = Language {
    name: "japanese",
    iso_639_1: "ja",
    iso_639_3: "jpn",
};
const LANG_KOREAN: Language = Language {
    name: "korean",
    iso_639_1: "ko",
    iso_639_3: "kor",
};
const LANG_LATVIAN: Language = Language {
    name: "latvian",
    iso_639_1: "lv",
    iso_639_3: "lav",
};
const LANG_LITHUANIAN: Language = Language {
    name: "lithuanian",
    iso_639_1: "lt",
    iso_639_3: "lit",
};
const LANG_MACEDONIAN: Language = Language {
    name: "macedonian",
    iso_639_1: "mk",
    iso_639_3: "mkd",
};
const LANG_MALAY: Language = Language {
    name: "malay",
    iso_639_1: "ms",
    iso_639_3: "msa",
};
const LANG_MONGOLIAN: Language = Language {
    name: "mongolian",
    iso_639_1: "mn",
    iso_639_3: "mon",
};
const LANG_NORWEGIAN: Language = Language {
    name: "norwegian",
    iso_639_1: "no",
    iso_639_3: "nor",
};
const LANG_NORWEGIAN_BOKMAL: Language = Language {
    name: "norwegian bokmål",
    iso_639_1: "nb",
    iso_639_3: "nob",
};
const LANG_POLISH: Language = Language {
    name: "polish",
    iso_639_1: "pl",
    iso_639_3: "pol",
};
const LANG_LATIN: Language = Language {
    name: "latin",
    iso_639_1: "la",
    iso_639_3: "lat",
};
const LANG_PERSIAN: Language = Language {
    name: "persian",
    iso_639_1: "fa",
    iso_639_3: "fas",
};
const LANG_PORTUGUESE: Language = Language {
    name: "portuguese",
    iso_639_1: "pt",
    iso_639_3: "por",
};
const LANG_RUSSIAN: Language = Language {
    name: "russian",
    iso_639_1: "ru",
    iso_639_3: "rus",
};
const LANG_ROMANIAN: Language = Language {
    name: "romanian",
    iso_639_1: "ro",
    iso_639_3: "ron",
};
const LANG_SERBIAN: Language = Language {
    name: "serbian",
    iso_639_1: "sr",
    iso_639_3: "srp",
};
const LANG_SLOVAK: Language = Language {
    name: "slovak",
    iso_639_1: "sk",
    iso_639_3: "slk",
};
const LANG_SLOVENIAN: Language = Language {
    name: "slovenian",
    iso_639_1: "sl",
    iso_639_3: "slv",
};
const LANG_SPANISH: Language = Language {
    name: "spanish",
    iso_639_1: "es",
    iso_639_3: "spa",
};
const LANG_SWEDISH: Language = Language {
    name: "swedish",
    iso_639_1: "sv",
    iso_639_3: "swe",
};
const LANG_TELUGU: Language = Language {
    name: "telugu",
    iso_639_1: "te",
    iso_639_3: "tel",
};
const LANG_THAI: Language = Language {
    name: "thai",
    iso_639_1: "th",
    iso_639_3: "tha",
};
const LANG_TURKISH: Language = Language {
    name: "turkish",
    iso_639_1: "tr",
    iso_639_3: "tur",
};
const LANG_UKRAINIAN: Language = Language {
    name: "ukrainian",
    iso_639_1: "uk",
    iso_639_3: "ukr",
};
const LANG_VIETNAMESE: Language = Language {
    name: "vietnamese",
    iso_639_1: "vi",
    iso_639_3: "vie",
};

const LANG_ALL_COUNT: usize = 45;

/// A list of all languages that are known and supported by the library.
pub const LANG_ALL: [&Language; LANG_ALL_COUNT] = [
    &LANG_ARABIC,
    &LANG_BULGARIAN,
    &LANG_CATALAN,
    &LANG_CHINESE,
    &LANG_CROATIAN,
    &LANG_CZECH,
    &LANG_DANISH,
    &LANG_DUTCH,
    &LANG_ENGLISH,
    &LANG_ESTONIAN,
    &LANG_FINNISH,
    &LANG_FRENCH,
    &LANG_GERMAN,
    &LANG_GREEK,
    &LANG_HEBREW,
    &LANG_HINDI,
    &LANG_HUNGARIAN,
    &LANG_ICELANDIC,
    &LANG_INDONESIAN,
    &LANG_ITALIAN,
    &LANG_JAPANESE,
    &LANG_KOREAN,
    &LANG_LATIN,
    &LANG_LATVIAN,
    &LANG_LITHUANIAN,
    &LANG_MACEDONIAN,
    &LANG_MALAY,
    &LANG_MONGOLIAN,
    &LANG_NORWEGIAN,
    &LANG_NORWEGIAN_BOKMAL,
    &LANG_PERSIAN,
    &LANG_POLISH,
    &LANG_PORTUGUESE,
    &LANG_ROMANIAN,
    &LANG_RUSSIAN,
    &LANG_SERBIAN,
    &LANG_SLOVAK,
    &LANG_SLOVENIAN,
    &LANG_SPANISH,
    &LANG_SWEDISH,
    &LANG_TELUGU,
    &LANG_THAI,
    &LANG_TURKISH,
    &LANG_UKRAINIAN,
    &LANG_VIETNAMESE,
];

/// Returns a [`whatlang::Detector`](https://docs.rs/whatlang/latest/whatlang/struct.Detector)
/// capable of detecting the library's supported languages.
///
/// The detector is cached and will be initialized lazily on the first call.
#[cfg(feature = "with_whatlang")]
fn whatlang_detector() -> &'static whatlang::Detector {
    use std::sync::LazyLock;
    static WHATLANG_DETECTOR: LazyLock<whatlang::Detector> = LazyLock::new(|| {
        whatlang::Detector::with_allowlist(
            LANG_ALL
                .iter()
                .filter_map(|language| whatlang::Lang::from_code(language.iso_639_3))
                .collect(),
        )
    });
    &WHATLANG_DETECTOR
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.iso_639_1 == other.iso_639_1
    }
}

impl std::hash::Hash for Language {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iso_639_1.hash(state);
    }
}

impl Language {
    /// Finds a supported language by ISO 639 code, common alias, IETF tag, or full name.
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        let normalized = identifier.trim().to_ascii_lowercase();
        let normalized = normalized
            .strip_prefix("sub-")
            .or_else(|| normalized.strip_prefix("sub"))
            .unwrap_or(&normalized);
        let normalized = normalized
            .strip_suffix("-sub")
            .or_else(|| normalized.strip_suffix("sub"))
            .unwrap_or(normalized);
        let alias = match normalized {
            "chi"
            | "cn"
            | "chinese-simplified"
            | "simplified-chinese"
            | "chinese-traditional"
            | "traditional-chinese" => "zh",
            "cz" | "cze" => "cs",
            "dut" | "flemish" => "nl",
            "fre" | "vf" | "vff" | "vfi" | "vfq" | "french-canadian" | "canadian-french" => "fr",
            "ger" | "swissgerman" => "de",
            "gre" => "el",
            "ice" => "is",
            "jp" => "ja",
            "khk" => "mn",
            "mac" => "mk",
            "may" => "ms",
            "slo" => "sk",
            "engb" | "enus" => "en",
            "esp"
            | "espanol"
            | "español"
            | "esla"
            | "latin-american-spanish"
            | "castilian-spanish" => "es",
            "pb" | "po" | "pob" | "brazilian" | "ptbr" => "pt",
            "rum" => "ro",
            "ua" => "uk",
            "catala" | "català" => "ca",
            "zhcn" | "zhtw" => "zh",
            value => value,
        };
        if let Some(language) = Self::detect_from_id(alias) {
            return Some(*language);
        }
        let (primary, subtags) = alias.split_once(['-', '_'])?;
        valid_ietf_subtags(subtags).then(|| Self::detect_from_id(primary).copied())?
    }

    /// Returns a language by its ISO 639-1 code.
    fn from_iso_639_1(iso_639_1: &str) -> Option<&'static Self> {
        LANG_ALL
            .into_iter()
            .find(|language| language.iso_639_1.to_lowercase() == iso_639_1.to_lowercase())
    }

    /// Returns a language by its ISO 639-3 code.
    fn from_iso_639_3(iso_639_3: &str) -> Option<&'static Self> {
        LANG_ALL
            .into_iter()
            .find(|language| language.iso_639_3.to_lowercase() == iso_639_3.to_lowercase())
    }

    /// Returns a language by its full name.
    fn from_name(name: &str) -> Option<&'static Self> {
        LANG_ALL
            .into_iter()
            .find(|language: &&Self| language.name.to_lowercase() == name.to_lowercase())
    }

    /// Attempts to find a language by either its ISO 639 code or full name.
    fn detect_from_id(s: &str) -> Option<&'static Self> {
        Self::from_iso_639_1(s)
            .or_else(|| Self::from_iso_639_3(s))
            .or_else(|| Self::from_name(s))
    }

    /// Detects a supported language from natural-language text.
    ///
    /// This method is available when the `with_whatlang` feature is enabled.
    #[cfg(feature = "with_whatlang")]
    pub fn detect_from_text(text: &str) -> Option<Self> {
        whatlang_detector().detect_lang(text).map(|lang| {
            *Self::from_iso_639_3(lang.code()).expect("language code not supported by library")
        })
    }
}

fn valid_ietf_subtags(value: &str) -> bool {
    let mut extension = None;
    let mut needs_extension_value = false;
    let mut found = false;
    for subtag in value.split(['-', '_']) {
        found = true;
        if subtag.is_empty()
            || !subtag
                .chars()
                .all(|character| character.is_ascii_alphanumeric())
        {
            return false;
        }
        if subtag.len() == 1 {
            if needs_extension_value {
                return false;
            }
            extension = Some(subtag.eq_ignore_ascii_case("x"));
            needs_extension_value = true;
            continue;
        }
        if let Some(private_use) = extension {
            let allowed = if private_use { 1..=8 } else { 2..=8 };
            if !allowed.contains(&subtag.len()) {
                return false;
            }
            needs_extension_value = false;
            continue;
        }
        let characters = || subtag.chars();
        let standard = match subtag.len() {
            2 => characters().all(|character| character.is_ascii_alphabetic()),
            3 => {
                characters().all(|character| character.is_ascii_alphabetic())
                    || characters().all(|character| character.is_ascii_digit())
            }
            4 => {
                characters().all(|character| character.is_ascii_alphabetic())
                    || (subtag.as_bytes()[0].is_ascii_digit()
                        && characters().all(|character| character.is_ascii_alphanumeric()))
            }
            5..=8 => characters().all(|character| character.is_ascii_alphanumeric()),
            _ => false,
        };
        if !standard {
            return false;
        }
    }
    found && !needs_extension_value
}

crate::unit_tests!("language.test.rs");

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_identifier(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown language: {s}")))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.name)
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.iso_639_1)
    }
}
