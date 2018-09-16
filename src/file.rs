#[derive(Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub path: String,
}