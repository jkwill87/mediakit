//! Parses a file path and prints inspection results as JSON.

use mediakit::inspect::{FilenameInspector, Inspector};
use std::path::PathBuf;

/// Retrieves file path from the first argument passed to the program.
///
/// Exits on error, printing usage instructions.
fn get_file_path_from_args() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let path = PathBuf::from(args[1].clone());
        path.canonicalize().unwrap_or(path)
    } else {
        let exec_name = std::env::current_exe()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        println!("Usage: {} <path>", exec_name);
        std::process::exit(1)
    }
}

fn main() {
    let path = &get_file_path_from_args();
    let filename_inspector = FilenameInspector::new(path).analyze();
    // let file_inspector = FileInspector::new(path).inspect();
    // let tags = filename_inspector.tags().extend(file_inspector.tags());
    println!("{:#?}", filename_inspector);
}
