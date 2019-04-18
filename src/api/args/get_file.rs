use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct GetFile {
    pub file_id: String,
}

impl GetFile {
    pub fn new(file_id: String) -> Self {
        GetFile { file_id }
    }
}