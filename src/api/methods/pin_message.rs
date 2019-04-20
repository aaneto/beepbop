use futures::Future;

use crate::api::args::ChatID;
use crate::api::args::PinMessage;
use crate::api::error::APIError;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    pub fn pin_message<ID: Into<ChatID>>(
        self,
        id: ID,
        message_id: i64,
        disable_notification: bool,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        let pin_message = PinMessage::new(id.into(), message_id, disable_notification);

        TelegramRequest::new(Method::GET, self.get_route(&"pinChatMessage"), self)
            .with_query(pin_message)
            .execute()
    }
}
