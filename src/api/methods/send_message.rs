use futures::Future;

use crate::api::args::SendMessage;
use crate::api::datatypes::Message;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn send_message(
        self,
        send_message: SendMessage,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendMessage"), self)
            .with_query(send_message)
            .execute()
    }
}
