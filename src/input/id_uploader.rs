use crate::input::IdPostUploader;
use crate::input::Uploader;
use crate::telegram_request::TelegramRequest;

#[derive(Clone, Default, Debug)]
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
