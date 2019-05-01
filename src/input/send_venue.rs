use optional_builder::optional_builder;

use serde_derive::Serialize;

use crate::input::{ChatID, ReplyMarkup};

#[optional_builder]
#[derive(Default, Debug, Serialize)]
pub struct SendVenue {
    pub chat_id: ChatID,
    pub latitude: f32,
    pub longitude: f32,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub disable_notification: bool,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVenue {
    pub fn new<S, ID>(chat_id: ID, latitude: f32, longitude: f32, title: S, address: S) -> Self
    where
        S: Into<String>,
        ID: Into<ChatID>
    {
        SendVenue {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            ..Default::default()
        }
    }
}
