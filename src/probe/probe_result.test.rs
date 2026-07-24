//! Verifies ordered views and primary-track selection.

use super::*;

#[test]
fn typed_views_preserve_cross_kind_order() {
    let mut result = ProbeResult::new(MediaFormat::Mkv);
    result.tracks = vec![
        Track::Video(VideoTrack::default()),
        Track::Audio(AudioTrack {
            bit_rate: Some(1),
            ..AudioTrack::default()
        }),
        Track::Subtitle(SubtitleTrack::default()),
        Track::Audio(AudioTrack {
            bit_rate: Some(2),
            ..AudioTrack::default()
        }),
    ];

    assert_eq!(result.video_tracks().count(), 1);
    assert_eq!(result.audio_tracks().count(), 2);
    assert_eq!(result.subtitle_tracks().count(), 1);
    assert_eq!(
        result
            .audio_tracks()
            .map(|track| track.bit_rate.unwrap())
            .collect::<Vec<_>>(),
        [1, 2]
    );
}

#[test]
fn primary_track_prefers_default_then_enabled_then_first() {
    let mut result = ProbeResult::new(MediaFormat::Mkv);
    result.tracks = vec![
        Track::Audio(AudioTrack {
            info: TrackInfo {
                is_enabled: false,
                ..TrackInfo::default()
            },
            ..AudioTrack::default()
        }),
        Track::Video(VideoTrack::default()),
        Track::Audio(AudioTrack::default()),
        Track::Subtitle(SubtitleTrack {
            info: TrackInfo {
                is_default: true,
                ..TrackInfo::default()
            },
            ..SubtitleTrack::default()
        }),
        Track::Audio(AudioTrack {
            info: TrackInfo {
                is_default: true,
                ..TrackInfo::default()
            },
            ..AudioTrack::default()
        }),
    ];

    let audio: Vec<_> = result.audio_tracks().collect();
    assert!(std::ptr::eq(result.primary_audio_track().unwrap(), audio[2]));
    assert!(result.primary_video_track().is_some());
    assert!(result.primary_subtitle_track().is_some());

    if let Track::Audio(track) = &mut result.tracks[4] {
        track.info.is_enabled = false;
    }
    let audio: Vec<_> = result.audio_tracks().collect();
    assert!(std::ptr::eq(result.primary_audio_track().unwrap(), audio[1]));

    if let Track::Audio(track) = &mut result.tracks[2] {
        track.info.is_enabled = false;
    }
    let audio: Vec<_> = result.audio_tracks().collect();
    assert!(std::ptr::eq(result.primary_audio_track().unwrap(), audio[0]));
}

#[test]
fn primary_accessors_select_independently_across_interleaved_tracks() {
    let mut result = ProbeResult::new(MediaFormat::Mkv);
    result.tracks = vec![
        Track::Video(VideoTrack {
            info: TrackInfo {
                is_enabled: false,
                ..TrackInfo::default()
            },
            ..VideoTrack::default()
        }),
        Track::Audio(AudioTrack {
            info: TrackInfo {
                is_default: true,
                ..TrackInfo::default()
            },
            ..AudioTrack::default()
        }),
        Track::Subtitle(SubtitleTrack {
            info: TrackInfo {
                is_enabled: false,
                ..TrackInfo::default()
            },
            ..SubtitleTrack::default()
        }),
        Track::Video(VideoTrack::default()),
        Track::Audio(AudioTrack::default()),
        Track::Subtitle(SubtitleTrack {
            info: TrackInfo {
                is_default: true,
                ..TrackInfo::default()
            },
            ..SubtitleTrack::default()
        }),
    ];

    let Track::Audio(default_audio) = &result.tracks[1] else {
        unreachable!()
    };
    let Track::Video(enabled_video) = &result.tracks[3] else {
        unreachable!()
    };
    let Track::Subtitle(default_subtitle) = &result.tracks[5] else {
        unreachable!()
    };
    assert!(std::ptr::eq(
        result.primary_audio_track().unwrap(),
        default_audio
    ));
    assert!(std::ptr::eq(
        result.primary_video_track().unwrap(),
        enabled_video
    ));
    assert!(std::ptr::eq(
        result.primary_subtitle_track().unwrap(),
        default_subtitle
    ));

    if let Track::Video(track) = &mut result.tracks[3] {
        track.info.is_enabled = false;
    }
    let Track::Video(first_video) = &result.tracks[0] else {
        unreachable!()
    };
    assert!(std::ptr::eq(
        result.primary_video_track().unwrap(),
        first_video
    ));
}
