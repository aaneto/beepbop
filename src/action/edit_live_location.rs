use futures::Future;

use crate::error::BotError;
use crate::input::EditLiveLocation;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn edit_live_location(
        self,
        edit_live_location: EditLiveLocation,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(
            Method::GET,
            self.get_route(&"editMessageLiveLocation"),
            self,
        )
        .with_query(edit_live_location)
        .execute()
    }
}
