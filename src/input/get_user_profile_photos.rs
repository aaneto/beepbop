use optional_builder::optional_builder;
use serde_derive::Serialize;

#[optional_builder]
#[derive(Default, Debug, Serialize)]
pub struct GetUserProfilePhotos {
    pub user_id: i64,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl GetUserProfilePhotos {
    pub fn new(user_id: i64) -> Self {
        GetUserProfilePhotos {
            user_id,
            ..Default::default()
        }
    }
}
