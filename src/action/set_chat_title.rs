use futures::Future;

use crate::input::ChatID;
use crate::input::SetChatTitle;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    pub fn set_chat_title<ID: Into<ChatID>>(
        self,
        id: ID,
        title: String,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let set_chat_title = SetChatTitle::new(id.into(), title);

        TelegramRequest::new(Method::GET, self.get_route(&"setChatTitle"), self)
            .with_query(set_chat_title)
            .execute()
    }
}