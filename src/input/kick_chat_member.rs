use serde_derive::Serialize;

use crate::input::ChatID;

#[derive(Debug, Serialize)]
pub struct KickChatMember {
    pub chat_id: ChatID,
    pub user_id: i64,
    pub until_date: Option<u64>,
}