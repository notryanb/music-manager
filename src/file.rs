use schema::*;

#[derive(Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub path: String,
}

#[derive(Debug, Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub path: String,
}