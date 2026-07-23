//! Verifies container detection and probe error handling.

use super::*;
use crate::meta::fields::{AudioCodec, MediaFormat, SubtitleCodec, VideoCodec};
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

fn probe_data(extension: &str, data: &[u8]) -> Result<MediaInfo, ProbeError> {
    let fixture = Fixture::new(extension, data);
    FileProber::new(&fixture.path).and_then(FileProber::probe)
}

#[test]
fn file_prober_enumerates_transport_stream_tracks() {
    let info = probe_data("ts", &ts_fixture()).unwrap();
    assert_eq!(info.container, MediaFormat::Ts);
    assert_eq!(info.video_streams.len(), 2);
    assert_eq!(info.audio_streams.len(), 1);
    assert_eq!(info.subtitle_streams.len(), 1);
    assert_eq!(info.video_streams[0].codec, Some(VideoCodec::H265));
    assert_eq!(info.video_streams[1].codec, Some(VideoCodec::H264));
    assert_eq!(
        info.audio_streams[0].codec,
        Some(AudioCodec::DolbyDigitalPlus)
    );
    assert_eq!(
        info.subtitle_streams[0].codec,
        Some(SubtitleCodec::Pgs)
    );
}

#[test]
fn probe_errors_distinguish_unsupported_invalid_and_io() {
    assert!(matches!(
        probe_data("bin", b"not a supported media file"),
        Err(ProbeError::UnsupportedFormat)
    ));

    assert!(matches!(
        probe_data("mp4", b"\0\0\0\x10ftypisom"),
        Err(ProbeError::InvalidData {
            format: MediaFormat::Mp4,
            ..
        })
    ));

    assert!(matches!(
        probe_data("mp4", b"\0\0\0\x08ftyp\0\0\0\0"),
        Err(ProbeError::InvalidData {
            format: MediaFormat::Mp4,
            ..
        })
    ));

    assert!(matches!(
        probe_data(
            "webm",
            b"\x1A\x45\xDF\xA3\x87\x42\x82\x84webm"
        ),
        Err(ProbeError::InvalidData {
            format: MediaFormat::Webm,
            ..
        })
    ));

    assert!(matches!(
        probe_data("mov", b"\0\0\0\x14ftypqt  \0\0\0\0isom"),
        Err(ProbeError::InvalidData {
            format: MediaFormat::Mov,
            ..
        })
    ));

    let missing = std::env::temp_dir().join("mediakit-probe-file-does-not-exist");
    assert!(matches!(
        FileProber::new(missing),
        Err(ProbeError::Io(_))
    ));
}

fn ts_signature(packet_size: usize, sync_offset: usize) -> Vec<u8> {
    let mut signature = vec![0_u8; packet_size * 5];
    for packet in 0..5 {
        signature[sync_offset + packet * packet_size] = 0x47;
    }
    signature
}

#[test]
fn avi_probe_precedes_mpeg_ts_when_signature_matches_both() {
    let mut signature = ts_signature(192, 4);
    signature[..4].copy_from_slice(b"RIFF");
    signature[8..12].copy_from_slice(b"AVI ");

    let info = probe_data("avi", &signature).unwrap();
    assert_eq!(info.container, MediaFormat::Avi);
}

#[test]
fn complete_signatures_dispatch_to_the_expected_probe() {
    let signatures = [
        (MediaFormat::Mkv, vec![0x1A, 0x45, 0xDF, 0xA3]),
        (
            MediaFormat::Wmv,
            vec![
                0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA,
                0x00, 0x62, 0xCE, 0x6C,
            ],
        ),
        (MediaFormat::Mp4, b"\0\0\0\x08ftyp".to_vec()),
        (MediaFormat::Ts, ts_signature(188, 0)),
    ];

    for (format, signature) in signatures {
        assert!(
            matches!(
                probe_data("bin", &signature),
                Err(ProbeError::InvalidData {
                    format: actual,
                    ..
                }) if actual == format
            ),
            "{format}"
        );
    }

    let avi = probe_data("avi", b"RIFF\0\0\0\0AVI ").unwrap();
    assert_eq!(avi.container, MediaFormat::Avi);
}

#[test]
fn incomplete_and_near_match_signatures_are_unsupported() {
    let signatures = [
        (vec![0x1A, 0x45, 0xDF, 0xA3], 0, 3),
        (b"RIFF\0\0\0\0AVI ".to_vec(), 8, 11),
        (
            vec![
                0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA,
                0x00, 0x62, 0xCE, 0x6C,
            ],
            0,
            15,
        ),
        (b"\0\0\0\x08ftyp".to_vec(), 4, 7),
        (ts_signature(188, 0), 0, 752),
    ];

    for (signature, mismatch_index, short_len) in signatures {
        assert!(matches!(
            probe_data("bin", &signature[..short_len]),
            Err(ProbeError::UnsupportedFormat)
        ));
        let mut near_match = signature;
        near_match[mismatch_index] ^= 0xFF;
        assert!(matches!(
            probe_data("bin", &near_match),
            Err(ProbeError::UnsupportedFormat)
        ));
    }
}

#[test]
fn transport_probe_detects_every_supported_packet_layout() {
    for (signature, format) in [
        (ts_signature(188, 0), MediaFormat::Ts),
        (ts_signature(192, 4), MediaFormat::M2ts),
        (ts_signature(204, 0), MediaFormat::Ts),
    ] {
        assert!(matches!(
            probe_data("ts", &signature),
            Err(ProbeError::InvalidData {
                format: actual,
                ..
            }) if actual == format
        ));
    }
}
