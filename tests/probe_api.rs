//! Verifies the public container-probing contract.

use mediakit::probe::{MediaInfo, ProbeError, probe};

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
