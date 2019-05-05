use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::input::ChatID;
use crate::input::ReplyMarkup;
use crate::input::Uploader;

#[optional_builder]
#[derive(Debug, Default)]
pub struct SendAudio {
    pub chat_id: ChatID,
    pub voice: Uploader,
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

impl SendAudio {
    pub fn new<ID, U>(chat_id: ID, voice: U) -> Self
    where
        ID: Into<ChatID>,
        U: Into<Uploader>
    {
        Self {
            chat_id: chat_id.into(),
            voice: voice.into(),
            ..Default::default()
        }
    }

    pub fn split(self) -> (SendAudioQuery, Uploader) {
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
