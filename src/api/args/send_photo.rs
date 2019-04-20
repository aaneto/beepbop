use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::api::args::ChatID;
use crate::api::args::ReplyMarkup;

#[optional_builder]
#[derive(Default, Clone, Debug, Serialize)]
pub struct SendPhoto {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendPhoto {
    pub fn new(chat_id: ChatID) -> Self {
        Self {
            chat_id,
            ..Default::default()
        }
    }
}
