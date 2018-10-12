// #![windows_subsystem = "windows"]

extern crate diesel;
extern crate id3;
extern crate music_manager_lib;
extern crate walkdir;


// #[macro_use]
// extern crate native_windows_gui as nwg;

// use nwg::{
//     Event,
//     Ui,
//     simple_message,
//     fatal_message,
//     dispatch_events
// };

use self::music_manager_lib::*;

use diesel::prelude::*;
use walkdir::{DirEntry, WalkDir};
use id3::{Tag};

// #[derive(Debug, Clone, Hash)]
// pub enum AppId {
//     MainWindow,
//     NameInput,
//     HelloButton,
//     Label(u8),
//     SayHello,
//     MainFont,
//     TextFont,
// }

// use AppId::*;

// nwg_template!(
//     head: setup_ui<AppId>,
//     controls: [
//         (MainWindow, nwg_window!( title="Template Example"; size=(280, 105) )),
//         (Label(0), nwg_label!( parent=MainWindow; text="Your Name: "; position=(5,15); size=(80, 25); font=Some(TextFont) )),
//         (NameInput, nwg_textinput!( parent=MainWindow; position=(85,13); size=(185,22); font=Some(TextFont) )),
//         (HelloButton, nwg_button!( parent=MainWindow; text="Hello World!"; position=(5, 45); size=(270, 50); font=Some(MainFont) ))
//     ];
//     events: [
//         (HelloButton, SayHello, Event::Click, |ui,_,_,_| {
//             let your_name = nwg_get!(ui; (NameInput, nwg::TextInput));
//             simple_message("Hello", &format!("Hello {}!", your_name.get_text()) );
//         })
//     ];
//     resources: [
//         (MainFont, nwg_font!(family="Arial"; size=27)),
//         (TextFont, nwg_font!(family="Arial"; size=17))
//     ];
//     values: []
// );


fn main() {
    use music_manager_lib::schema::files::dsl::*;
    use music_manager_lib::schema::id3_tags::dsl::*;
    use music_manager_lib::schema::frames::dsl::*;

    let connection = establish_connection();

    let music_dir = "E:\\Torrent Finished";

    // WalkDir::new(music_dir).into_iter()
    //     .filter_map(|e| e.ok())
    //     .filter_map(|e| get_mp3_file_paths(&e))
    //     .for_each(|e| {
    //         let tag = Tag::read_from_path(&e);

    //         let new_file = models::file::NewFile::new(&e);

    //         let file = diesel::insert_into(files)
    //             .values(&new_file)
    //             .get_result::<models::file::File>(&connection)
    //             .expect("Error inserting file");

    //         let new_tag = models::tag::NewTag::new(3, file.id);

    //         let inserted_tag = diesel::insert_into(id3_tags)
    //             .values(&new_tag)
    //             .get_result::<models::tag::Tag>(&connection)
    //             .expect("error inserting id3 tag");

    //         if tag.is_ok() {
    //             let safe_tag = tag.unwrap();
    //             let artist = safe_tag.artist().map_or(String::new(), |artist| artist.to_string());
    //             let title = safe_tag.title().map_or(String::new(), |title| title.to_string());
    //             let album = safe_tag.album().map_or(String::new(), |album| album.to_string());

    //             diesel::insert_into(frames)
    //                 .values(&vec![
    //                     (id3_tag_id.eq(&inserted_tag.id), frame_type_id.eq(2), content.eq(artist)),
    //                     (id3_tag_id.eq(&inserted_tag.id), frame_type_id.eq(6), content.eq(title)),
    //                     (id3_tag_id.eq(&inserted_tag.id), frame_type_id.eq(4), content.eq(album)),
    //                 ])
    //                 .execute(&connection)
    //                 .unwrap();
    //         }
    //     });

    let tags_frames = id3_tags.inner_join(frames);
    let tags_files = tags_frames.inner_join(files).filter(frame_type_id.eq(2));

    let results = tags_files.limit(5).select(content).filter(content.ilike("%matt%")).load::<String>(&connection).unwrap();

    println!("Yo");
    for res in results.into_iter() {
        println!("{:?}", res);
    }

    // let tags_files = id3_tags.inner_join(files).select(id3_tags::id, files::path);
    // let artist_frame = frames.filter(frame_type_id.eq(2));
    // let tf_artist = tags_files.inner_join(artist_frame).load(&connection);

    // let file_count = files.count().get_result(&connection);
    // println!("FileCount: {:?}", Ok(file_count));

    // let results = files
    //     .load::<File>(&connection)
    //     .expect("Error loading files");

    // println!("Displaying {} files", results.len());

    // for file in results {
    //     println!("{}", file.path);
    // }

    // let app: Ui<AppId>;

    //     match Ui::new() {
    //     Ok(_app) => { app = _app; },
    //     Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    // }

    // if let Err(e) = setup_ui(&app) {
    //     fatal_message("Fatal Error", &format!("{:?}", e));
    // }

    // dispatch_events();

}

/// Returns the file path if it's a .mp3 file or None.
pub fn get_mp3_file_paths(entry: &DirEntry) -> Option<String> {
    match entry.path().extension() {
        Some(ext) => match ext.to_str() {
            Some(exxt) if exxt == "mp3" => {
                match entry.path().to_str() {
                    Some(p) => Some(p.to_string()),
                    None => None,
                }
            },
            Some(_) => None,
            None => None,
        }
        None => None,
    }
}
