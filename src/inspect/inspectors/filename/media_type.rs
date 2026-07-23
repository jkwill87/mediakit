//! Inspects filename tokens for media categories.

use super::FilenameInspector;
use crate::meta::{Tag, fields::MediaType};

impl FilenameInspector {
    /// Returns the media type selected from an explicit hint or structural metadata.
    ///
    /// Automatic classification reflects the structural metadata currently present in the
    /// inspector's tokens and is therefore intended to be read after
    /// [`Inspector::analyze`](crate::inspect::Inspector::analyze).
    pub fn media_type(&self) -> MediaType {
        self.media_type_hint.clone().unwrap_or_else(|| {
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
        })
    }
}

crate::unit_tests!("media_type.test.rs");
