use futures::Future;

use crate::api::args::ChatID;
use crate::api::args::SetChatDescription;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn set_chat_description<ID: Into<ChatID>>(
        self,
        id: ID,
        description: String,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        let set_chat_description = SetChatDescription::new(id.into(), description);

        TelegramRequest::new(Method::GET, self.get_route(&"setChatDescription"), self)
            .with_query(set_chat_description)
            .execute()
    }
}
