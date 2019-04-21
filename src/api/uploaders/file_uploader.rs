use futures::future::Future;
use reqwest::r#async::multipart::Form;
use reqwest::r#async::multipart::Part;
use std::path::PathBuf;

use crate::api::uploaders::IdPostUploader;
use crate::api::uploaders::Uploader;
use crate::api::uploaders::UploaderError;
use crate::api::TelegramRequest;

#[derive(Debug)]
pub struct FileUploader {
    part: Part,
}

impl FileUploader {
    pub fn new<P: Into<PathBuf>>(path_into: P, mime_string: &str) -> Result<Self, UploaderError> {
        let file_path: PathBuf = path_into.into();

        let file_name = file_path
            .file_name()
            .ok_or(UploaderError::NoFileName)
            .and_then(|name| name.to_str().ok_or(UploaderError::InvalidUTF))
            .map(std::borrow::ToOwned::to_owned);

        let read = tokio::fs::read(file_path);

        let mut part = Part::stream(read.into_stream());

        part = part
            .mime_str(mime_string)
            .map_err(UploaderError::WrongMime)?;
        part = part.file_name(file_name?);

        Ok(Self { part })
    }
}

impl Uploader for FileUploader {
    fn upload_into(self, tag: &str, builder: TelegramRequest) -> TelegramRequest {
        builder.with_multipart(Form::new().part(tag.to_owned(), self.part))
    }
}

impl IdPostUploader for FileUploader {}
