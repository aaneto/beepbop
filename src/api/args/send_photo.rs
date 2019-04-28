use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::api::args::ChatID;
use crate::api::args::ReplyMarkup;
use crate::api::uploaders::Uploader;

#[derive(Serialize)]
pub struct SendPhotoQuery {
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[optional_builder]
#[derive(Debug, Default)]
pub struct SendPhoto<U>
where
    U: Uploader + Default
{
    pub photo_uploader: U,
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl<U> SendPhoto<U>
where
    U: Uploader + Default,
{
    pub fn new<ID: Into<ChatID>>(chat_id: ID, photo_uploader: U) -> Self {
        SendPhoto {
            photo_uploader,
            chat_id: chat_id.into(),
            ..Default::default()
        }
    }

    pub fn split(self) -> (SendPhotoQuery, U) {
        let query = SendPhotoQuery {
            chat_id: self.chat_id,
            caption: self.caption,
            parse_mode: self.parse_mode,
            disable_notification: self.disable_notification,
            reply_to_message_id: self.reply_to_message_id,
            reply_markup: self.reply_markup,
        };

        (query, self.photo_uploader)
    }
}
