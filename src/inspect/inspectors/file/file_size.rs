//! Reads media file sizes into metadata tags.

use super::FileInspector;
use crate::meta::Tag;

impl FileInspector {
    pub(super) fn inspect_file_size(self) -> Self {
        let tag = self
            .path
            .metadata()
            .ok()
            .map(|metadata| Tag::FileSize(metadata.len()));
        let mut tags = self.tags;
        tags.extend(tag);
        Self { tags, ..self }
    }
}
