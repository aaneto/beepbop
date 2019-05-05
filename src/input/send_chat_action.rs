use serde_derive::Serialize;

use crate::input::{Action, ChatID};

#[derive(Serialize)]
pub struct SendChatAction {
    pub chat_id: ChatID,
    pub action: String,
}

impl SendChatAction {
    pub fn new(chat_id: ChatID, action: Action) -> Self {
        SendChatAction {
            chat_id,
            action: action.to_string(),
        }
    }
}
