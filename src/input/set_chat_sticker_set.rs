use serde_derive::Serialize;

use crate::input::ChatID;

#[derive(Default, Debug, Serialize)]
pub struct SetChatStickerSet {
    pub chat_id: ChatID,
    pub sticker_set_name: String,
}

impl SetChatStickerSet {
    pub fn new(chat_id: ChatID, sticker_set_name: String) -> Self {
        Self {
            chat_id,
            sticker_set_name,
        }
    }
}
