use futures::Future;

use crate::input::ChatID;
use crate::error::BotError;
use crate::input::FileUploader;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    pub fn set_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        file_uploader: FileUploader,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::POST, self.get_route(&"setChatPhoto"), self)
            .with_query(chat_id.into())
            .with_uploader("photo", file_uploader)
            .execute()
    }
}