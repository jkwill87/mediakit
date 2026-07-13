//! Inspects filename tokens for media categories.

use super::FilenameInspector;
use crate::meta::{Tag, fields::MediaType};

impl FilenameInspector {
    /// Selects the media type from an explicit hint or structural metadata.
    ///
    /// **Preconditions:**
    /// - Requires the episode ordering and air date to have been previously selected.
    pub(super) fn inspect_media_type(self) -> Self {
        let media_type = self.media_type_hint.clone().unwrap_or_else(|| {
            self.tokens
                .iter()
                .find(|token| {
                    matches!(
                        token.tag,
                        Some(Tag::EpisodeNumber(_))
                            | Some(Tag::SeasonNumber(_))
                            | Some(Tag::AirDate(_))
                    )
                })
                .map_or(MediaType::Movie, |_| MediaType::Television)
        });
        Self { media_type, ..self }
    }
}

crate::unit_tests!("media_type.test.rs");
