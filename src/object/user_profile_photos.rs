use serde_derive::Deserialize;

use crate::object::PhotoSize;

#[derive(Debug, Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: usize,
    pub photos: Vec<Vec<PhotoSize>>,
}
