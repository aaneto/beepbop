use serde_derive::Serialize;

use crate::api::args::ChatID;

#[derive(Debug, Serialize)]
pub struct SetChatTitle {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub title: String,
}

impl SetChatTitle {
    pub fn new(chat_id: ChatID, title: String) -> Self {
        Self { chat_id, title }
    }
}
