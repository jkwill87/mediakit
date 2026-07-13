//! Probes supported media containers for bounded stream metadata.

pub mod error;
pub mod media_info;
mod parsers;

pub use error::ProbeError;
pub use media_info::{AudioStream, MediaInfo, VideoStream};

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

const ASF_HEADER_GUID: [u8; 16] = [
    0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C,
];

/// Probes a media file and returns structured container metadata.
///
/// Reads are bounded and media payloads are not decoded. Supported container
/// families are Matroska/WebM, ISO-BMFF/QuickTime, AVI, MPEG-TS/M2TS, and ASF/WMV.
pub fn probe<P: AsRef<Path>>(path: P) -> Result<MediaInfo, ProbeError> {
    let mut file = File::open(path.as_ref())?;
    let file_len = file.metadata()?.len();
    if file_len < 4 {
        return Err(ProbeError::UnsupportedFormat);
    }

    let mut header = [0_u8; 16];
    let count = file.read(&mut header)?;
    file.seek(SeekFrom::Start(0))?;
    let header = &header[..count];

    let (format, result) = if header.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
        (
            "Matroska/WebM",
            parsers::parse_matroska(&mut file, file_len),
        )
    } else if header.len() >= 12 && &header[..4] == b"RIFF" && &header[8..12] == b"AVI " {
        ("AVI", parsers::parse_avi(&mut file, file_len))
    } else if header.starts_with(&ASF_HEADER_GUID) {
        ("ASF", parsers::parse_asf(&mut file, file_len))
    } else if header.len() >= 8 && (&header[4..8] == b"ftyp" || &header[4..8] == b"moov") {
        ("ISO-BMFF", parsers::parse_mp4(&mut file, file_len))
    } else if parsers::matches_mpeg_ts(&mut file, file_len)? {
        ("MPEG-TS", parsers::parse_mpeg_ts(&mut file, file_len))
    } else {
        return Err(ProbeError::UnsupportedFormat);
    };

    result.map_err(|error| ProbeError::from_parser(format, error))
}

crate::unit_tests!("mod.test.rs");
