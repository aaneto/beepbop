use futures::Future;

use crate::api::args::ChatID;
use crate::api::args::SetChatTitle;
use crate::api::error::APIError;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    pub fn set_chat_title<ID: Into<ChatID>>(
        self,
        id: ID,
        title: String,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        let set_chat_title = SetChatTitle::new(id.into(), title);

        TelegramRequest::new(Method::GET, self.get_route(&"setChatTitle"), self)
            .with_query(set_chat_title)
            .execute()
    }
}
