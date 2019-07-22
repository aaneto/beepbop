use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}
