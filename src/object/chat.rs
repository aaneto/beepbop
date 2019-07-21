use serde_derive::Deserialize;

use crate::object::{ChatPhoto, RawMessage};

#[derive(Clone, Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<RawMessage>>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
}
