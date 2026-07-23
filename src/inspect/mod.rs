//! High-level inspection of filenames and filesystem media.
//!
//! Inspection produces display-oriented [`Tag`](crate::meta::Tag) values through the shared
//! [`Inspector`] trait. Choose an inspector according to where the metadata comes from:
//!
//! | Inspector | Opens a file? | Metadata source |
//! | --- | --- | --- |
//! | [`FilenameInspector`] | No | Filename tokens, extension, and external-track suffixes |
//! | [`FileInspector`] | Yes | Extension, filesystem properties, and supported container headers |
//!
//! For an ordered, lossless view of every embedded stream, use
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
//! ## Structured filename metadata
//!
//! Tags are convenient for rendering and generic consumers. The
//! [`FilenameInspector::metadata`] view additionally exposes the normalized
//! [`crate::meta::fields::MediaFormat`] and any external audio or subtitle track encoded by a
//! sidecar filename.
//!
//! ```
//! use mediakit::inspect::{FilenameInspector, Inspector};
//! use mediakit::meta::fields::{
//!     MediaFormat, TrackDisposition, TrackKind,
//! };
//!
//! let inspected =
//!     FilenameInspector::new("Rango.2011.pt-BR.2.forced.srt").analyze();
//! let metadata = inspected.metadata();
//! let track = metadata.track.as_ref().expect("external track metadata");
//!
//! assert_eq!(metadata.format, Some(MediaFormat::Srt));
//! assert_eq!(metadata.identity_stem(), Some("Rango.2011"));
//! assert_eq!(track.kind, TrackKind::Subtitle);
//! assert_eq!(track.language.map(|language| language.iso_639_1), Some("pt"));
//! assert_eq!(track.number, Some(2));
//! assert_eq!(track.dispositions, [TrackDisposition::Forced]);
//! ```
//!
//! [`FilenameMetadata::identity_stem`] removes recognized external-track suffixes so callers can
//! associate sidecars with their parent media identity.
//! [`FilenameMetadata::has_generic_identity`] distinguishes names such as `English.srt` that do not
//! carry a useful media identity.
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
//! also probes supported container headers and converts the duration and preferred audio/video
//! streams into tags.
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

mod filename_metadata;
mod inspector;
mod inspectors;
mod token;
mod token_identity;

pub use filename_metadata::FilenameMetadata;
pub use inspector::Inspector;
pub use inspectors::file::FileInspector;
pub use inspectors::filename::FilenameInspector;
pub use token::Token;
pub use token_identity::TokenIdentity;
