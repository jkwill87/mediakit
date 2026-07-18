//! Defines private detected-container dispatch.

use super::containers::{self, asf, avi, matroska, mp4, mpeg_ts};
use super::{MediaInfo, ProbeError, ProbeInput};
use crate::meta::fields::MediaFormat;
use std::io;

/// Selected container implementation and any reusable state produced during detection.
pub(super) enum DetectedContainer {
    /// Detected ASF container.
    Asf,
    /// Detected AVI container.
    Avi,
    /// Detected ISO-BMFF container.
    IsoBmff,
    /// Detected Matroska container.
    Matroska,
    /// Detected MPEG transport-stream packet layout.
    MpegTransportStream(mpeg_ts::Layout),
}

impl DetectedContainer {
    /// Detects the first matching container in the documented precedence order.
    pub(super) fn detect(input: &mut ProbeInput) -> io::Result<Option<Self>> {
        let mut prefix = [0; containers::DETECTION_BYTES];
        let prefix = input.read_prefix(&mut prefix)?;

        Ok(matroska::matches(prefix)
            .then_some(Self::Matroska)
            .or_else(|| avi::matches(prefix).then_some(Self::Avi))
            .or_else(|| asf::matches(prefix).then_some(Self::Asf))
            .or_else(|| mp4::matches(prefix).then_some(Self::IsoBmff))
            .or_else(|| mpeg_ts::detect(prefix).map(Self::MpegTransportStream)))
    }

    /// Returns the representative format used when probing fails before subtype resolution.
    pub(super) const fn fallback_format(&self) -> MediaFormat {
        match self {
            Self::Asf => MediaFormat::Wmv,
            Self::Avi => MediaFormat::Avi,
            Self::IsoBmff => MediaFormat::Mp4,
            Self::Matroska => MediaFormat::Mkv,
            Self::MpegTransportStream(layout) => layout.media_format(),
        }
    }

    /// Runs the selected container implementation, forwarding retained state when present.
    pub(super) fn probe(self, input: &mut ProbeInput) -> Result<MediaInfo, ProbeError> {
        let fallback = self.fallback_format();
        match self {
            Self::Asf => asf::probe(input).map_err(|error| ProbeError::from_probe(fallback, error)),
            Self::Avi => avi::probe(input).map_err(|error| ProbeError::from_probe(fallback, error)),
            Self::IsoBmff => mp4::probe(input),
            Self::Matroska => matroska::probe(input),
            Self::MpegTransportStream(layout) => mpeg_ts::probe(input, layout)
                .map_err(|error| ProbeError::from_probe(fallback, error)),
        }
    }
}
