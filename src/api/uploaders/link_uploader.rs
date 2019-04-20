use crate::api::methods::TelegramRequest;
use crate::api::uploaders::Uploader;

#[derive(Debug)]
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
