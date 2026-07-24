//! Infers media formats from file extensions.

use super::FileInspector;
use crate::meta::Tag;
use crate::meta::fields::MediaFormat;

impl FileInspector {
    pub(super) fn inspect_file_format(self) -> Self {
        let tag = self
            .path
            .extension()
            .and_then(|extension| extension.to_str())
            .and_then(MediaFormat::from_extension)
            .map(Tag::FileFormat);
        let mut tags = self.tags;
        tags.extend(tag);
        Self { tags, ..self }
    }
}
