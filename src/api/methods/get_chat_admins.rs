use futures::Future;

use crate::api::args::ChatID;
use crate::api::datatypes::ChatMember;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn get_chat_admins<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, Vec<ChatMember>), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getChatAdministrators"), self)
            .with_query(chat_id.into())
            .execute()
    }
}
