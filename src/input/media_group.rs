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
    r#type: String,
    media: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[optional_builder(skip)]
    thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
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
/// The MediaEntry Enum is a container type
/// for both MediaVideo and MediaPhoto.
///
/// This is a detail implementation of MediaGroup
/// and probably should not be used directly.
enum MediaEntry {
    Video(MediaVideo),
    Photo(MediaPhoto),
}

#[derive(Default, Debug, Serialize)]
/// The MediaGroupQuery is a struct with
/// all queryable fields on MediaGroup,
/// it is a implementation detail of MediaGroup
/// and probably should not be used directly.
pub struct MediaGroupQuery {
    chat_id: ChatID,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
}

#[derive(Debug)]
/// The Attachment is a uploader associated
/// with a name, this name is usefull because
/// uploading MediaGroup's includes having to
/// remember what files where uploaded under
/// what name.
///
/// This struct is an implementation detail of
/// MediaGroup and should not be used directly.
pub struct Attachment {
    pub uploader: Uploader,
    pub name: String,
}

#[derive(Default, Debug)]
/// The MediaGroup is a container
/// with all the information necessary
/// to send a MediaGroup to telegram.
pub struct MediaGroup {
    pub query: MediaGroupQuery,
    pub media_encoded: String,
    pub attachments: Vec<Attachment>,
}

#[optional_builder]
#[derive(Default, Debug)]
pub struct MediaGroupBuilder {
    chat_id: ChatID,
    media: Vec<MediaEntry>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    attachments: Vec<Attachment>,
}

impl MediaGroup {
    /// Create a new MediaGroupBuilder
    pub fn build<ID: Into<ChatID>>(chat_id: ID) -> MediaGroupBuilder {
        MediaGroupBuilder {
            chat_id: chat_id.into(),
            ..Default::default()
        }
    }
}

impl MediaGroupBuilder {
    /// Finish the building of the MediaGroup.
    pub fn finish(self) -> Result<MediaGroup, BotError> {
        let number_of_medias = self.media.len();

        if number_of_medias > 10 || number_of_medias < 2 {
            return Err(BotError::InvalidMediaGroup(format!(
                "Group media must have between 2 and 10 media files, found {}",
                number_of_medias
            )));
        }

        match serde_json::to_string(&self.media) {
            Ok(media_encoded) => Ok(MediaGroup {
                media_encoded,
                query: MediaGroupQuery {
                    chat_id: self.chat_id,
                    disable_notification: self.disable_notification,
                    reply_to_message_id: self.reply_to_message_id,
                },
                attachments: self.attachments,
            }),
            Err(err) => {
                return Err(BotError::InvalidMediaGroup(format!(
                    "Cannot deserialize media group: {}",
                    err.description()
                )));
            }
        }
    }

    /// Add a new photo to the MediaGroup, the photo
    /// will be created from the file provided.
    pub fn add_photo<U>(self, uploader: U) -> Self
    where
        U: Into<Uploader>,
    {
        // Add photo without modifying the output photo.
        self.add_photo_with(uploader, std::convert::identity)
    }

    /// Add a new video to the MediaGroup, the video
    /// will be created from the file provided.
    pub fn add_video<U>(self, uploader: U, thumbnail: Option<FileUploader>) -> Self
    where
        U: Into<Uploader>,
    {
        self.add_video_with(uploader, thumbnail, std::convert::identity)
    }

    /// Add a new photo to the MediaGroup but with an edit_function
    /// to edit some details of the photo after its creation.
    pub fn add_photo_with<U, F>(mut self, uploader: U, edit_function: F) -> Self
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

                self.media.push(MediaEntry::Photo(edit_function(photo)));
                self.attachments.push(Attachment {
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

                self.media.push(MediaEntry::Photo(edit_function(photo)));
            }
            Uploader::Url(url) => {
                let photo = MediaPhoto {
                    r#type: "photo".into(),
                    media: url.0,
                    ..Default::default()
                };

                self.media.push(MediaEntry::Photo(edit_function(photo)));
            }
            Uploader::Empty => (),
        }

        self
    }

    /// Add a new video to the MediaGroup, but with an edit_function
    /// to edit some details of the photo after its creation.
    pub fn add_video_with<U, F>(
        mut self,
        uploader: U,
        thumbnail: Option<FileUploader>,
        edit_function: F,
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
                    // with_thumb method is not created to avoid
                    // the user from using it inside the build edit_function closure.
                    video.thumb = Some(format!("attach://{}", thumb.file_name));

                    self.attachments.push(Attachment {
                        name: thumb.file_name.clone(),
                        uploader: thumb.into(),
                    });
                };

                self.media.push(MediaEntry::Video(edit_function(video)));

                self.attachments.push(Attachment {
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

                self.media.push(MediaEntry::Video(edit_function(video)));
            }
            Uploader::Url(url) => {
                let video = MediaVideo {
                    r#type: "video".into(),
                    media: url.0.clone(),
                    ..Default::default()
                };

                self.media.push(MediaEntry::Video(edit_function(video)));
            }
            Uploader::Empty => (),
        }

        self
    }
}
