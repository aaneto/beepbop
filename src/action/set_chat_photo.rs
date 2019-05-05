use futures::Future;

use crate::error::BotError;
use crate::input::ChatID;
use crate::input::FileUploader;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn set_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        photo: FileUploader,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::POST, self.get_route(&"setChatPhoto"), self)
            .with_query(chat_id.into())
            .with_uploader("photo", photo.into())
            .execute()
    }
}
