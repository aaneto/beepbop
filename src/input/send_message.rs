use optbuilder::OptionalBuilder;
use serde_derive::Serialize;

use crate::input::ChatID;
use crate::input::ReplyMarkup;

#[derive(OptionalBuilder, Debug, Default, Serialize)]
pub struct SendMessage {
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
