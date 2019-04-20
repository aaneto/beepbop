use futures::Future;

use crate::api::args::ChatID;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn delete_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"deleteChatPhoto"), self)
            .with_query(chat_id.into())
            .execute()
    }
}
