//! Defines structured metadata recovered from media filenames.

use crate::meta::fields::{MediaFormat, TrackMetadata};

/// Structured metadata established while inspecting a filename.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FilenameMetadata {
    /// The recognized media file format.
    pub format: Option<MediaFormat>,
    /// Metadata for an external track described by the filename.
    pub track: Option<TrackMetadata>,
    identity_stem: Option<String>,
    generic_identity: bool,
}

impl FilenameMetadata {
    /// Returns the filename stem with external-track suffixes removed.
    pub fn identity_stem(&self) -> Option<&str> {
        self.identity_stem.as_deref()
    }

    /// Returns whether the external-track filename carries no useful media identity.
    pub const fn has_generic_identity(&self) -> bool {
        self.generic_identity
    }

    pub(crate) fn set_format(&mut self, format: MediaFormat, identity_stem: Option<String>) {
        self.format = Some(format);
        self.identity_stem = identity_stem;
    }

    pub(crate) fn set_external_track(
        &mut self,
        track: TrackMetadata,
        identity_stem: Option<String>,
        generic_identity: bool,
    ) {
        self.track = Some(track);
        self.identity_stem = identity_stem;
        self.generic_identity = generic_identity;
    }
}
