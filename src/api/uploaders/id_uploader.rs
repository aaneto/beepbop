use crate::api::uploaders::IdPostUploader;
use crate::api::uploaders::Uploader;
use crate::api::TelegramRequest;

#[derive(Clone, Debug)]
pub struct IdUploader {
    id: String,
}

impl IdUploader {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_owned() }
    }
}

impl Uploader for IdUploader {
    fn upload_into(self, tag: &str, builder: TelegramRequest) -> TelegramRequest {
        builder.with_query(&[(tag, self.id)])
    }
}

impl IdPostUploader for IdUploader {}
