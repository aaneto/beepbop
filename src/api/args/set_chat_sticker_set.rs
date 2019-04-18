use serde_derive::Serialize;

use crate::api::args::ChatID;

#[derive(Default, Debug, Serialize)]
pub struct SetChatStickerSet {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub sticker_set_name: String,
}
