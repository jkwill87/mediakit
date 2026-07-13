//! Reads media file sizes into metadata tags.

use super::FileInspector;
use crate::meta::Tag;

impl FileInspector {
    pub(super) fn inspect_file_size(mut self) -> Self {
        if let Ok(metadata) = self.path.metadata() {
            self.tags.push(Tag::FileSize(metadata.len()));
        }
        self
    }
}
