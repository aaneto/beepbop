use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::api::args::ChatID;
use crate::api::args::ReplyMarkup;
use crate::api::uploaders::Uploader;

#[optional_builder]
#[derive(Default, Clone, Debug, Serialize)]
pub struct SendPhotoMeta {
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

pub struct SendPhoto<U: Uploader> {
    pub query: SendPhotoMeta,
    pub photo_uploader: U,
}

impl<U> SendPhoto<U>
where
    U: Uploader,
{
    pub fn new<ID: Into<ChatID>>(chat_id: ID, photo_uploader: U) -> Self {
        let query = SendPhotoMeta {
            chat_id: chat_id.into(),
            ..Default::default()
        };

        Self {
            query,
            photo_uploader,
        }
    }
}
