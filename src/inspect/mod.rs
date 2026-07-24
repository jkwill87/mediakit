//! High-level inspection of filenames and filesystem media.
//!
//! Inspection produces display-oriented [`Tag`](crate::meta::Tag) values through the shared
//! [`Inspector`] trait. Choose an inspector according to where the metadata comes from:
//!
//! | Inspector | Opens a file? | Metadata source |
//! | --- | --- | --- |
//! | [`FilenameInspector`] | No | Filename tokens, extension, and standalone-subtitle suffixes |
//! | [`FileInspector`] | Yes | Extension, filesystem properties, and supported container headers |
//!
//! For an ordered, lossless view of every embedded track, use
//! [`crate::probe::FileProber`] instead of [`FileInspector`].
//!
//! # Filename inspection
//!
//! [`FilenameInspector`] accepts a path-like value, isolates its final component, tokenizes it, and
//! runs the filename inspectors in a deterministic order. Calling [`Inspector::analyze`] consumes
//! and returns the inspector so a media-type hint can be configured before the pipeline runs.
//!
//! ```
//! use mediakit::inspect::{FilenameInspector, Inspector};
//! use mediakit::meta::Tag;
//! use mediakit::meta::fields::MediaType;
//!
//! let inspected = FilenameInspector::new(
//!     "Pulp.Fiction.1994.1080p.BluRay.DTS-HD.MA.5.1.H.264.High-ARCHiViST.mkv",
//! )
//! .with_media_type_hint(MediaType::Movie)
//! .analyze();
//!
//! assert_eq!(
//!     inspected.filename(),
//!     "Pulp.Fiction.1994.1080p.BluRay.DTS-HD.MA.5.1.H.264.High-ARCHiViST.mkv",
//! );
//! assert!(inspected.tags().into_iter().any(
//!     |tag| matches!(tag, Tag::Title(title) if title == "Pulp Fiction")
//! ));
//! assert!(inspected.tags().into_iter().any(
//!     |tag| matches!(tag, Tag::PremiereYear(1994))
//! ));
//! ```
//!
//! Automatic movie-versus-television classification is available from
//! [`FilenameInspector::media_type`]. Use [`FilenameInspector::with_media_type_hint`] when a name is
//! ambiguous; [`crate::meta::fields::MediaType::Unknown`] restores automatic classification.
//!
//! ## Flat subtitle metadata and identity
//!
//! Filename facts are exposed only as a flat, ordered tag list. Standalone subtitle filenames can
//! add language and disposition tags. The suffix-free identity is the one exception because
//! callers need it to associate a subtitle with its parent media.
//!
//! ```
//! use mediakit::inspect::{FilenameInspector, Inspector};
//! use mediakit::meta::Tag;
//! use mediakit::meta::fields::{LanguageTag, SubtitleDisposition};
//!
//! let inspected =
//!     FilenameInspector::new("Rango.2011.pt-BR.2.forced.srt").analyze();
//!
//! assert_eq!(inspected.identity_stem(), Some("Rango.2011"));
//! assert!(inspected.tags().into_iter().any(|tag| matches!(
//!     tag,
//!     Tag::SubtitleLanguage(LanguageTag::Language(language))
//!         if language.iso_639_1 == "pt"
//! )));
//! assert!(inspected.tags().into_iter().any(|tag| matches!(
//!     tag,
//!     Tag::SubtitleDisposition(SubtitleDisposition::Forced)
//! )));
//! ```
//!
//! [`FilenameInspector::identity_stem`] removes recognized subtitle suffixes.
//!
//! Language tags summarize each category. One normalized language uses
//! [`LanguageTag::Language`](crate::meta::fields::LanguageTag::Language), while a contiguous block
//! of multiple filename language markers or an explicit scene `MULTi` marker uses
//! [`LanguageTag::Multi`](crate::meta::fields::LanguageTag::Multi) and format as `"multi"`.
//!
//! ```
//! use mediakit::inspect::{FilenameInspector, Inspector};
//! use mediakit::meta::Tag;
//! use mediakit::meta::fields::LanguageTag;
//!
//! let inspected = FilenameInspector::new("Movie.ita.eng.1080p.mkv").analyze();
//! assert!(inspected.tags().into_iter().any(
//!     |tag| matches!(tag, Tag::AudioLanguage(LanguageTag::Multi))
//! ));
//! ```
//!
//! ## Positioned tokens
//!
//! [`FilenameInspector::tokens`] returns the complete tokenization used by the pipeline. Every
//! [`Token`] stores a half-open UTF-8 byte range in [`Token::start`]..[`Token::end`], a
//! [`TokenIdentity`], and an optional [`Tag`](crate::meta::Tag). The ranges are suitable for slicing
//! the string returned by [`FilenameInspector::filename`].
//!
//! ```
//! use mediakit::inspect::{FilenameInspector, Inspector};
//!
//! let inspected = FilenameInspector::new("Amélie.2001.mkv").analyze();
//! for token in inspected.tokens() {
//!     let text = &inspected.filename()[token.start..token.end];
//!     assert!(!text.is_empty());
//! }
//! ```
//!
//! # Filesystem and content inspection
//!
//! [`FileInspector`] emits tags for recognized extension, MIME type, and file size. By default it
//! also probes supported container headers and converts the duration and primary audio/video tracks
//! into technical tags. Audio language is summarized from all embedded audio tracks, including
//! disabled tracks; embedded subtitle tracks are never converted to inspection tags.
//!
//! ```no_run
//! use mediakit::inspect::{FileInspector, Inspector};
//! use mediakit::meta::Tag;
//!
//! let inspected = FileInspector::new("movie.mkv").analyze();
//! for tag in inspected.tags() {
//!     match tag {
//!         Tag::Container(format) => println!("detected {format} content"),
//!         Tag::Runtime(seconds) => println!("runtime: {seconds}s"),
//!         _ => {}
//!     }
//! }
//! ```
//!
//! Content probing is best-effort: [`FileInspector`] does not return an error if a file is
//! missing, unsupported, unreadable, or malformed. Disable it explicitly with
//! [`FileInspector::with_content_inspection`] when only cheap filesystem and extension-derived
//! tags are wanted. Use [`crate::probe::FileProber`] when failures must be observable.
//!
//! # Working with tags
//!
//! [`Inspector::tags`] returns borrowed tags in inspection order. Match on
//! [`Tag`](crate::meta::Tag) variants when retaining type information, or use
//! [`Tag::key`](crate::meta::Tag::key) and [`Tag::value`](crate::meta::Tag::value) for generic
//! display and key/value output. See the [`crate::meta`] module for the complete tag taxonomy.

mod inspector;
mod inspectors;
mod token;
mod token_identity;

pub use inspector::Inspector;
pub use inspectors::file::FileInspector;
pub use inspectors::filename::FilenameInspector;
pub use token::Token;
pub use token_identity::TokenIdentity;
