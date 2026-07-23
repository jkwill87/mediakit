//! Verifies ordered filename inspection and structured metadata.

use super::*;
use crate::inspect::Inspector;
use crate::meta::fields::MediaType;

fn values(inspector: &FilenameInspector, key: &str) -> Vec<String> {
    inspector
        .tags()
        .into_iter()
        .filter(|tag| tag.key() == key)
        .map(Tag::value)
        .collect()
}

fn assert_value(inspector: &FilenameInspector, key: &str, expected: &str) {
    assert!(
        values(inspector, key).iter().any(|value| value == expected),
        "missing {key}={expected} in {inspector:#?}"
    );
}

fn assert_stable_spans(inspector: &FilenameInspector) {
    assert_eq!(inspector.tokens.first().map(|token| token.start), Some(0));
    assert_eq!(
        inspector.tokens.last().map(|token| token.end),
        Some(inspector.filename.len())
    );
    for tokens in inspector.tokens.windows(2) {
        assert_eq!(tokens[0].end, tokens[1].start, "{inspector:#?}");
    }
    for token in &inspector.tokens {
        assert!(token.start < token.end);
        assert!(inspector.filename.get(token.start..token.end).is_some());
    }
}

#[test]
fn python_mnamer_episode_fixture() {
    let inspector =
        FilenameInspector::new("ninja.turtles.s01e04.1080p.ac3.rargb.sample.mkv").analyze();
    assert_eq!(inspector.media_type(), MediaType::Television);
    assert_value(&inspector, "title", "ninja turtles");
    assert_value(&inspector, "season_number", "1");
    assert_value(&inspector, "episode_number", "4");
    assert_value(&inspector, "video_resolution", "1080p");
    assert_value(&inspector, "audio_codec", "dolby_digital");
    assert_value(&inspector, "release_group", "rargb");
    assert_stable_spans(&inspector);
}

#[test]
fn date_based_episode_fixture() {
    let inspector =
        FilenameInspector::new("Frontline.2024.04.23.1080p.WEB.H264-FLAME.mkv").analyze();
    assert_eq!(inspector.media_type(), MediaType::Television);
    assert_value(&inspector, "title", "Frontline");
    assert_value(&inspector, "air_date", "2024-04-23");
    assert_stable_spans(&inspector);
}

#[test]
fn alternate_title_and_ep_notation_fixture() {
    let inspector = FilenameInspector::new(
        "Atlantas Missing and Murdered - The Lost Children S01EP03 (2020) (1080p).mp4",
    )
    .analyze();
    assert_eq!(inspector.media_type(), MediaType::Television);
    assert_value(&inspector, "title", "Atlantas Missing and Murdered");
    assert_value(&inspector, "alternative_title", "The Lost Children");
    assert_value(&inspector, "season_number", "1");
    assert_value(&inspector, "episode_number", "3");
    assert_value(&inspector, "year", "2020");
    assert_stable_spans(&inspector);
}

#[test]
fn technical_profile_fixture() {
    let inspector = FilenameInspector::new(
        "Pulp.Fiction.1994.1080p.BluRay.DTS-HD.MA.5.1.H.264.High-ARCHiViST.mkv",
    )
    .analyze();
    assert_eq!(inspector.media_type(), MediaType::Movie);
    assert_value(&inspector, "title", "Pulp Fiction");
    assert_value(&inspector, "year", "1994");
    assert_value(&inspector, "audio_codec", "dts_hd");
    assert_value(&inspector, "audio_profile", "master_audio");
    assert_value(&inspector, "audio_layout", "5.1");
    assert_value(&inspector, "video_codec", "h264");
    assert_value(&inspector, "video_profile", "high");
    assert_stable_spans(&inspector);
}

#[test]
fn multi_episode_and_subtitle_language_fixtures() {
    let episode = FilenameInspector::new(
        "Dark.Matter.2024.S01E01E02.Are.You.Happy.in.Your.Life.720p-ARCHiViST.mp4",
    )
    .analyze();
    assert_eq!(values(&episode, "episode_number"), ["1", "2"]);
    assert_value(&episode, "episode_title", "Are You Happy in Your Life");
    assert_stable_spans(&episode);

    let subtitle = FilenameInspector::new("Rupauls.Drag.Race.S01E02.fr.srt").analyze();
    assert_value(&subtitle, "subtitle_language", "fr");
    assert_stable_spans(&subtitle);
}
