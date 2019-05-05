use std::error::Error;

use optional_builder::optional_builder;
use serde_derive::Serialize;

use crate::error::BotError;
use crate::input::ChatID;
use crate::input::FileUploader;
use crate::input::Uploader;

#[optional_builder]
#[derive(Default, Debug, Serialize)]
/// The MediaVideo is a struct containing metadata
/// about a video used within an MediaGroup. the media
/// String is a name granted by the MediaGroup object.
pub struct MediaVideo {
    pub r#type: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

#[optional_builder]
#[derive(Default, Debug, Serialize)]
/// The MediaPhoto is a struct containing metadata
/// about a photo used within an MediaGroup. the media
/// String is a name granted by the MediaGroup object.
pub struct MediaPhoto {
    pub r#type: String,
    pub media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum MediaEntry {
    Video(MediaVideo),
    Photo(MediaPhoto),
}

#[optional_builder]
#[derive(Default, Debug, Serialize)]
pub struct MediaGroupQuery {
    pub chat_id: ChatID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
}

#[derive(Debug)]
pub struct Attachment {
    pub uploader: Uploader,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct MediaGroup {
    pub chat_id: ChatID,
    pub media: Vec<MediaEntry>,
    pub media_encoded: String,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub attachments: Vec<Attachment>,
}

impl MediaGroup {
    pub fn build<ID: Into<ChatID>>(chat_id: ID) -> MediaGroupBuilder {
        let media = MediaGroup {
            chat_id: chat_id.into(),
            ..Default::default()
        };

        MediaGroupBuilder { builder: media }
    }

    pub fn split(self) -> (MediaGroupQuery, String, Vec<Attachment>) {
        let query = MediaGroupQuery {
            chat_id: self.chat_id,
            disable_notification: self.disable_notification,
            reply_to_message_id: self.reply_to_message_id,
        };

        (query, self.media_encoded, self.attachments)
    }
}

pub struct MediaGroupBuilder {
    builder: MediaGroup,
}

impl MediaGroupBuilder {
    pub fn finish(mut self) -> Result<MediaGroup, BotError> {
        let number_of_medias = self.builder.media.len();

        if number_of_medias > 10 || number_of_medias < 2 {
            return Err(BotError::InvalidMediaGroup(format!(
                "Group media must have between 2 and 10 media files, found {}",
                number_of_medias
            )));
        }

        match serde_json::to_string(&self.builder.media) {
            Ok(media_encoded) => {
                self.builder.media_encoded = media_encoded;
            }
            Err(err) => {
                return Err(BotError::InvalidMediaGroup(format!(
                    "Cannot deserialize media group: {}",
                    err.description()
                )));
            }
        };

        Ok(self.builder)
    }

    pub fn build_photo<U, F>(mut self, uploader: U, func: F) -> Self
    where
        U: Into<Uploader>,
        F: Fn(MediaPhoto) -> MediaPhoto,
    {
        match uploader.into() {
            Uploader::File(file_uploader) => {
                let photo = MediaPhoto {
                    r#type: "photo".into(),
                    media: format!("attach://{}", file_uploader.file_name.clone()),
                    ..Default::default()
                };

                self.builder.media.push(MediaEntry::Photo(func(photo)));
                self.builder.attachments.push(Attachment {
                    name: file_uploader.file_name.clone(),
                    uploader: file_uploader.into(),
                });
            }
            Uploader::Id(id) => {
                let photo = MediaPhoto {
                    r#type: "photo".into(),
                    media: id.0,
                    ..Default::default()
                };

                self.builder.media.push(MediaEntry::Photo(func(photo)));
            }
            Uploader::Url(url) => {
                let photo = MediaPhoto {
                    r#type: "photo".into(),
                    media: url.0,
                    ..Default::default()
                };

                self.builder.media.push(MediaEntry::Photo(func(photo)));
            }
            Uploader::Empty => (),
        }

        self
    }

    pub fn add_photo<U>(self, uploader: U) -> Self
    where
        U: Into<Uploader>,
    {
        self.build_photo(uploader, std::convert::identity)
    }

    pub fn build_video<U, F>(
        mut self,
        uploader: U,
        thumbnail: Option<FileUploader>,
        func: F,
    ) -> Self
    where
        U: Into<Uploader>,
        F: Fn(MediaVideo) -> MediaVideo,
    {
        match uploader.into() {
            Uploader::File(file_uploader) => {
                let mut video = MediaVideo {
                    r#type: "video".into(),
                    media: format!("attach://{}", file_uploader.file_name.clone()),
                    ..Default::default()
                };

                if let Some(thumb) = thumbnail {
                    video = video.with_thumb(thumb.file_name.clone());

                    self.builder.attachments.push(Attachment {
                        name: thumb.file_name.clone(),
                        uploader: thumb.into(),
                    });
                };

                self.builder.media.push(MediaEntry::Video(func(video)));

                self.builder.attachments.push(Attachment {
                    name: file_uploader.file_name.clone(),
                    uploader: file_uploader.into(),
                });
            }
            Uploader::Id(id) => {
                let video = MediaVideo {
                    r#type: "video".into(),
                    media: id.0.clone(),
                    ..Default::default()
                };

                self.builder.media.push(MediaEntry::Video(func(video)));
            }
            Uploader::Url(url) => {
                let video = MediaVideo {
                    r#type: "video".into(),
                    media: url.0.clone(),
                    ..Default::default()
                };

                self.builder.media.push(MediaEntry::Video(func(video)));
            }
            Uploader::Empty => (),
        }

        self
    }

    pub fn add_video<U>(self, uploader: U, thumbnail: Option<FileUploader>) -> Self
    where
        U: Into<Uploader>,
    {
        self.build_video(uploader, thumbnail, std::convert::identity)
    }
}
