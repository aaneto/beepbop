use futures::Future;

use crate::api::args::ChatID;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn unpin_message<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"unpinChatMessage"), self)
            .with_query(id.into())
            .execute()
    }
}
