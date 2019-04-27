use crate::api::TelegramRequest;
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

pub mod file_uploader;
pub mod id_uploader;
pub mod link_uploader;

pub use file_uploader::*;
pub use id_uploader::*;
pub use link_uploader::*;
