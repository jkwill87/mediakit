//! Normalized values used by [`Tag`](crate::meta::Tag) and probe tracks.
//!
//! Field types keep parsing aliases and presentation spelling at the edge of the library. Once a
//! value is recognized, consumers can compare an enum or structured value instead of carrying
//! filename spellings, container identifiers, or ad-hoc strings through application code.
//!
//!
//! # Field groups
//!
//! | Domain              | Types                                                                        |
//! | ------------------- | ---------------------------------------------------------------------------- |
//! | File classification | [`MediaFormat`], [`ContentKind`], [`MediaType`]                              |
//! | Locale              | [`Language`], [`LanguageTag`], [`LANG_ALL`], [`Country`], [`COUNTRY_ALL`]    |
//! | Release and timing  | [`AirDate`], [`ReleaseSource`]                                               |
//! | Audio               | [`AudioCodec`], [`AudioProfile`], [`AudioLayout`]                            |
//! | Video               | [`VideoCodec`], [`VideoProfile`], [`VideoResolution`], [`VideoDynamicRange`] |
//! | Subtitle filenames  | [`SubtitleDisposition`]                                                      |
//!
//!
//! # Formats
//!
//! [`MediaFormat`] is shared by extension inspection and content probing. It connects a normalized
//! format to its canonical extension, MIME type, and broad [`ContentKind`].
//!
//! ```
//! use mediakit::meta::fields::{ContentKind, MediaFormat};
//!
//! let format = MediaFormat::from_extension(".MKV").expect("known extension");
//! assert_eq!(format, MediaFormat::Mkv);
//! assert_eq!(format.extension(), "mkv");
//! assert_eq!(format.mime_type(), "video/x-matroska");
//! assert_eq!(format.content_kind(), ContentKind::AudioVideo);
//! assert!(!format.is_subtitle());
//! ```
//!
//! [`MediaFormat::ALL`] enumerates every extension-recognized format. The smaller set of formats
//! that can be detected from content is documented by [`crate::probe`].
//!
//! # Languages and countries
//!
//! [`Language::from_identifier`] accepts ISO 639-1 and ISO 639-3 codes, full names, common aliases,
//! and well-formed IETF language tags. The returned value exposes a normalized name and both ISO
//! codes.
//!
//! ```
//! use mediakit::meta::fields::Language;
//!
//! let language = Language::from_identifier("pt-BR").expect("supported language");
//! assert_eq!(language.name, "portuguese");
//! assert_eq!(language.iso_639_1, "pt");
//! assert_eq!(language.iso_639_3, "por");
//! assert_eq!(language.to_string(), "pt");
//! ```
//!
//! [`LANG_ALL`] and [`COUNTRY_ALL`] provide complete lookup tables. When the `with_whatlang`
//! feature is enabled, `Language::detect_from_text` additionally recognizes supported languages
//! from natural-language samples.
//!
//! # Inspection language summaries
//!
//! Probe tracks use `Option<Language>` because every track has at most one known language.
//! Inspection instead uses [`LanguageTag`] to flatten a category into one value: either one
//! [`LanguageTag::Language`] or [`LanguageTag::Multi`]. Absence of a language tag represents no
//! known language. [`SubtitleDisposition`] describes standalone subtitle suffixes such as `forced`.
//!
//! # Formatting and serialization
//!
//! Normalized field types implement [`std::fmt::Display`] where they have a canonical textual
//! representation. With the default `with_serde` feature, formats, languages, countries, and
//! subtitle-filename metadata also implement the appropriate Serde traits.

mod air_date;
mod audio_codec;
mod audio_layout;
mod audio_profile;
mod country;
mod language;
mod language_tag;
mod media_format;
mod media_type;
mod release_source;
mod subtitle_disposition;
mod video_codec;
mod video_dynamic_range;
mod video_profile;
mod video_resolution;

pub use air_date::AirDate;
pub use audio_codec::AudioCodec;
pub use audio_layout::AudioLayout;
pub use audio_profile::AudioProfile;
pub use country::{COUNTRY_ALL, Country};
pub use language::{LANG_ALL, Language};
pub use language_tag::LanguageTag;
pub use media_format::{ContentKind, MediaFormat};
pub use media_type::MediaType;
pub use release_source::ReleaseSource;
pub use subtitle_disposition::SubtitleDisposition;
pub use video_codec::VideoCodec;
pub use video_dynamic_range::VideoDynamicRange;
pub use video_profile::VideoProfile;
pub use video_resolution::VideoResolution;

#[expect(
    dead_code,
    reason = "field abstraction is reserved for metadata formatting"
)]
trait Field {
    fn label(&self) -> &'static str;
    fn key(&self) -> &'static str;
    fn value(&self) -> String;
}
