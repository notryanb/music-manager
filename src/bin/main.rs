// #![windows_subsystem = "windows"]

extern crate diesel;
extern crate id3;
extern crate music_manager_lib;
extern crate walkdir;


#[macro_use]
extern crate native_windows_gui as nwg;

use nwg::{
    Event,
    Ui,
    simple_message,
    fatal_message,
    dispatch_events
};

use self::music_manager_lib::*;
use self::file::{File, NewFile};
use self::diesel::prelude::*;
use diesel::insert_into;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone, Hash)]
pub enum AppId {
    MainWindow,
    NameInput,
    HelloButton,
    Label(u8),
    SayHello,
    MainFont,
    TextFont,
}

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

    let connection = establish_connection();

    let music_dir = "E:\\Torrent Finished";

    WalkDir::new(music_dir).into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| get_mp3_file_paths(&e))
        .for_each(|e| {
            let new_file = NewFile { path: e };

            insert_into(files)
                .values(new_file)
                .execute(&connection);
        });

    let results = files
        .load::<File>(&connection)
        .expect("Error loading files");

    println!("Displaying {} files", results.len());

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
            Some(exxt) if  exxt == "mp3" => {
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