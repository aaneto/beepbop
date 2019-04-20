use futures::Future;

use crate::api::Bot;
use crate::api::args::ChatID;
use crate::api::error::APIError;
use crate::api::Method;
use crate::api::TelegramRequest;
use crate::api::args::SetChatDescription;

impl Bot {
    pub fn get_chat_members_count<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, u64), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getChatMembersCount"), self)
            .with_query(id.into())
            .execute()
    }
}