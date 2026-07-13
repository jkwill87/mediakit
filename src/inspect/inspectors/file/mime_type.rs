//! Infers media MIME types from file extensions.

use super::FileInspector;
use crate::meta::Tag;

impl FileInspector {
    pub(super) fn inspect_mime_type(mut self) -> Self {
        let mime_type = self.tags.iter().find_map(|tag| match tag {
            Tag::FileFormat(format) => Some(format.mime_type()),
            _ => None,
        });
        if let Some(mime_type) = mime_type {
            self.tags.push(Tag::MimeType(mime_type.to_owned()));
        }
        self
    }
}
