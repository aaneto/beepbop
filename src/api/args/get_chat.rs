use serde_derive::Serialize;

use crate::api::args::ChatID;

#[derive(Debug, Serialize)]
pub struct GetChat {
    pub chat_id: ChatID,
}

impl GetChat {
    pub fn new(chat_id: ChatID) -> Self {
        Self { chat_id }
    }
}
