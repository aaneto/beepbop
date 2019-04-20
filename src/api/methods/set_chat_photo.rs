use futures::Future;

use crate::api::args::ChatID;
use crate::api::error::APIError;
use crate::api::uploaders::FileUploader;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    pub fn set_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        file_uploader: FileUploader,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::POST, self.get_route(&"setChatPhoto"), self)
            .with_query(chat_id.into())
            .with_uploader("photo", file_uploader)
            .execute()
    }
}
