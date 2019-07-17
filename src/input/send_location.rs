use optbuilder::OptionalBuilder;

use serde_derive::Serialize;

use crate::input::ChatID;
use crate::input::ReplyMarkup;

#[derive(OptionalBuilder, Clone, Debug, Serialize)]
pub struct SendLocation {
    pub chat_id: ChatID,
    pub latitude: f64,
    pub longitude: f64,
    #[optbuilder(skip)]
    pub live_period: Option<u32>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    #[serde(flatten)]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendLocation {
    pub fn new<ID: Into<ChatID>>(chat_id: ID, latitude: f64, longitude: f64) -> Self {
        SendLocation {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            live_period: Some(60),
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn with_live_period(mut self, live_period: u32) -> Self {
        if live_period > 86400 {
            self.live_period = Some(86400);
        } else if live_period < 60 {
            self.live_period = Some(60);
        } else {
            self.live_period = Some(live_period);
        }

        self
    }

    pub fn without_live_period(mut self) -> Self {
        self.live_period = None;

        self
    }
}
