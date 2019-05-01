use futures::Future;

use crate::error::BotError;
use crate::input::GetFile;
use crate::object::FileInfo;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn get_file(
        self,
        file_id: String,
    ) -> impl Future<Item = (Self, FileInfo), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getFile"), self)
            .with_query(GetFile::new(file_id))
            .execute()
    }
}
