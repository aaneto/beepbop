use futures::Future;

use crate::error::BotError;
use crate::input::ChatID;
use crate::object::ChatMember;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn get_chat_admins<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, Vec<ChatMember>), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getChatAdministrators"), self)
            .with_query(chat_id.into())
            .execute()
    }
}
