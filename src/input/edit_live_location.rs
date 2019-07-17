use optbuilder::OptionalBuilder;

use serde_derive::Serialize;

use crate::input::ChatID;
use crate::input::ReplyMarkup;

#[derive(OptionalBuilder, Default, Clone, Debug, Serialize)]
pub struct EditLiveLocation {
    #[optbuilder(skip)]
    pub chat_id: Option<ChatID>,
    pub latitude: f64,
    pub longitude: f64,
    #[optbuilder(skip)]
    pub message_id: Option<i64>,
    #[optbuilder(skip)]
    pub inline_message_id: Option<String>,
    #[serde(flatten)]
    pub reply_markup: Option<ReplyMarkup>,
}

impl EditLiveLocation {
    pub fn new<ID: Into<ChatID>>(
        chat_id: ID,
        message_id: i64,
        latitude: f64,
        longitude: f64,
    ) -> EditLiveLocation {
        EditLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            latitude,
            longitude,
            ..Default::default()
        }
    }

    pub fn inline(inline_message_id: String, latitude: f64, longitude: f64) -> EditLiveLocation {
        EditLiveLocation {
            inline_message_id: Some(inline_message_id),
            latitude,
            longitude,
            ..Default::default()
        }
    }
}
