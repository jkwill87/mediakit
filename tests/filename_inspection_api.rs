//! Verifies the public flat filename-inspection contract.

use mediakit::inspect::{FilenameInspector, Inspector};
use mediakit::meta::Tag;
use mediakit::meta::fields::{LanguageTag, MediaFormat, MediaType, SubtitleDisposition};

#[test]
fn subtitle_metadata_is_exposed_as_flat_tags() {
    let inspector = FilenameInspector::new("Rango.2011.pt-BR.2.forced.srt").analyze();

    assert_eq!(inspector.filename(), "Rango.2011.pt-BR.2.forced.srt");
    assert!(!inspector.tokens().is_empty());
    assert_eq!(inspector.media_type(), MediaType::Movie);
    assert_eq!(inspector.identity_stem(), Some("Rango.2011"));
    assert!(
        inspector
            .tags()
            .into_iter()
            .any(|tag| matches!(tag, Tag::FileFormat(MediaFormat::Srt)))
    );
    assert!(inspector.tags().into_iter().any(|tag| matches!(
        tag,
        Tag::SubtitleLanguage(LanguageTag::Language(language)) if language.iso_639_1 == "pt"
    )));
    assert!(
        inspector
            .tags()
            .into_iter()
            .all(|tag| tag.key() != "subtitle_track_number")
    );
    assert!(
        inspector
            .tags()
            .into_iter()
            .any(|tag| matches!(tag, Tag::SubtitleDisposition(SubtitleDisposition::Forced)))
    );
}

#[test]
fn ordinary_media_retains_only_direct_identity_accessors() {
    let inspector = FilenameInspector::new("Rango.2011.mkv").analyze();

    assert_eq!(inspector.identity_stem(), Some("Rango.2011"));
    assert!(
        inspector
            .tags()
            .into_iter()
            .any(|tag| matches!(tag, Tag::FileFormat(MediaFormat::Mkv)))
    );
    assert!(
        !inspector
            .tags()
            .into_iter()
            .any(|tag| matches!(tag, Tag::SubtitleLanguage(_) | Tag::SubtitleDisposition(_)))
    );
}

#[test]
fn generic_subtitle_names_have_no_identity() {
    let inspector = FilenameInspector::new("English.srt").analyze();
    assert_eq!(inspector.identity_stem(), None);
}

#[cfg(unix)]
#[test]
fn non_utf8_filenames_are_ignored_without_panicking() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use std::path::PathBuf;

    let path = PathBuf::from(OsString::from_vec(b"rango-\xff.en.srt".to_vec()));
    let inspector = FilenameInspector::new(path).analyze();

    assert_eq!(inspector.filename(), "");
    assert!(inspector.tokens().is_empty());
    assert!(inspector.tags().is_empty());
    assert_eq!(inspector.identity_stem(), None);
}
