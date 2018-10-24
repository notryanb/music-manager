use diesel::prelude::*;
use schema::*;

#[derive(Debug, Associations, Identifiable, Queryable)]
#[belongs_to(Tag, foreign_key = "id3_tag_id")]
pub struct Frame {
    pub id: i32,
    pub id3_tag_id: i32,
    pub frame_type_id: i32,
    pub content: String,
}

#[derive(Debug, Insertable, PartialEq)]
#[table_name = "frames"]
pub struct NewFrame {
    pub id3_tag_id: i32,
    pub frame_type_id: i32,
    pub content: String,
}

impl NewFrame {
    pub fn new(id3_tag_id: i32, frame_type_id: i32, content: &str) -> NewFrame {
        NewFrame {
            id3_tag_id,
            frame_type_id,
            content: content.to_string(),
        }
    }
}
