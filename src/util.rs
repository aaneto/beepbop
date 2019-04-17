use std::error::Error;
use std::path::PathBuf;

use futures::Future;

use reqwest::r#async::multipart::Form;
use reqwest::r#async::multipart::Part;

use crate::api::methods::TelegramRequest;

/// Find a ENV variable by key
pub fn get_argv(key: &str) -> Option<String> {
    std::env::vars()
        .find(|(k, _)| *k == *key)
        .map(|(_, value)| value)
}

#[derive(Debug)]
pub enum FileUploaderError {
    WrongMime(reqwest::Error),
    NoFileName,
    InvalidUTF,
}

impl std::fmt::Display for FileUploaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FileUploaderError::WrongMime(err) => err.fmt(f),
            FileUploaderError::NoFileName => "No file name.".fmt(f),
            FileUploaderError::InvalidUTF => "Invalid UTF-8 on path.".fmt(f),
        }
    }
}

impl Error for FileUploaderError {
    fn description(&self) -> &str {
        match self {
            FileUploaderError::WrongMime(err) => err.description(),
            FileUploaderError::NoFileName => "No file name.",
            FileUploaderError::InvalidUTF => "Invalid UTF-8 on path",
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
pub trait FileUploader {
    /// Get a RequestBuilder and add self to it as query or multiform data.
    fn upload_into(self, builder: TelegramRequest) -> TelegramRequest;
}

pub struct PostUploader {
    form: Form,
}

impl PostUploader {
    pub fn new(
        file_path: PathBuf,
        tag: &str,
        mime_string: &str,
    ) -> Result<Self, FileUploaderError> {
        let tag = tag.to_owned();

        let file_name = file_path
            .file_name()
            .ok_or(FileUploaderError::NoFileName)
            .and_then(|name| name.to_str().ok_or(FileUploaderError::InvalidUTF))
            .map(|name_str| name_str.to_owned());

        let read = tokio::fs::read(file_path);

        let mut form = Form::new();
        let mut part = Part::stream(read.into_stream());

        part = part
            .mime_str(mime_string)
            .map_err(|err| FileUploaderError::WrongMime(err))?;
        part = part.file_name(file_name?);
        form = form.part(tag, part);

        Ok(Self { form })
    }
}

impl FileUploader for PostUploader {
    fn upload_into(self, builder: TelegramRequest) -> TelegramRequest {
        builder.with_multipart(self.form)
    }
}

pub struct IdUploader {
    tag: String,
    id: String,
}

impl IdUploader {
    pub fn new(tag: &str, id: &str) -> Self {
        Self {
            tag: tag.to_owned(),
            id: id.to_owned(),
        }
    }
}

impl FileUploader for IdUploader {
    fn upload_into(self, builder: TelegramRequest) -> TelegramRequest {
        builder.with_query(&[(self.tag, self.id)])
    }
}

pub struct UrlUploader {
    tag: String,
    url: String,
}

impl UrlUploader {
    pub fn new(tag: &str, url: &str) -> Self {
        Self {
            tag: tag.to_owned(),
            url: url.to_owned(),
        }
    }
}

impl FileUploader for UrlUploader {
    fn upload_into(self, builder: TelegramRequest) -> TelegramRequest {
        builder.with_query(&[(self.tag, self.url)])
    }
}

/// A trait for when we want to upload
/// either by Id or by Posting.
trait IdPostUploader: FileUploader {}

impl IdPostUploader for IdUploader {}
impl IdPostUploader for PostUploader {}
