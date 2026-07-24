//! Structured media metadata from filenames and container headers.
//!
//! `mediakit` separates two common sources of media metadata:
//!
//! - [`inspect`] extracts descriptive tags from path names and file properties.
//! - [`probe`] reads supported container headers into ordered, typed track metadata.
//!
//! [`meta`] owns flat inspection tags and scalar vocabulary shared by those workflows; probe-owned
//! aggregate records remain in [`probe`].
//!
//! # Choose a workflow
//!
//! | Goal | Entry point | Result |
//! | --- | --- | --- |
//! | Parse a movie, episode, or subtitle filename | [`inspect::FilenameInspector`] | Flat [`meta::Tag`] values |
//! | Inspect extension, size, MIME type, and primary tracks | [`inspect::FileInspector`] | Best-effort flat [`meta::Tag`] values |
//! | Enumerate container tracks or handle probe failures | [`probe::FileProber`] | [`probe::ProbeResult`] or [`probe::ProbeError`] |
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
//! assert_eq!(inspected.media_type(), MediaType::Television);
//! assert!(inspected.tags().into_iter().any(
//!     |tag| matches!(tag, Tag::Title(title) if title == "The Bear")
//! ));
//! assert!(inspected.tags().into_iter().any(
//!     |tag| matches!(tag, Tag::EpisodeNumber(1))
//! ));
//! ```
//!
//! See the [`inspect` module][inspect] for media-type hints, positioned filename
//! [`inspect::Token`]s, subtitle identity accessors, and filesystem inspection.
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
//! Use [`probe::FileProber`] when global track order, every embedded track, or typed error handling
//! matters. Probing detects the container from its bytes rather than trusting the filename
//! extension.
//!
//! ```no_run
//! use mediakit::probe::FileProber;
//!
//! let media = FileProber::new("movie.mkv")?.probe()?;
//! println!("container: {}", media.container);
//! println!("tracks: {}", media.tracks.len());
//! println!("audio tracks: {}", media.audio_tracks().count());
//! println!("subtitle tracks: {}", media.subtitle_tracks().count());
//! # Ok::<(), mediakit::probe::ProbeError>(())
//! ```
//!
//! The [`probe` module][probe] documents supported container families, typed views,
//! primary-track selection, and the [`probe::ProbeError`] variants.
//!
//! # Metadata model
//!
//! [`meta::Tag`] is the common, display-oriented output of inspection. Each variant retains a typed
//! value from [`meta::fields`], such as [`meta::fields::MediaFormat`],
//! [`meta::fields::LanguageTag`], or [`meta::fields::VideoCodec`]. Direct probing returns the more
//! detailed [`probe::ProbeResult`] model instead of flattening every track into tags.
//!
//! An extension-derived [`meta::Tag::FileFormat`] and a content-derived
//! [`meta::Tag::Container`] describe different facts and can legitimately disagree. For the same
//! reason, [`probe::ProbeResult::container`] reports the format detected from probed content.
//!
//! # Feature flags
//!
//! | Feature | Default | Adds |
//! | --- | --- | --- |
//! | `with_serde` | Yes | Serde support for normalized scalar metadata values |
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
