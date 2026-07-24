//! Inspects filesystem properties and container content for media metadata.

mod container;
mod file_format;
mod file_size;
mod mime_type;

use crate::inspect::Inspector;
use crate::meta::Tag;
use std::path::{Path, PathBuf};

/// Parses media metadata from a file's properties.
///
/// Container probing is best-effort: invalid and unsupported content retains extension-derived
/// file-format and MIME tags rather than failing inspection.
pub struct FileInspector {
    path: PathBuf,
    tags: Vec<Tag>,
    inspect_content: bool,
}

impl FileInspector {
    /// Creates a new [`FileInspector`] for the given file path.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            tags: Vec::new(),
            inspect_content: true,
        }
    }

    /// Enables or disables media-container content probing.
    ///
    /// File size and extension-derived file-format/MIME tags are still inspected when content
    /// probing is disabled.
    pub const fn with_content_inspection(mut self, enabled: bool) -> Self {
        self.inspect_content = enabled;
        self
    }
}

impl Inspector for FileInspector {
    fn analyze(self) -> Self {
        self.inspect_file_format()
            .inspect_file_size()
            .inspect_container()
            .inspect_mime_type()
    }

    fn tags(&self) -> Vec<&Tag> {
        self.tags.iter().collect()
    }
}

crate::unit_tests!("mod.test.rs");
