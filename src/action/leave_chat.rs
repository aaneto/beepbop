use futures::Future;

use crate::input::ChatID;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    pub fn leave_chat<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"leaveChat"), self)
            .with_query(id.into())
            .execute()
    }
}
