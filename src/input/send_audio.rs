use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::input::ChatID;
use crate::input::ReplyMarkup;
use crate::input::Uploader;

#[optional_builder]
#[derive(Debug, Default)]
pub struct SendAudio<T>
where
    T: Uploader + Default,
{
    pub chat_id: ChatID,
    pub voice: T,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    // Telegram Docs specifies this as integer(?).
    pub duration: Option<u32>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize)]
pub struct SendAudioQuery {
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    // Telegram Docs specifies this as integer(?).
    pub duration: Option<u32>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl<T: Uploader> SendAudio<T>
where
    T: Uploader + Default,
{
    pub fn new<ID: Into<ChatID>>(chat_id: ID, voice: T) -> Self {
        Self {
            chat_id: chat_id.into(),
            voice,
            ..Default::default()
        }
    }

    pub fn split(self) -> (SendAudioQuery, T) {
        let query = SendAudioQuery {
            chat_id: self.chat_id,
            caption: self.caption,
            parse_mode: self.parse_mode,
            duration: self.duration,
            performer: self.performer,
            title: self.title,
            disable_notification: self.disable_notification,
            reply_to_message_id: self.reply_to_message_id,
            reply_markup: self.reply_markup,
        };

        (query, self.voice)
    }
}
