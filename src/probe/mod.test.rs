//! Verifies container detection and probe error handling.

use super::*;
use crate::meta::fields::{AudioCodec, VideoCodec};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

struct Fixture {
    path: PathBuf,
}

impl Fixture {
    fn new(extension: &str, data: &[u8]) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("mediakit-probe-{nonce}.{extension}"));
        fs::write(&path, data).unwrap();
        Self { path }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn ts_packet(pid: u16, unit_start: bool, payload: &[u8]) -> Vec<u8> {
    let mut packet = vec![0xFF; 188];
    packet[0] = 0x47;
    packet[1] = u8::try_from(pid >> 8).unwrap() & 0x1F;
    if unit_start {
        packet[1] |= 0x40;
    }
    packet[2] = pid as u8;
    packet[3] = 0x10;
    let mut offset = 4;
    if unit_start {
        packet[offset] = 0;
        offset += 1;
    }
    packet[offset..offset + payload.len()].copy_from_slice(payload);
    packet
}

fn ts_fixture() -> Vec<u8> {
    let pat = [
        0x00, 0xB0, 0x0D, 0, 1, 0xC1, 0, 0, 0, 1, 0xE1, 0, 0, 0, 0, 0,
    ];
    let pmt = [
        0x02, 0xB0, 0x21, 0, 1, 0xC1, 0, 0, 0xE1, 1, 0xF0, 0, 0x24, 0xE1, 1, 0xF0, 0, 0x1B, 0xE1,
        2, 0xF0, 0, 0x87, 0xE1, 3, 0xF0, 0, 0x90, 0xE1, 4, 0xF0, 0, 0, 0, 0, 0,
    ];
    let mut output = ts_packet(0, true, &pat);
    output.extend_from_slice(&ts_packet(0x100, true, &pmt));
    for _ in 0..3 {
        output.extend_from_slice(&ts_packet(0x1FFF, false, &[]));
    }
    output
}

#[test]
fn public_probe_enumerates_transport_stream_tracks() {
    let fixture = Fixture::new("ts", &ts_fixture());
    let info = probe(&fixture.path).unwrap();
    assert_eq!(info.container, "ts");
    assert_eq!(info.video_streams.len(), 2);
    assert_eq!(info.audio_streams.len(), 1);
    assert_eq!(info.subtitle_streams.len(), 1);
    assert_eq!(info.video_streams[0].codec, Some(VideoCodec::H265));
    assert_eq!(info.video_streams[1].codec, Some(VideoCodec::H264));
    assert_eq!(
        info.audio_streams[0].codec,
        Some(AudioCodec::DolbyDigitalPlus)
    );
    assert_eq!(info.subtitle_streams[0].codec.as_deref(), Some("pgs"));
}

#[test]
fn probe_errors_distinguish_unsupported_invalid_and_io() {
    let unsupported = Fixture::new("bin", b"not a supported media file");
    assert!(matches!(
        probe(&unsupported.path),
        Err(ProbeError::UnsupportedFormat)
    ));

    let invalid = Fixture::new("mp4", b"\0\0\0\x10ftypisom");
    assert!(matches!(
        probe(&invalid.path),
        Err(ProbeError::InvalidData {
            format: "ISO-BMFF",
            ..
        })
    ));

    let truncated = Fixture::new("mp4", b"\0\0\0\x08ftyp\0\0\0\0");
    assert!(matches!(
        probe(&truncated.path),
        Err(ProbeError::InvalidData {
            format: "ISO-BMFF",
            ..
        })
    ));

    let missing = std::env::temp_dir().join("mediakit-probe-file-does-not-exist");
    assert!(matches!(probe(missing), Err(ProbeError::Io(_))));
}
