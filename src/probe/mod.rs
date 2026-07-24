//! Bounded, content-derived metadata from supported media containers.
//!
//! Probing opens a file, identifies its container from structural signatures, and reads only the
//! headers, track descriptions, and timing tables needed to build [`ProbeResult`]. Media payloads
//! are never decoded.
//!
//! Use this module when every embedded track, container order, or typed failures matter. For a
//! best-effort sequence of display-oriented [`Tag`](crate::meta::Tag) values, see
//! [`crate::inspect::FileInspector`].
//!
//! # Basic workflow
//!
//! [`FileProber::new`] opens the path and detects a supported container. [`FileProber::probe`] then
//! consumes the prepared prober and parses its metadata.
//!
//! ```no_run
//! use mediakit::probe::FileProber;
//!
//! let media = FileProber::new("episode.mkv")?.probe()?;
//!
//! println!("container: {}", media.container);
//! if let Some(duration) = media.duration {
//!     println!("duration: {:.3}s", duration.as_secs_f64());
//! }
//! for (index, track) in media.tracks.iter().enumerate() {
//!     println!("track[{index}]: {track:#?}");
//! }
//! # Ok::<(), mediakit::probe::ProbeError>(())
//! ```
//!
//! Container detection is based on file content, not the filename extension. The normalized result
//! is stored in [`ProbeResult::container`].
//!
//! # Supported containers
//!
//! The built-in probes recognize these content families:
//!
//! | Family | Reported [`MediaFormat`](crate::meta::fields::MediaFormat) |
//! | --- | --- |
//! | Matroska / WebM | [`Mkv`](crate::meta::fields::MediaFormat::Mkv), [`Webm`](crate::meta::fields::MediaFormat::Webm) |
//! | ISO base media / QuickTime | [`Mp4`](crate::meta::fields::MediaFormat::Mp4), [`Mov`](crate::meta::fields::MediaFormat::Mov) |
//! | Audio Video Interleave | [`Avi`](crate::meta::fields::MediaFormat::Avi) |
//! | MPEG transport stream | [`Ts`](crate::meta::fields::MediaFormat::Ts), [`M2ts`](crate::meta::fields::MediaFormat::M2ts) |
//! | Advanced Systems Format | [`Wmv`](crate::meta::fields::MediaFormat::Wmv) |
//!
//! A format appearing in [`crate::meta::fields::MediaFormat::ALL`] does not necessarily imply a
//! content probe. That list also includes extension-recognized external subtitle and multimedia
//! formats.
//!
//! # Result model
//!
//! [`ProbeResult::tracks`] preserves the native order across all supported track kinds.
//!
//! | Type | Container-specific metadata |
//! | --- | --- |
//! | [`VideoTrack`] | Codec and profile, dimensions, normalized resolution, frame rate, and dynamic range |
//! | [`AudioTrack`] | Codec and profile, channel layout, and bit rate |
//! | [`SubtitleTrack`] | Normalized subtitle codec |
//!
//! Every variant embeds [`TrackInfo`] for enabled/default flags and normalized
//! [`Language`](crate::meta::fields::Language). Fields are optional when a container omits them or
//! the bounded parser does not establish them.
//!
//! The typed views [`ProbeResult::video_tracks`], [`ProbeResult::audio_tracks`], and
//! [`ProbeResult::subtitle_tracks`] are allocation-free and retain relative container order. The
//! corresponding `primary_*_track` methods choose an enabled default track first, then any enabled
//! track, then the first track of that kind. Primary status is computed from current [`TrackInfo`]
//! values rather than stored separately.
//!
//! # Error handling
//!
//! [`ProbeError`] distinguishes unsupported content, malformed supported content, and file I/O:
//!
//! ```no_run
//! use mediakit::probe::{FileProber, ProbeError};
//!
//! let result = FileProber::new("movie.mkv").and_then(FileProber::probe);
//! match result {
//!     Ok(media) => println!("detected {}", media.container),
//!     Err(ProbeError::UnsupportedFormat) => println!("unsupported container"),
//!     Err(ProbeError::InvalidData { format, message }) => {
//!         println!("invalid {format} data: {message}");
//!     }
//!     Err(ProbeError::Io(error)) => eprintln!("cannot read file: {error}"),
//!     Err(error) => eprintln!("future probe error: {error}"),
//! }
//! ```
//!
//! The final wildcard keeps the match forward-compatible because [`ProbeError`] is marked
//! `#[non_exhaustive]`.
//!
//! # Relationship to inspection
//!
//! [`crate::inspect::FileInspector`] uses this API internally but intentionally changes the
//! contract: probing failures are ignored, only primary audio/video tracks are converted to
//! technical tags, embedded subtitles are omitted, and extension-derived baseline metadata is
//! retained. Call [`FileProber`] directly when
//! those tradeoffs are not appropriate.
//!
//! Format implementations live under the private `containers` namespace, while reusable byte
//! readers and cross-container normalization live under private `support`. The public API remains
//! container-independent through [`ProbeResult`] and [`Track`].

mod containers;
mod detected_container;
mod error;
mod input;
mod probe_result;
mod support;
mod tracks;

pub use error::ProbeError;
pub use probe_result::ProbeResult;
pub use tracks::{AudioTrack, SubtitleCodec, SubtitleTrack, Track, TrackInfo, VideoTrack};

use detected_container::DetectedContainer;
use input::ProbeInput;
use std::path::Path;

/// Stateful media-file prober shared by every bundled container implementation.
pub struct FileProber {
    /// Open media-file state used by the selected container probe.
    input: ProbeInput,
    /// Prepared typed state for the detected container.
    detected: DetectedContainer,
}

impl FileProber {
    /// Opens a media file and detects its container.
    ///
    /// Returns [`ProbeError::UnsupportedFormat`] when the file does not match a supported
    /// container family.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ProbeError> {
        let mut input = ProbeInput::open(path.as_ref())?;
        let detected =
            DetectedContainer::detect(&mut input)?.ok_or(ProbeError::UnsupportedFormat)?;
        Ok(Self { input, detected })
    }

    /// Parses the detected container and returns its structured metadata.
    pub fn probe(self) -> Result<ProbeResult, ProbeError> {
        let Self {
            mut input,
            detected,
        } = self;
        detected.probe(&mut input)
    }
}

crate::unit_tests!("mod.test.rs");
