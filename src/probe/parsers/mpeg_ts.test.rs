//! Verifies bounded MPEG-TS and M2TS stream parsing.

use super::*;

#[test]
fn identifies_packet_layouts() {
    let mut ts = vec![0; 188 * 5];
    for offset in (0..ts.len()).step_by(188) {
        ts[offset] = 0x47;
    }
    assert_eq!(detect_layout(&ts).unwrap().size, 188);

    let mut m2ts = vec![0; 192 * 5];
    for offset in (4..m2ts.len()).step_by(192) {
        m2ts[offset] = 0x47;
    }
    assert_eq!(detect_layout(&m2ts).unwrap().size, 192);
}

#[test]
fn maps_private_stream_descriptors() {
    assert_eq!(
        descriptor_audio_codec(&[0x6A, 0]),
        Some(AudioCodec::DolbyDigital)
    );
    assert_eq!(
        descriptor_audio_codec(&[0x05, 4, b'E', b'A', b'C', b'3']),
        Some(AudioCodec::DolbyDigitalPlus)
    );
}
