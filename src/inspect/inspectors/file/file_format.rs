//! Infers media formats from file extensions.

use super::FileInspector;
use crate::meta::Tag;
use crate::meta::fields::MediaFormat;

impl FileInspector {
    pub(super) fn inspect_file_format(mut self) -> Self {
        if let Some(format) = self
            .path
            .extension()
            .and_then(|extension| extension.to_str())
            .and_then(MediaFormat::from_extension)
        {
            self.tags.push(Tag::FileFormat(format));
        }
        self
    }
}
