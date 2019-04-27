use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::api::args::ChatID;
use crate::api::args::ReplyMarkup;
use crate::api::uploaders::Uploader;

#[optional_builder]
#[derive(Default, Debug, Serialize)]
pub struct SendDocumentQuery {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug)]
pub struct SendDocument<T: Uploader> {
    pub document: T,
    pub query: SendDocumentQuery,
}

impl<U> SendDocument<U>
where
    U: Uploader,
{
    pub fn new<ID>(chat_id: ID, document: U) -> Self
    where
        ID: Into<ChatID>,
    {
        let query = SendDocumentQuery {
            chat_id: chat_id.into(),
            ..Default::default()
        };

        Self { document, query }
    }
}
