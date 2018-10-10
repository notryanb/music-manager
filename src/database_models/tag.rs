use schema::*;

#[derive(Debug, Queryable)]
pub struct Tag {
    pub id: i32,
    pub version_id: i32,
    pub file_id: i32,
}

#[derive(Debug, Insertable, PartialEq)]
#[table_name = "id3_tags"]
pub struct NewTag {
    pub version_id: i32,
    pub file_id: i32,
}

impl NewTag {
    pub fn new(version_id: i32, file_id: i32) -> NewTag {
        NewTag {
            version_id: version_id,
            file_id: file_id,
        }
    }
}
