use futures::Future;

use crate::error::BotError;
use crate::input::ChatID;
use crate::input::PinMessage;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn pin_message<ID: Into<ChatID>>(
        self,
        id: ID,
        message_id: i64,
        disable_notification: bool,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let pin_message = PinMessage::new(id.into(), message_id, disable_notification);

        TelegramRequest::new(Method::GET, self.get_route(&"pinChatMessage"), self)
            .with_query(pin_message)
            .execute()
    }
}
