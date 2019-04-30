use crate::models::file::File;
use crate::models::tag::Tag;

use crate::schema::*;

pub struct Track {
    pub file: File,
    pub tag: Tag,
}

impl Track {
    pub fn get_by_artist(_artist: &str) -> Option<Track> {
        // use music_manager::schema::id3_tags::dsl::*;
        // id3_tags.inner_join(frames)
        None
    }
}
