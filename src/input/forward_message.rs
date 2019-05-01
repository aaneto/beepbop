use serde_derive::Serialize;

use crate::input::ChatID;

#[derive(Debug, Serialize)]
pub struct ForwardMessage {
    pub chat_id: ChatID,
    pub from_chat_id: ChatID,
    pub message_id: i64,
    pub disable_notification: bool,
}
