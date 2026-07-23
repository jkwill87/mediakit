//! Shared metadata vocabulary for inspection and probing.
//!
//! This module has three layers:
//!
//! - [`Tag`] identifies what a value means in a generic inspection result.
//! - [`fields`] contains the normalized domain types carried by tags and probe streams.
//! - [`streams`] contains typed audio, video, and embedded-subtitle stream metadata.
//!
//! The separation lets generic consumers render a sequence of tags with [`Tag::key`] and
//! [`Tag::value`], while typed consumers can match a variant and continue working with values such
//! as [`fields::MediaFormat`], [`fields::Language`], or [`fields::VideoCodec`].
//!
//! # Reading inspection tags
//!
//! ```
//! use mediakit::inspect::{FilenameInspector, Inspector};
//! use mediakit::meta::Tag;
//!
//! let inspected =
//!     FilenameInspector::new("Arrival.2016.2160p.UHD.BluRay.H265.mkv").analyze();
//!
//! for tag in inspected.tags() {
//!     match tag {
//!         Tag::Title(title) => assert_eq!(title, "Arrival"),
//!         Tag::PremiereYear(year) => assert_eq!(*year, 2016),
//!         Tag::VideoCodec(codec) => println!("video codec: {codec}"),
//!         _ => {}
//!     }
//! }
//! ```
//!
//! [`Tag::key`] returns a snake-case category name and [`Tag::value`] formats the contained value:
//!
//! ```
//! use mediakit::meta::Tag;
//! use mediakit::meta::fields::MediaFormat;
//!
//! let tag = Tag::FileFormat(MediaFormat::Mkv);
//! assert_eq!(tag.key(), "file_format");
//! assert_eq!(tag.value(), "mkv");
//! ```
//!
//! # Tag taxonomy
//!
//! The variants are grouped by the source or media property they describe:
//!
//! | Group | Variants |
//! | --- | --- |
//! | File and container | [`Tag::FileFormat`], [`Tag::Container`], [`Tag::MimeType`], [`Tag::FileSize`] |
//! | Identity and release | [`Tag::Title`], [`Tag::AlternativeTitle`], [`Tag::ReleaseGroup`], [`Tag::ReleaseSource`], [`Tag::PremiereYear`] |
//! | Episode and timing | [`Tag::AirDate`], [`Tag::SeasonNumber`], [`Tag::EpisodeNumber`], [`Tag::EpisodeTitle`], [`Tag::Runtime`] |
//! | Audio | [`Tag::AudioCodec`], [`Tag::AudioProfile`], [`Tag::AudioLayout`], [`Tag::AudioBitRate`], [`Tag::AudioLanguage`] |
//! | Subtitle | [`Tag::SubtitleLanguage`], [`Tag::SubtitleTrack`], [`Tag::SubtitleDisposition`] |
//! | Video | [`Tag::VideoCodec`], [`Tag::VideoProfile`], [`Tag::VideoResolution`], [`Tag::VideoFrameRate`], [`Tag::VideoDynamicRange`] |
//! | Disc | [`Tag::DiscNumber`] |
//!
//! # Inferred formats and detected containers
//!
//! [`Tag::FileFormat`] is inferred from a filename extension. [`Tag::Container`] is detected from
//! file content by [`crate::inspect::FileInspector`]. They intentionally remain separate: an
//! incorrectly named file may contain a different container, and filename-only inspection cannot
//! establish the content format.
//!
//! Direct [`crate::probe`] results use [`crate::probe::MediaInfo::container`] and the structures in
//! [`streams`] instead of flattening every stream into tags.
//!
//! # Normalized fields
//!
//! The [`fields`] page groups format, language, country, codec, profile, resolution, date, and
//! external-track types. Most implement [`std::fmt::Display`], and enum-like fields can be parsed
//! through [`std::str::FromStr`] where their item documentation advertises that implementation.
//! Prefer these typed values over comparing the output of [`Tag::value`] in application logic.

pub mod fields;
pub mod streams;
mod tag;
pub use tag::Tag;
