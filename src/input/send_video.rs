use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::input::ChatID;
use crate::input::ReplyMarkup;
use crate::input::Uploader;

#[derive(Serialize)]
pub struct SendVideoQuery {
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
    pub support_streaming: Option<bool>,
    pub duration: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[optional_builder]
#[derive(Default, Debug)]
pub struct SendVideo<U>
where
    U: Uploader + Default,
{
    pub video: U,
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
    pub support_streaming: Option<bool>,
    pub duration: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl<U> SendVideo<U>
where
    U: Uploader + Default,
{
    pub fn new<ID>(chat_id: ID, video: U) -> Self
    where
        ID: Into<ChatID>,
    {
        SendVideo {
            video,
            chat_id: chat_id.into(),
            ..Default::default()
        }
    }

    pub fn split(self) -> (SendVideoQuery, U) {
        let query = SendVideoQuery {
            chat_id: self.chat_id,
            caption: self.caption,
            parse_mode: self.parse_mode,
            disable_notification: self.disable_notification,
            reply_to_message_id: self.reply_to_message_id,
            reply_markup: self.reply_markup,
            support_streaming: self.support_streaming,
            duration: self.duration,
            width: self.width,
            height: self.height,
        };

        (query, self.video)
    }
}
