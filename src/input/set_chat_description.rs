use serde_derive::Serialize;

use crate::input::ChatID;

#[derive(Debug, Serialize)]
pub struct SetChatDescription {
    pub chat_id: ChatID,
    pub description: String,
}

impl SetChatDescription {
    pub fn new(chat_id: ChatID, description: String) -> Self {
        Self {
            chat_id,
            description,
        }
    }
}
