use serde_derive::Serialize;

use crate::api::args::ChatID;
use crate::api::args::ReplyMarkup;

#[derive(Debug, Serialize)]
pub struct SendLocation {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub latitude: f64,
    pub longitude: f64,
    pub live_period: Option<u32>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    #[serde(flatten)]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendLocation {
    pub fn new(chat_id: ChatID, latitude: f64, longitude: f64) -> Self {
        SendLocation {
            chat_id,
            latitude,
            longitude,
            live_period: None,
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
}
