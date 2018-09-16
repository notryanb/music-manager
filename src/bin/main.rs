extern crate diesel;
extern crate music_manager_lib;

use self::music_manager_lib::*;
use self::file::File;
use self::diesel::prelude::*;

fn main() {
    use music_manager_lib::schema::files::dsl::*;

    let connection = establish_connection();
    let results = files
        .load::<File>(&connection)
        .expect("Error loading files");

    println!("Displaying {} files", results.len());

    for file in results {
        println!("{}", file.path);
    }
}