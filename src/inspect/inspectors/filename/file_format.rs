//! Inspects filename tokens for media file formats.

use super::FilenameInspector;
use crate::meta::Tag;
use crate::meta::fields::MediaFormat;

impl FilenameInspector {
    /// Selects the media file format from the filename extension.
    ///
    /// **Preconditions:**
    /// - None
    pub(super) fn inspect_file_format(mut self) -> Self {
        let Some(format) = self.file_format() else {
            return self;
        };

        let ntokens = self.tokens.len();
        if ntokens < 2 {
            return self;
        }
        let second_last_token = &self.tokens[ntokens - 2];
        let text = second_last_token.template(&self.filename);
        if text != "." {
            return self;
        }
        let last_token = &self.tokens[ntokens - 1];
        if MediaFormat::from_extension(last_token.template(&self.filename)) != Some(format) {
            return self;
        }
        self.tokens[ntokens - 1].tag = Some(Tag::FileFormat(format));
        self
    }
}

crate::unit_tests!("file_format.test.rs");
