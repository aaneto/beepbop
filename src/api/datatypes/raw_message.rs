use serde_derive::Deserialize;

use crate::api::datatypes::User;

#[derive(Debug, Deserialize)]
pub struct RawMessage {
    pub message_id: i64,
    pub date: i64,
    pub from: User,
    pub text: Option<String>,
    pub new_chat_members: Option<Vec<User>>,
}
