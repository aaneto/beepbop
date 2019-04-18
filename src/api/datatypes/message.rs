use serde_derive::Deserialize;

use crate::api::datatypes::Chat;
use crate::api::datatypes::User;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub date: i64,
    pub from: User,
    pub chat: Chat,
    pub text: Option<String>,
    pub new_chat_members: Option<Vec<User>>,
}
