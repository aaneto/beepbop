use futures::Future;

use crate::api::args::ChatID;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    /// Delete sticker set for a group or chat.
    ///
    /// Will only work if there is a sticker set defined.
    pub fn delete_chat_sticker_set<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"deleteChatStickerSet"), self)
            .with_query(chat_id.into())
            .execute()
    }
}
