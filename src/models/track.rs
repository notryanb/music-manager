use models::file::File;
use models::tag::Tag;

use schema::*;

pub struct Track {
    pub File: File,
    pub Tag: Tag,
}

impl Track {
    pub fn get_by_artist(artist: &str) -> Option<Track> {
        // use music_manager::schema::id3_tags::dsl::*;
        // id3_tags.inner_join(frames)
        None
    }
}