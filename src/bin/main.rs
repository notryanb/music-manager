extern crate diesel;
extern crate id3;
extern crate music_manager_lib;
extern crate progress;
#[macro_use]
extern crate quicli;
extern crate walkdir;

use self::music_manager_lib::*;

use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::select;
use id3::Tag;
use quicli::prelude::*;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

main!(|args: Cli, log_level: verbosity| {

    match args.cmd {
        Command::Import { directory } => {
            use music_manager_lib::schema::files::dsl::*;
            use music_manager_lib::schema::frames::dsl::*;
            use music_manager_lib::schema::id3_tags::dsl::*;

            let connection = establish_connection();

            info!("Reading .mp3's from directory: {:?}", directory);

            if !Path::new(&directory).exists() {
                warn!("Error: {:?} is not a valid path", directory);
            }

            let total_files = WalkDir::new(&directory)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter_map(|e| get_mp3_file_paths(&e))
                .count();

            let mut file_count = 0;

            let mut bar = progress::Bar::new();
            bar.set_job_title("Importing files...");

            WalkDir::new(&directory)
                .into_iter()
                .filter_map(|dir| dir.ok())
                .filter_map(|dir| get_mp3_file_paths(&dir))
                .for_each(|file_path| {
                    let existing_file = select(exists(files.filter(path.eq(&file_path))))
                        .get_result::<bool>(&connection)
                        .unwrap();

                    if !existing_file {
                        let tag = Tag::read_from_path(&file_path);

                        let new_file = models::file::NewFile::new(&file_path);

                        let file = diesel::insert_into(files)
                            .values(&new_file)
                            .get_result::<models::file::File>(&connection)
                            .expect("Error inserting file");

                        let new_tag = models::tag::NewTag::new(3, file.id);

                        let inserted_tag = diesel::insert_into(id3_tags)
                            .values(&new_tag)
                            .get_result::<models::tag::Tag>(&connection)
                            .expect("error inserting id3 tag");

                        if tag.is_ok() {
                            let safe_tag = tag.unwrap();
                            let artist = safe_tag
                                .artist()
                                .map_or(String::new(), |artist| artist.to_string());
                            let title = safe_tag
                                .title()
                                .map_or(String::new(), |title| title.to_string());
                            let album = safe_tag
                                .album()
                                .map_or(String::new(), |album| album.to_string());

                            diesel::insert_into(frames)
                                .values(&vec![
                                    (
                                        id3_tag_id.eq(&inserted_tag.id),
                                        frame_type_id.eq(2),
                                        content.eq(artist),
                                    ),
                                    (
                                        id3_tag_id.eq(&inserted_tag.id),
                                        frame_type_id.eq(6),
                                        content.eq(title),
                                    ),
                                    (
                                        id3_tag_id.eq(&inserted_tag.id),
                                        frame_type_id.eq(4),
                                        content.eq(album),
                                    ),
                                ])
                                .execute(&connection)
                                .unwrap();
                        }
                    }

                    let progress_percent = ((file_count as f32 / total_files as f32) * 100.0).round() as i32;
                    bar.reach_percent(progress_percent);
                    file_count += 1;
                });

            info!("Completed importing {} tracks", file_count);
        }
        Command::SearchArtist { artist_name } => {
            // TODO: Figure out how to write a query that gets different frame types for a tag and
            // can report on all of them.

            use music_manager_lib::schema::files::dsl::*;
            use music_manager_lib::schema::frames::dsl::*;
            use music_manager_lib::schema::id3_tags::dsl::*;

            use models::frame::*;

            let connection = establish_connection();

            info!("Searching for artist: {:?}", artist_name);

            // let all_id3_tags = id3_tags.load::<models::tag::Tag>(&connection);
            // let tag_frame = Frame::belonging_to(&all_id3_tags)
            //     .load::<Frame>(&connection)?
            //     .grouped_by(&all_id3_tags);

            let tags_frames = id3_tags.inner_join(frames);
            let tags_files = tags_frames.inner_join(files);

            // let tags_files = tags_frames.inner_join(files).filter(frame_type_id.eq(2));

            let artist = format!("%{}%", artist_name);
            let results = tags_files.select(path).filter(content.ilike(&artist)).load::<String>(&connection).unwrap();

            for res in results.into_iter() {
                println!("{:?}", res);
            }
        }
        _ => ()
    }

});

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

// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "import")]
    Import {
        directory: String,
    },

    #[structopt(name = "search_artist")]
    SearchArtist {
        artist_name: String,
    }

}