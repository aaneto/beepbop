use futures::Future;

use crate::error::BotError;
use crate::input::ChatID;
use crate::input::SetChatDescription;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn set_chat_description<ID: Into<ChatID>>(
        self,
        id: ID,
        description: String,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let set_chat_description = SetChatDescription::new(id.into(), description);

        TelegramRequest::new(Method::GET, self.get_route(&"setChatDescription"), self)
            .with_query(set_chat_description)
            .execute()
    }
}
