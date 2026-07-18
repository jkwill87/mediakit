//! Owns shared file state and bounded reads used during detection and probing.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// Open media-file state shared by the selected container implementation.
pub(super) struct ProbeInput {
    /// Open media file.
    file: File,
    /// File length captured when the input is initialized.
    len: u64,
}

impl ProbeInput {
    /// Opens a media file and captures its current length.
    pub(super) fn open(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let len = file.metadata()?.len();
        Ok(Self { file, len })
    }

    /// Returns the captured file length.
    pub(super) const fn len(&self) -> u64 {
        self.len
    }

    /// Returns the underlying file for seek-based container parsing.
    pub(super) const fn file(&mut self) -> &mut File {
        &mut self.file
    }

    /// Reads a bounded leading region into caller-owned detection storage.
    pub(super) fn read_prefix<'a>(&mut self, buffer: &'a mut [u8]) -> io::Result<&'a [u8]> {
        let capacity = u64::try_from(buffer.len()).expect("detection prefix fits in u64");
        let read_len = usize::try_from(self.len.min(capacity))
            .expect("bounded detection prefix fits in usize");
        self.file.seek(SeekFrom::Start(0))?;
        self.file.read_exact(&mut buffer[..read_len])?;
        self.file.seek(SeekFrom::Start(0))?;
        Ok(&buffer[..read_len])
    }
}
