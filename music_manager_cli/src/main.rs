extern crate quicli;
extern crate structopt;
extern crate walkdir;

// use id3::Tag;
use quicli::prelude::*;
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

use std::path::Path;

fn main() -> CliResult {
    let args = Cli::from_args();

    println!("Reading .mp3's from directory: {:?}", args.directory);

    if !Path::new(&args.directory).exists() {
        warn!("Error: {:?} is not a valid path", args.directory);
    }

    let total_files = WalkDir::new(&args.directory).into_iter().count();
    let mut file_count = 0;

    WalkDir::new(&args.directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| get_mp3_file_paths(&e))
        .for_each(|e| {
            let progress = (file_count as f32 / total_files as f32) * 100.0;
            println!(
                "Count: {}, Progress: {}. File: {:?}",
                &file_count, progress, &e
            );
            file_count += 1;
        });

    Ok(())
}

/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    // The absolute filepath you want to import all mp3s
    directory: String,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

/// Returns the file path if it's a .mp3 file or None.
pub fn get_mp3_file_paths(entry: &DirEntry) -> Option<String> {
    match entry.path().extension() {
        Some(ext) => match ext.to_str() {
            Some(exxt) if exxt == "mp3" => match entry.path().to_str() {
                Some(p) => Some(p.to_string()),
                None => None,
            },
            Some(_) => None,
            None => None,
        },
        None => None,
    }
}


