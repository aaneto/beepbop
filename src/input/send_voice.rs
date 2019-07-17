use optbuilder::OptionalBuilder;
use serde_derive::Serialize;

use crate::input::ChatID;
use crate::input::ReplyMarkup;
use crate::input::Uploader;

#[derive(OptionalBuilder, Debug, Default)]
pub struct SendVoice {
    pub chat_id: ChatID,
    pub voice: Uploader,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    // Telegram Docs specifies this as integer(?).
    pub duration: Option<u32>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize)]
pub struct SendVoiceQuery {
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    // Telegram Docs specifies this as integer(?).
    pub duration: Option<u32>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVoice {
    pub fn new<ID, U>(chat_id: ID, voice: U) -> Self
    where
        ID: Into<ChatID>,
        U: Into<Uploader>,
    {
        Self {
            chat_id: chat_id.into(),
            voice: voice.into(),
            ..Default::default()
        }
    }

    pub fn split(self) -> (SendVoiceQuery, Uploader) {
        let query = SendVoiceQuery {
            chat_id: self.chat_id,
            caption: self.caption,
            parse_mode: self.parse_mode,
            duration: self.duration,
            disable_notification: self.disable_notification,
            reply_to_message_id: self.reply_to_message_id,
            reply_markup: self.reply_markup,
        };

        (query, self.voice)
    }
}
