use futures::Future;

use crate::error::BotError;
use crate::input::ChatID;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn unpin_message<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"unpinChatMessage"), self)
            .with_query(id.into())
            .execute()
    }
}
