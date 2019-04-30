extern crate quicli;
extern crate structopt;

use quicli::prelude::*;
use structopt::StructOpt;


// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: usize,
    // Add a positional argument that the user has to supply:
    /// The file to read
    file: String,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();

    read_file(&args.file)?
        .lines()
        .take(args.count)
        .for_each(|line| println!("{}", line));
    
    Ok(())
}

// extern crate diesel;
// extern crate id3;
// extern crate music_manager_lib;
// #[macro_use]
// // extern crate quicli;
// extern crate walkdir;

// use self::music_manager_lib::*;

// use diesel::dsl::exists;
// use diesel::prelude::*;
// use diesel::select;
// use id3::Tag;
// // use quicli::prelude::*;
// use std::path::Path;
// use walkdir::{DirEntry, WalkDir};

// main!(|args: Cli, log_level: verbosity| {
//     use music_manager_lib::schema::files::dsl::*;
//     use music_manager_lib::schema::frames::dsl::*;
//     use music_manager_lib::schema::id3_tags::dsl::*;

//     let connection = establish_connection();

//     info!("Reading .mp3's from directory: {:?}", args.directory);

//     if !Path::new(&args.directory).exists() {
//         warn!("Error: {:?} is not a valid path", args.directory);
//     }

//     let total_files = WalkDir::new(&args.directory).into_iter().count();
//     let mut file_count = 0;

//     WalkDir::new(&args.directory)
//         .into_iter()
//         .filter_map(|e| e.ok())
//         .filter_map(|e| get_mp3_file_paths(&e))
//         .for_each(|e| {
//             let existing_file = select(exists(files.filter(path.eq(&e))))
//                 .get_result::<bool>(&connection)
//                 .unwrap();

//             if existing_file {
//                 info!("File already in library: {:?}", &e);
//             } else {
//                 let tag = Tag::read_from_path(&e);

//                 let new_file = models::file::NewFile::new(&e);

//                 let file = diesel::insert_into(files)
//                     .values(&new_file)
//                     .get_result::<models::file::File>(&connection)
//                     .expect("Error inserting file");

//                 let new_tag = models::tag::NewTag::new(3, file.id);

//                 let inserted_tag = diesel::insert_into(id3_tags)
//                     .values(&new_tag)
//                     .get_result::<models::tag::Tag>(&connection)
//                     .expect("error inserting id3 tag");

//                 if tag.is_ok() {
//                     let safe_tag = tag.unwrap();
//                     let artist = safe_tag
//                         .artist()
//                         .map_or(String::new(), |artist| artist.to_string());
//                     let title = safe_tag
//                         .title()
//                         .map_or(String::new(), |title| title.to_string());
//                     let album = safe_tag
//                         .album()
//                         .map_or(String::new(), |album| album.to_string());

//                     diesel::insert_into(frames)
//                         .values(&vec![
//                             (
//                                 id3_tag_id.eq(&inserted_tag.id),
//                                 frame_type_id.eq(2),
//                                 content.eq(artist),
//                             ),
//                             (
//                                 id3_tag_id.eq(&inserted_tag.id),
//                                 frame_type_id.eq(6),
//                                 content.eq(title),
//                             ),
//                             (
//                                 id3_tag_id.eq(&inserted_tag.id),
//                                 frame_type_id.eq(4),
//                                 content.eq(album),
//                             ),
//                         ])
//                         .execute(&connection)
//                         .unwrap();
//                 }

//                 let progress = (file_count as f32 / total_files as f32) * 100.0;
//                 println!(
//                     "Count: {}, Progress: {}. File: {:?}",
//                     &file_count, progress, &e
//                 );
//                 file_count += 1;
//             }
//         });
// });

// /// Returns the file path if it's a .mp3 file or None.
// pub fn get_mp3_file_paths(entry: &DirEntry) -> Option<String> {
//     match entry.path().extension() {
//         Some(ext) => match ext.to_str() {
//             Some(exxt) if exxt == "mp3" => match entry.path().to_str() {
//                 Some(p) => Some(p.to_string()),
//                 None => None,
//             },
//             Some(_) => None,
//             None => None,
//         },
//         None => None,
//     }
// }

// // Add cool slogan for your app here, e.g.:
// /// Get first n lines of a file
// #[derive(Debug, StructOpt)]
// struct Cli {
//     // The absolute filepath you want to import all mp3s
//     directory: String,

//     #[structopt(flatten)]
//     verbosity: Verbosity,
// }

