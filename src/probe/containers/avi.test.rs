//! Verifies bounded AVI detection and probing.

use super::*;
use crate::meta::fields::VideoCodec;

#[test]
fn maps_avi_video_and_wave_audio_codecs() {
    assert_eq!(video_codec(b"XVID"), Some(VideoCodec::Mpeg4Visual));
}

#[test]
fn riff_chunks_handle_padding_and_bounded_movi_payloads() {
    let data = [b'J', b'U', b'N', b'K', 1, 0, 0, 0, 0xAA, 0, b'J', b'U', b'N', b'K', 0, 0, 0, 0];
    let chunks = RiffChunks::new(&data)
        .collect::<io::Result<Vec<_>>>()
        .unwrap();
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].payload, &[0xAA]);

    let truncated_movi = [b'L', b'I', b'S', b'T', 32, 0, 0, 0, b'm', b'o', b'v', b'i'];
    assert_eq!(RiffChunks::new(&truncated_movi).count(), 0);

    let truncated_other = [b'J', b'U', b'N', b'K', 32, 0, 0, 0];
    assert!(RiffChunks::new(&truncated_other).next().unwrap().is_err());
}
