use serde_derive::Serialize;

use crate::input::ChatID;

#[derive(Debug, Serialize)]
pub struct UnbanChatMember {
    pub chat_id: ChatID,
    pub user_id: i64,
}