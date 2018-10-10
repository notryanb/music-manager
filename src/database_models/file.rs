use schema::*;

#[derive(Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub path: String,
}

#[derive(Debug, Insertable, PartialEq)]
#[table_name = "files"]
pub struct NewFile {
    pub path: String,
}

impl NewFile {
    pub fn new(path: &str) -> NewFile {
        NewFile {
            path: path.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_file_from_str() {
        let path = "some/path.mp3";
        let result = NewFile::new(path);
        let expected = NewFile { path: path.to_string() };
        assert_eq!(result, expected);
    }
}