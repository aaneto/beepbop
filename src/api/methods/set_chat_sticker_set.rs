use futures::Future;

use crate::api::args::ChatID;
use crate::api::args::SetChatStickerSet;
use crate::api::error::APIError;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    /// Set sticker set for a group or chat.
    ///
    /// Will only work for groups of at least
    /// 100 members.
    pub fn set_chat_sticker_set<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        sticker_set: &str,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        let set_chat_sticker_set = SetChatStickerSet {
            sticker_set_name: sticker_set.to_string(),
            chat_id: chat_id.into(),
        };

        TelegramRequest::new(Method::GET, self.get_route(&"setChatStickerSet"), self)
            .with_query(set_chat_sticker_set)
            .execute()
    }
}
