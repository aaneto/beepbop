use serde_derive::Serialize;

use crate::input::ChatID;

#[derive(Debug, Serialize)]
pub struct ExportChatInviteLink {
    pub chat_id: ChatID,
}

impl ExportChatInviteLink {
    pub fn new(chat_id: ChatID) -> Self {
        Self { chat_id }
    }
}
