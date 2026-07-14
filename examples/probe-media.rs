//! Probes a media container and prints its ordered stream metadata.

use mediakit::probe::{ProbeError, probe};
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

    let media = match probe(&path) {
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

    let primary_audio = media.primary_audio_stream();
    for (index, stream) in media.audio_streams.iter().enumerate() {
        let primary = primary_audio.is_some_and(|primary| std::ptr::eq(stream, primary));
        println!("audio_stream[{index}] primary={primary}: {stream:#?}");
    }

    let primary_video = media.primary_video_stream();
    for (index, stream) in media.video_streams.iter().enumerate() {
        let primary = primary_video.is_some_and(|primary| std::ptr::eq(stream, primary));
        println!("video_stream[{index}] primary={primary}: {stream:#?}");
    }

    for (index, stream) in media.subtitle_streams.iter().enumerate() {
        println!("subtitle_stream[{index}]: {stream:#?}");
    }

    ExitCode::SUCCESS
}
