use serde_derive::Serialize;

use crate::input::ChatID;

#[derive(Debug, Serialize)]
pub struct GetChatMember {
    pub chat_id: ChatID,
    pub user_id: i64,
}

impl GetChatMember {
    pub fn new(chat_id: ChatID, user_id: i64) -> Self {
        Self { chat_id, user_id }
    }
}
