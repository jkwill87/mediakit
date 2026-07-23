//! Structured media metadata from filenames and container headers.
//!
//! `mediakit` separates two common sources of media metadata:
//!
//! - [`inspect`] extracts descriptive tags from path names and file properties.
//! - [`probe`] reads supported container headers into ordered, typed stream metadata.
//!
//! Both workflows use the types in [`meta`], making it possible to start with lightweight
//! filename inspection and opt into file I/O only when content-derived metadata is needed.
//!
//! # Choose a workflow
//!
//! | Goal | Entry point | Result |
//! | --- | --- | --- |
//! | Parse a movie, episode, or sidecar filename | [`inspect::FilenameInspector`] | [`meta::Tag`] values plus [`inspect::FilenameMetadata`] |
//! | Inspect extension, size, MIME type, and primary streams | [`inspect::FileInspector`] | Best-effort [`meta::Tag`] values |
//! | Enumerate container streams or handle probe failures | [`probe::FileProber`] | [`probe::MediaInfo`] or [`probe::ProbeError`] |
//!
//! # Inspect a filename
//!
//! Filename inspection does not open the path, so it also works for names that do not exist on the
//! local filesystem.
//!
//! ```
//! use mediakit::inspect::{FilenameInspector, Inspector};
//! use mediakit::meta::Tag;
//! use mediakit::meta::fields::MediaType;
//!
//! let inspected = FilenameInspector::new(
//!     "The.Bear.S01E01.System.1080p.WEB.H264-FLAME.mkv",
//! )
//! .analyze();
//!
//! assert_eq!(inspected.media_type(), &MediaType::Television);
//! assert!(inspected.tags().into_iter().any(
//!     |tag| matches!(tag, Tag::Title(title) if title == "The Bear")
//! ));
//! assert!(inspected.tags().into_iter().any(
//!     |tag| matches!(tag, Tag::EpisodeNumber(1))
//! ));
//! ```
//!
//! See the [`inspect` module][inspect] for media-type hints, positioned filename
//! [`inspect::Token`]s, structured external-track metadata, and filesystem inspection.
//!
//! # Inspect a file
//!
//! [`inspect::FileInspector`] combines extension and filesystem properties with best-effort
//! container probing. It deliberately keeps useful extension-derived tags if content probing is
//! unsupported or malformed.
//!
//! ```no_run
//! use mediakit::inspect::{FileInspector, Inspector};
//!
//! let inspected = FileInspector::new("movie.mkv").analyze();
//! for tag in inspected.tags() {
//!     println!("{}: {}", tag.key(), tag.value());
//! }
//! ```
//!
//! # Probe container metadata
//!
//! Use [`probe::FileProber`] when stream order, every embedded track, or typed error handling
//! matters. Probing detects the container from its bytes rather than trusting the filename
//! extension.
//!
//! ```no_run
//! use mediakit::probe::FileProber;
//!
//! let media = FileProber::new("movie.mkv")?.probe()?;
//! println!("container: {}", media.container);
//! println!("video streams: {}", media.video_streams.len());
//! println!("audio streams: {}", media.audio_streams.len());
//! println!("subtitle streams: {}", media.subtitle_streams.len());
//! # Ok::<(), mediakit::probe::ProbeError>(())
//! ```
//!
//! The [`probe` module][probe] documents supported container families, primary-stream selection,
//! and the [`probe::ProbeError`] variants. Its stream results use the shared types in
//! [`meta::streams`].
//!
//! # Metadata model
//!
//! [`meta::Tag`] is the common, display-oriented output of inspection. Each variant retains a typed
//! value from [`meta::fields`], such as [`meta::fields::MediaFormat`],
//! [`meta::fields::Language`], or [`meta::fields::VideoCodec`]. Direct probing returns the more
//! detailed [`probe::MediaInfo`] model with streams from [`meta::streams`] instead of flattening
//! every stream into tags.
//!
//! An extension-derived [`meta::Tag::FileFormat`] and a content-derived
//! [`meta::Tag::Container`] describe different facts and can legitimately disagree. For the same
//! reason, [`probe::MediaInfo::container`] is authoritative for probed content.
//!
//! # Feature flags
//!
//! | Feature | Default | Adds |
//! | --- | --- | --- |
//! | `with_serde` | Yes | Serde support for normalized formats, languages, countries, and external-track metadata |
//! | `with_whatlang` | No | Natural-language detection through `Language::detect_from_text` |
//!
//! No feature flag is required for filename inspection, filesystem inspection, or container
//! probing.

#![deny(missing_docs)]

// Allows you to customize the logo
// #![doc(html_logo_url = "path_to_logo", html_favicon_url = "path_to_favicon")] // TODO uncomment

pub mod inspect;
mod macros;
pub(crate) use macros::unit_tests;
pub mod meta;
pub mod probe;
pub(crate) mod regexp;
pub(crate) mod utils;

// // only include if "with_wasm" feature is enabled
// #[cfg(feature = "with_wasm")]
// pub mod wasm;
// #[cfg(feature = "with_wasm")]
// pub use wasm::*;
