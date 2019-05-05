use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: u32,
    pub height: u32,
    pub file_size: Option<usize>,
}
