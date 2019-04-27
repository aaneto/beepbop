use futures::future::Future;
use reqwest::r#async::multipart::Part;
use std::path::PathBuf;

use crate::api::uploaders::IdPostUploader;
use crate::api::uploaders::Uploader;
use crate::api::uploaders::UploaderError;
use crate::api::TelegramRequest;

/// The FileUploader is an proxy object
/// for a FileUploader Future:
///
/// This object represents a
/// file that can be uploaded asynchronously, the file
/// is encoded as a multipart request.
///
/// It is possible to add a thumbnail to this File.
#[derive(Debug)]
pub struct FileUploader {
    part: Part,
    thumbnail: Option<Part>,
}

/// Add a mime type to a FileUploader.
///
/// This is designed to be combined with and_then when
/// creating an FileUploader.
///
/// ```rust
/// use telegrambot::prelude::*;
///
/// let pupper_thumbnail = FileUploader::new("res/puppy.jpg")
///     .and_then(add_mime("image/jpg"));
///
/// assert!(pupper_thumbnail.is_ok());
/// ```
pub fn add_mime(
    mime_str: &str,
) -> impl FnOnce(FileUploader) -> Result<FileUploader, UploaderError> {
    let mime_string = mime_str.to_owned();

    move |uploader: FileUploader| uploader.with_mime(&mime_string)
}

/// Add a thumbnail to a FileUploader.
///
/// This is designed to be combined with and_then when
/// creating an FileUploader.
///
/// ```rust
/// use telegrambot::prelude::*;
///
/// let pupper_thumbnail = FileUploader::new("res/puppy.jpg")
///     .and_then(add_mime("image/jpg"))
///     .unwrap();
///
/// let text_file = FileUploader::new("res/some_text")
///     .and_then(add_mime("text/plain"))
///     .map(add_thumbnail(pupper_thumbnail));
///
/// assert!(text_file.is_ok());
/// ```
pub fn add_thumbnail(thumbnail: FileUploader) -> impl FnOnce(FileUploader) -> FileUploader {
    move |uploader: FileUploader| uploader.with_thumbnail(thumbnail)
}

impl FileUploader {
    /// Create a new FileUploader from a Path.
    pub fn new<P: Into<PathBuf>>(path_into: P) -> Result<Self, UploaderError> {
        let file_path: PathBuf = path_into.into();

        let file_name = file_path
            .file_name()
            .ok_or(UploaderError::NoFileName)
            .and_then(|name| name.to_str().ok_or(UploaderError::InvalidUTF))
            .map(std::borrow::ToOwned::to_owned);

        let read = tokio::fs::read(file_path);

        let part = Part::stream(read.into_stream());

        match file_name {
            Ok(name) => Ok(Self {
                part: part.file_name(name),
                thumbnail: None,
            }),
            Err(err) => Err(err),
        }
    }

    /// Try to add a mime type to the FileUploader.
    pub fn with_mime(mut self, mime_string: &str) -> Result<Self, UploaderError> {
        self.part = self
            .part
            .mime_str(mime_string)
            .map_err(UploaderError::WrongMime)?;

        Ok(self)
    }

    /// Add an thumbnail to the FileUploader.
    pub fn with_thumbnail(mut self, uploader: FileUploader) -> Self {
        self.thumbnail = Some(uploader.part);

        self
    }
}

impl Uploader for FileUploader {
    /// Upload itself into the request by using multipart form data.
    fn upload_into(self, tag: &str, builder: TelegramRequest) -> TelegramRequest {
        let mut request = builder.with_form_part(tag, self.part);

        if let Some(thumbnail) = self.thumbnail {
            request = request.with_form_part("thumb", thumbnail);
        }

        request
    }
}

impl IdPostUploader for FileUploader {}
