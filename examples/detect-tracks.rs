//! Detects external media tracks described by one or more filenames.

use mediakit::inspect::{FilenameInspector, Inspector};
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn path_arguments() -> Vec<PathBuf> {
    env::args_os().skip(1).map(PathBuf::from).collect()
}

fn main() -> ExitCode {
    let paths = path_arguments();
    if paths.is_empty() {
        eprintln!("Usage: cargo run --example detect-tracks -- <track-file> [<track-file> ...]");
        return ExitCode::FAILURE;
    }

    for path in paths {
        let inspector = FilenameInspector::new(&path).analyze();
        let Some(track) = &inspector.metadata.track else {
            println!("{}: no external track detected", path.display());
            continue;
        };

        println!("{}", path.display());
        println!(
            "  identity_stem: {}",
            inspector.metadata.identity_stem().unwrap_or("<none>")
        );
        println!(
            "  generic_identity: {}",
            inspector.metadata.has_generic_identity()
        );
        println!("  kind: {:?}", track.kind);
        if let Some(language) = track.language {
            println!(
                "  language: {} ({}, {})",
                language.name, language.iso_639_1, language.iso_639_3
            );
        }
        if let Some(number) = track.number {
            println!("  number: {number}");
        }
        for disposition in &track.dispositions {
            println!("  disposition: {disposition}");
        }
    }

    ExitCode::SUCCESS
}
