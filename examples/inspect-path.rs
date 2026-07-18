//! Inspects metadata encoded in a media path's filename.

use mediakit::inspect::{FilenameInspector, Inspector};
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn path_argument() -> Option<PathBuf> {
    let mut arguments = env::args_os().skip(1);
    match (arguments.next(), arguments.next()) {
        (Some(path), None) => Some(path.into()),
        _ => {
            eprintln!("Usage: cargo run --example inspect-path -- <path>");
            None
        }
    }
}

fn main() -> ExitCode {
    let Some(path) = path_argument() else {
        return ExitCode::FAILURE;
    };

    let inspector = FilenameInspector::new(&path).analyze();

    println!("path: {}", path.display());
    println!("media_type: {}", inspector.media_type());
    for tag in inspector.tags() {
        println!("{}: {}", tag.key(), tag.value());
    }

    ExitCode::SUCCESS
}
