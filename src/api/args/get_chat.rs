use serde_derive::Serialize;

use crate::api::args::ChatID;

#[derive(Debug, Serialize)]
pub struct GetChat {
    #[serde(flatten)]
    pub chat_id: ChatID,
}

impl GetChat {
    pub fn new(chat_id: ChatID) -> Self {
        Self { chat_id }
    }
    pub fn from_integer(id: i64) -> Self {
        Self {
            chat_id: ChatID::Integer { chat_id: id },
        }
    }

    pub fn from_string(id: String) -> Self {
        Self {
            chat_id: ChatID::String { chat_id: id },
        }
    }
}