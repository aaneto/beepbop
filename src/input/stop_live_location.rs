use optional_builder::optional_builder;

use serde_derive::Serialize;

use crate::input::ChatID;
use crate::input::ReplyMarkup;

#[optional_builder]
#[derive(Default, Clone, Debug, Serialize)]
pub struct StopLiveLocation {
    pub chat_id: Option<ChatID>,
    pub message_id: Option<i64>,
    pub inline_message_id: Option<String>,
    #[serde(flatten)]
    pub reply_markup: Option<ReplyMarkup>,
}

impl StopLiveLocation {
    pub fn new<ID: Into<ChatID>>(chat_id: ID, message_id: i64) -> StopLiveLocation {
        StopLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            ..Default::default()
        }
    }

    pub fn inline(inline_message_id: String) -> StopLiveLocation {
        StopLiveLocation {
            inline_message_id: Some(inline_message_id),
            ..Default::default()
        }
    }
}
