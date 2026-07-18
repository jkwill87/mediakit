//! Verifies bounded MPEG-TS detection and probing.

use super::*;

#[test]
fn identifies_packet_layouts() {
    let mut ts = vec![0; 188 * 5];
    for offset in (0..ts.len()).step_by(188) {
        ts[offset] = 0x47;
    }
    assert_eq!(detect(&ts).unwrap().size(), 188);

    let mut m2ts = vec![0; 192 * 5];
    for offset in (4..m2ts.len()).step_by(192) {
        m2ts[offset] = 0x47;
    }
    assert_eq!(detect(&m2ts).unwrap().size(), 192);

    let mut fec = vec![0; 204 * 5];
    for offset in (0..fec.len()).step_by(204) {
        fec[offset] = 0x47;
    }
    assert_eq!(detect(&fec).unwrap().size(), 204);
}

#[test]
fn descriptor_iterator_stops_at_malformed_trailing_data() {
    let descriptors = [0x05, 4, b'A', b'C', b'-', b'3', 0x59, 4, b'e'];
    let parsed = Descriptors::new(&descriptors).collect::<Vec<_>>();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].tag, DESCRIPTOR_REGISTRATION);
    assert_eq!(parsed[0].payload, b"AC-3");
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

#[test]
fn maps_embedded_subtitle_streams() {
    assert_eq!(subtitle_stream_type(0x90, &[]), Some("pgs"));
    assert_eq!(
        subtitle_stream_type(0x06, &[0x59, 0]),
        Some("dvb_subtitle")
    );
    assert_eq!(
        subtitle_stream_type(0x06, &[0x56, 0]),
        Some("teletext")
    );
    assert_eq!(
        descriptor_language(&[0x59, 4, b'p', b'o', b'r', 0])
            .map(|language| language.iso_639_1),
        Some("pt")
    );
}
