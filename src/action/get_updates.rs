use futures::Future;

use crate::error::BotError;
use crate::input::GetUpdates;
use crate::object::Update;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn get_updates(
        self,
        get_updates: GetUpdates,
    ) -> impl Future<Item = (Self, Vec<Update>), Error = BotError> {
        TelegramRequest::new(Method::POST, self.get_route(&"getUpdates"), self)
            .with_body(get_updates)
            .execute()
    }
}
