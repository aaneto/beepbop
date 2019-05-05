use std::path::PathBuf;

use futures::future::Future;
use reqwest::r#async::multipart::Part;

use crate::input::UploaderError;
use crate::object::FileBuffer;

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
    pub part: Part,
    pub thumbnail: Option<Part>,
    pub file_name: String,
}

/// Add a mime type to a FileUploader.
///
/// This is designed to be combined with and_then when
/// creating an FileUploader.
///
/// ```rust
/// use beepbop::input::{FileUploader, add_mime};
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
/// use beepbop::input::{FileUploader, add_mime, add_thumbnail};
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
                part: part.file_name(name.clone()),
                thumbnail: None,
                file_name: name,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn from_bytes(name: &str, bytes: Vec<u8>) -> Self {
        let file_name = name.to_owned();
        let part = Part::bytes(bytes).file_name(file_name.clone());

        Self {
            part,
            thumbnail: None,
            file_name,
        }
    }

    pub fn from_file(file: FileBuffer) -> Self {
        let file_name = file.name.replace("/", "_").to_owned();
        let part = Part::bytes(file.data).file_name(file_name.clone());

        Self {
            part,
            thumbnail: None,
            file_name
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
