use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileInfo {
    pub file_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}
