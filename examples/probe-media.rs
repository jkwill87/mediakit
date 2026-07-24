//! Probes a media container and prints tracks in global container order.

use mediakit::probe::{FileProber, ProbeError, Track};
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn path_argument() -> Option<PathBuf> {
    let mut arguments = env::args_os().skip(1);
    match (arguments.next(), arguments.next()) {
        (Some(path), None) => Some(path.into()),
        _ => {
            eprintln!("Usage: cargo run --example probe-media -- <media-file>");
            None
        }
    }
}

fn main() -> ExitCode {
    let Some(path) = path_argument() else {
        return ExitCode::FAILURE;
    };

    let media = match FileProber::new(&path).and_then(|prober| prober.probe()) {
        Ok(media) => media,
        Err(ProbeError::UnsupportedFormat) => {
            eprintln!("Unsupported media format: {}", path.display());
            return ExitCode::FAILURE;
        }
        Err(ProbeError::InvalidData { format, message }) => {
            eprintln!("Invalid {format} data in {}: {message}", path.display());
            return ExitCode::FAILURE;
        }
        Err(ProbeError::Io(error)) => {
            eprintln!("Cannot read {}: {error}", path.display());
            return ExitCode::FAILURE;
        }
        Err(error) => {
            eprintln!("Cannot probe {}: {error}", path.display());
            return ExitCode::FAILURE;
        }
    };

    println!("path: {}", path.display());
    println!("container: {}", media.container);
    if let Some(duration) = media.duration {
        println!("duration: {:.3}s", duration.as_secs_f64());
    }

    for (index, track) in media.tracks.iter().enumerate() {
        let primary = match track {
            Track::Audio(track) => media
                .primary_audio_track()
                .is_some_and(|primary| std::ptr::eq(track, primary)),
            Track::Video(track) => media
                .primary_video_track()
                .is_some_and(|primary| std::ptr::eq(track, primary)),
            Track::Subtitle(track) => media
                .primary_subtitle_track()
                .is_some_and(|primary| std::ptr::eq(track, primary)),
            _ => false,
        };
        println!("track[{index}] primary={primary}: {track:#?}");
    }

    ExitCode::SUCCESS
}
