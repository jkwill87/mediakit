//! Inspects filesystem and container metadata for a media file.

use mediakit::inspect::{FileInspector, Inspector};
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn path_argument() -> Option<PathBuf> {
    let mut arguments = env::args_os().skip(1);
    match (arguments.next(), arguments.next()) {
        (Some(path), None) => Some(path.into()),
        _ => {
            eprintln!("Usage: cargo run --example inspect-file -- <media-file>");
            None
        }
    }
}

fn main() -> ExitCode {
    let Some(path) = path_argument() else {
        return ExitCode::FAILURE;
    };

    match path.metadata() {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_) => {
            eprintln!("Not a file: {}", path.display());
            return ExitCode::FAILURE;
        }
        Err(error) => {
            eprintln!("Cannot inspect {}: {error}", path.display());
            return ExitCode::FAILURE;
        }
    }

    let inspector = FileInspector::new(&path).analyze();

    println!("path: {}", path.display());
    for tag in inspector.tags() {
        println!("{}: {}", tag.key(), tag.value());
    }

    ExitCode::SUCCESS
}
