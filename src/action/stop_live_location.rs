use futures::Future;

use crate::error::BotError;
use crate::input::StopLiveLocation;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn stop_live_location(
        self,
        stop_live_location: StopLiveLocation,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(
            Method::GET,
            self.get_route(&"stopMessageLiveLocation"),
            self,
        )
        .with_query(stop_live_location)
        .execute()
    }
}
