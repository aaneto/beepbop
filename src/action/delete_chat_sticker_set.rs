use futures::Future;

use crate::input::ChatID;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    /// Delete sticker set for a group or chat.
    ///
    /// Will only work if there is a sticker set defined.
    pub fn delete_chat_sticker_set<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"deleteChatStickerSet"), self)
            .with_query(chat_id.into())
            .execute()
    }
}
