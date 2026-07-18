//! Probes supported media containers for bounded stream metadata.
//!
//! Each built-in probe reads only the structural metadata needed to populate [`MediaInfo`]:
//! container headers, stream descriptions, and timing tables. Media payloads are never decoded.
//! Format implementations live under `containers`, while reusable byte readers and cross-container
//! media normalization live under `support`.

mod containers;
mod detected_container;
mod error;
mod input;
mod media_info;
mod support;

pub use error::ProbeError;
pub use media_info::{AudioStream, MediaInfo, StreamInfo, SubtitleStream, VideoStream};

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
    pub fn probe(self) -> Result<MediaInfo, ProbeError> {
        let Self {
            mut input,
            detected,
        } = self;
        detected.probe(&mut input)
    }
}

crate::unit_tests!("mod.test.rs");
