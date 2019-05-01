use futures::Future;

use crate::input::SendLocation;
use crate::object::Message;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    pub fn send_location(
        self,
        send_location: SendLocation,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendLocation"), self)
            .with_query(send_location)
            .execute()
    }
}
