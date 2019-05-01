use crate::input::Uploader;
use crate::telegram_request::TelegramRequest;

#[derive(Clone, Default, Debug)]
pub struct LinkUploader {
    url: String,
}

impl LinkUploader {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}

impl Uploader for LinkUploader {
    fn upload_into(self, tag: &str, builder: TelegramRequest) -> TelegramRequest {
        builder.with_query(&[(tag, self.url)])
    }
}
