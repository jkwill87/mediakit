//! Determines media MIME types from detected or extension-inferred formats.

use super::FileInspector;
use crate::meta::Tag;

impl FileInspector {
    pub(super) fn inspect_mime_type(self) -> Self {
        let format = self
            .tags
            .iter()
            .find_map(|tag| match tag {
                Tag::Container(format) => Some(format),
                _ => None,
            })
            .or_else(|| {
                self.tags.iter().find_map(|tag| match tag {
                    Tag::FileFormat(format) => Some(format),
                    _ => None,
                })
            });
        let tag = format.map(|format| Tag::MimeType(format.mime_type().to_owned()));
        let mut tags = self.tags;
        tags.extend(tag);
        Self { tags, ..self }
    }
}
