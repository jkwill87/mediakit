//! Verifies the public container-probing contract.

use mediakit::meta::fields::Language;
use mediakit::probe::{MediaInfo, ProbeError, SubtitleStream, probe};

#[test]
fn probe_api_is_available_to_external_callers() {
    fn assert_result_type(_: Result<MediaInfo, ProbeError>) {}

    let missing = std::env::temp_dir().join(format!(
        "mediakit-public-probe-missing-{}",
        std::process::id()
    ));
    let result = probe(missing);
    assert_result_type(result);

    let result = probe(std::env::temp_dir().join(format!(
        "mediakit-public-probe-missing-{}",
        std::process::id()
    )));
    assert!(matches!(result, Err(ProbeError::Io(_))));
}

#[test]
fn embedded_subtitle_stream_type_is_public() {
    fn assert_language_type(_: Option<Language>) {}

    let stream = SubtitleStream::default();

    assert!(stream.is_enabled);
    assert!(!stream.is_default);
    assert_language_type(stream.language);
    assert_eq!(stream.language, None);
    assert_eq!(stream.codec, None);
}
