//! Lists video, audio, and embedded subtitle tracks in a media file.

use mediakit::meta::fields::Language;
use mediakit::probe::{AudioTrack, FileProber, SubtitleTrack, TrackInfo, VideoTrack};
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn path_argument() -> Option<PathBuf> {
    let mut arguments = env::args_os().skip(1);
    match (arguments.next(), arguments.next()) {
        (Some(path), None) => Some(path.into()),
        _ => {
            eprintln!("Usage: cargo run --example detect-tracks -- <media-file>");
            None
        }
    }
}

fn append_track_info(fields: &mut Vec<String>, info: &TrackInfo) {
    if let Some(language) = info.language {
        fields.push(format_language(language));
    }
    if !info.is_enabled {
        fields.push("disabled".to_owned());
    }
    if info.is_default {
        fields.push("default".to_owned());
    }
}

fn format_language(language: Language) -> String {
    format!(
        "language={} ({}/{})",
        language.name, language.iso_639_1, language.iso_639_3
    )
}

fn print_video_tracks<'a>(tracks: impl Iterator<Item = &'a VideoTrack> + Clone) {
    println!("video tracks: {}", tracks.clone().count());
    for (index, track) in tracks.enumerate() {
        let mut fields = vec![
            track
                .codec
                .as_ref()
                .map_or_else(|| "unknown codec".to_owned(), ToString::to_string),
        ];
        if let Some(profile) = &track.profile {
            fields.push(profile.to_string());
        }
        if let (Some(width), Some(height)) = (track.width, track.height) {
            fields.push(format!("{width}x{height}"));
        }
        if let Some(resolution) = &track.resolution {
            fields.push(resolution.to_string());
        }
        if let Some(frame_rate) = track.frame_rate {
            fields.push(format!("{frame_rate:.3} fps"));
        }
        if let Some(dynamic_range) = &track.dynamic_range {
            fields.push(dynamic_range.to_string());
        }
        append_track_info(&mut fields, &track.info);
        println!("  [{index}] {}", fields.join(", "));
    }
}

fn print_audio_tracks<'a>(tracks: impl Iterator<Item = &'a AudioTrack> + Clone) {
    println!("audio tracks: {}", tracks.clone().count());
    for (index, track) in tracks.enumerate() {
        let mut fields = vec![
            track
                .codec
                .as_ref()
                .map_or_else(|| "unknown codec".to_owned(), ToString::to_string),
        ];
        if let Some(profile) = &track.profile {
            fields.push(profile.to_string());
        }
        if let Some(layout) = &track.layout {
            fields.push(layout.to_string());
        }
        if let Some(bit_rate) = track.bit_rate {
            fields.push(format!("{bit_rate} bps"));
        }
        append_track_info(&mut fields, &track.info);
        println!("  [{index}] {}", fields.join(", "));
    }
}

fn print_subtitle_tracks<'a>(tracks: impl Iterator<Item = &'a SubtitleTrack> + Clone) {
    println!("embedded subtitle tracks: {}", tracks.clone().count());
    for (index, track) in tracks.enumerate() {
        let mut fields = vec![
            track
                .codec
                .as_ref()
                .map_or_else(|| "unknown codec".to_owned(), ToString::to_string),
        ];
        append_track_info(&mut fields, &track.info);
        println!("  [{index}] {}", fields.join(", "));
    }
}

fn main() -> ExitCode {
    let Some(path) = path_argument() else {
        return ExitCode::FAILURE;
    };

    let media = match FileProber::new(&path).and_then(|prober| prober.probe()) {
        Ok(media) => media,
        Err(error) => {
            eprintln!("Cannot probe {}: {error}", path.display());
            return ExitCode::FAILURE;
        }
    };

    println!("path: {}", path.display());
    println!("container: {}", media.container);
    print_video_tracks(media.video_tracks());
    print_audio_tracks(media.audio_tracks());
    print_subtitle_tracks(media.subtitle_tracks());

    ExitCode::SUCCESS
}
