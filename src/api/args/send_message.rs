use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::api::args::ChatID;
use crate::api::args::ForceReply;
use crate::api::args::ReplyMarkup;

#[optional_builder]
#[derive(Debug, Default, Serialize)]
pub struct SendMessage {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub text: String,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    #[serde(flatten)]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    pub fn new(chat_id: ChatID, text: String) -> Self {
        Self {
            chat_id,
            text,
            ..Default::default()
        }
    }
}
