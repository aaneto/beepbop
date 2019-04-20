use futures::Future;

use crate::api::args::ChatID;
use crate::api::args::GetChatMember;
use crate::api::datatypes::ChatMember;
use crate::api::error::APIError;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    pub fn get_chat_member<ID>(
        self,
        chat_id: ID,
        user_id: i64,
    ) -> impl Future<Item = (Self, ChatMember), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        let get_chat_member = GetChatMember::new(chat_id.into(), user_id);

        TelegramRequest::new(Method::GET, self.get_route(&"getChatMember"), self)
            .with_query(get_chat_member)
            .execute()
    }
}
