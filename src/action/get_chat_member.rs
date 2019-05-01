use futures::Future;

use crate::input::ChatID;
use crate::input::GetChatMember;
use crate::object::ChatMember;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    pub fn get_chat_member<ID>(
        self,
        chat_id: ID,
        user_id: i64,
    ) -> impl Future<Item = (Self, ChatMember), Error = BotError>
    where
        ID: Into<ChatID>,
    {
        let get_chat_member = GetChatMember::new(chat_id.into(), user_id);

        TelegramRequest::new(Method::GET, self.get_route(&"getChatMember"), self)
            .with_query(get_chat_member)
            .execute()
    }
}
