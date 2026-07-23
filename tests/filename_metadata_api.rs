//! Verifies the public structured filename-metadata contract.

use mediakit::inspect::{FilenameInspector, Inspector};
use mediakit::meta::fields::{MediaFormat, MediaType, TrackDisposition, TrackKind};

#[test]
fn structured_external_track_metadata_is_public() {
    let inspector = FilenameInspector::new("Rango.2011.pt-BR.2.forced.srt").analyze();
    let metadata = inspector.metadata();

    assert_eq!(inspector.filename(), "Rango.2011.pt-BR.2.forced.srt");
    assert!(!inspector.tokens().is_empty());
    assert_eq!(inspector.media_type(), MediaType::Movie);
    assert_eq!(metadata.format, Some(MediaFormat::Srt));
    assert_eq!(metadata.identity_stem(), Some("Rango.2011"));
    assert!(!metadata.has_generic_identity());

    let track = metadata.track.as_ref().expect("external track metadata");
    assert_eq!(track.kind, TrackKind::Subtitle);
    assert_eq!(
        track.language.map(|language| language.iso_639_1),
        Some("pt")
    );
    assert_eq!(track.number, Some(2));
    assert_eq!(track.dispositions.as_slice(), [TrackDisposition::Forced]);
}

#[test]
fn ordinary_media_uses_the_same_format_model_without_track_metadata() {
    let inspector = FilenameInspector::new("Rango.2011.mkv").analyze();
    let metadata = inspector.metadata();

    assert_eq!(metadata.format, Some(MediaFormat::Mkv));
    assert_eq!(metadata.track, None);
    assert_eq!(metadata.identity_stem(), Some("Rango.2011"));
}

#[cfg(unix)]
#[test]
fn non_utf8_external_tracks_keep_structured_format_metadata() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use std::path::PathBuf;

    let path = PathBuf::from(OsString::from_vec(b"rango-\xff.en.srt".to_vec()));
    let inspector = FilenameInspector::new(path).analyze();
    let metadata = inspector.metadata();

    assert_eq!(metadata.format, Some(MediaFormat::Srt));
    assert!(metadata.track.is_some());
    assert_eq!(metadata.identity_stem(), None);
}
