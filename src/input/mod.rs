use crate::telegram_request::TelegramRequest;
use std::error::Error;

#[derive(Debug)]
pub enum UploaderError {
    WrongMime(reqwest::Error),
    NoFileName,
    InvalidUTF,
}

impl std::fmt::Display for UploaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            UploaderError::WrongMime(err) => err.fmt(f),
            UploaderError::NoFileName => "No file name.".fmt(f),
            UploaderError::InvalidUTF => "Invalid UTF-8 on path.".fmt(f),
        }
    }
}

impl Error for UploaderError {
    fn description(&self) -> &str {
        match self {
            UploaderError::WrongMime(err) => err.description(),
            UploaderError::NoFileName => "No file name.",
            UploaderError::InvalidUTF => "Invalid UTF-8 on path",
        }
    }
}

/// Files can be uploaded in many formats:
/// ## By Url
/// Telegram will download the file at the url.
/// (Max 5 MB for photos and 20MB for other content)
///
/// ## By Fileid
/// Just send the Id on telegram, and the file will
/// be sent with the original mime type and metadata.
///
/// ## By POST
/// Send a local file using multipart upload.
/// (Max 10MB for photos and 50MB for other content)
///
/// Filed and URL are just sent by query while POST is a multipart form.
pub trait Uploader: std::fmt::Debug {
    /// Get a RequestBuilder and add self to it as query or multiform data.
    fn upload_into(self, tag: &str, builder: TelegramRequest) -> TelegramRequest;
}

/// A trait for when we want to upload
/// either by Id or by file.
pub trait IdPostUploader: Uploader {}

pub fn file<P>(path: P) -> Result<FileUploader, UploaderError>
where
    P: Into<std::path::PathBuf>,
{
    FileUploader::new(path)
}

pub fn file_id(id: &str) -> IdUploader {
    IdUploader::new(id)
}

pub fn file_link(link: &str) -> LinkUploader {
    LinkUploader::new(link)
}

pub mod chat_id;
pub mod export_chat_invite_link;
pub mod file_uploader;
pub mod force_reply;
pub mod forward_message;
pub mod get_chat;
pub mod get_chat_member;
pub mod get_file;
pub mod get_updates;
pub mod id_uploader;
pub mod inline_keyboard_button;
pub mod inline_keyboard_button_message;
pub mod inline_keyboard_markup;
pub mod keyboard_button;
pub mod link_uploader;
pub mod pin_message;
pub mod promote_chat_member;
pub mod reply_board_markup;
pub mod reply_keyboard_remove;
pub mod reply_markup;
pub mod restrict_chat_member;
pub mod send_animation;
pub mod send_audio;
pub mod send_contact;
pub mod send_document;
pub mod send_location;
pub mod send_message;
pub mod send_photo;
pub mod send_video;
pub mod send_video_note;
pub mod send_voice;
pub mod set_chat_description;
pub mod set_chat_sticker_set;
pub mod set_chat_title;

pub use chat_id::*;
pub use export_chat_invite_link::*;
pub use file_uploader::*;
pub use force_reply::*;
pub use forward_message::*;
pub use get_chat::*;
pub use get_chat_member::*;
pub use get_file::*;
pub use get_updates::*;
pub use id_uploader::*;
pub use inline_keyboard_button::*;
pub use inline_keyboard_button_message::*;
pub use inline_keyboard_markup::*;
pub use keyboard_button::*;
pub use link_uploader::*;
pub use pin_message::*;
pub use promote_chat_member::*;
pub use reply_board_markup::*;
pub use reply_keyboard_remove::*;
pub use reply_markup::*;
pub use restrict_chat_member::*;
pub use send_animation::*;
pub use send_audio::*;
pub use send_contact::*;
pub use send_document::*;
pub use send_location::*;
pub use send_message::*;
pub use send_photo::*;
pub use send_video::*;
pub use send_video_note::*;
pub use send_voice::*;
pub use set_chat_description::*;
pub use set_chat_sticker_set::*;
pub use set_chat_title::*;
