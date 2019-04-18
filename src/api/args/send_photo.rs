use std::path::PathBuf;

use serde_derive::Serialize;

use crate::util::FileUploader;
use crate::util::FileUploaderError;
use crate::util::IdUploader;
use crate::util::PostUploader;
use crate::util::UrlUploader;

use crate::api::args::ChatID;
use crate::api::args::ReplyMarkup;

#[derive(Default, Debug, Serialize)]
pub struct SendPhotoArgs {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Default, Debug, Serialize)]
pub struct SendPhoto<U: FileUploader> {
    #[serde(flatten)]
    pub args: SendPhotoArgs,
    #[serde(skip_serializing)]
    pub file_uploader: U,
}

impl<U: FileUploader> SendPhoto<U> {
    fn new<ID: Into<ChatID>>(chat_id: ID, file_uploader: U) -> Self {
        let args = SendPhotoArgs {
            chat_id: chat_id.into(),
            ..Default::default()
        };

        SendPhoto {
            args,
            file_uploader,
        }
    }
}

impl SendPhoto<UrlUploader> {
    pub fn from_url<ID: Into<ChatID>>(chat_id: ID, url: &str) -> Self {
        let uploader: UrlUploader = UrlUploader::new("photo", url);

        SendPhoto::new(chat_id, uploader)
    }
}

impl SendPhoto<IdUploader> {
    pub fn from_id<ID: Into<ChatID>>(chat_id: ID, id: &str) -> Self {
        let uploader = IdUploader::new("photo", id);

        SendPhoto::new(chat_id, uploader)
    }
}

impl SendPhoto<PostUploader> {
    pub fn from_post<ID: Into<ChatID>>(
        chat_id: ID,
        file_path: PathBuf,
        mime_string: &str,
    ) -> Result<Self, FileUploaderError> {
        let uploader_result = PostUploader::new(file_path, "photo", mime_string);

        uploader_result.map(|uploader| SendPhoto::new(chat_id, uploader))
    }
}
