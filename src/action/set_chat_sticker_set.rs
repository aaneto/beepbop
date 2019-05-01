use futures::Future;

use crate::input::ChatID;
use crate::input::SetChatStickerSet;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    /// Set sticker set for a group or chat.
    ///
    /// Will only work for groups of at least
    /// 100 members.
    pub fn set_chat_sticker_set<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        sticker_set: &str,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let set_chat_sticker_set = SetChatStickerSet {
            sticker_set_name: sticker_set.to_string(),
            chat_id: chat_id.into(),
        };

        TelegramRequest::new(Method::GET, self.get_route(&"setChatStickerSet"), self)
            .with_query(set_chat_sticker_set)
            .execute()
    }
}
