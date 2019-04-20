use futures::Future;

use crate::api::args::GetFile;
use crate::api::datatypes::FileInfo;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn get_file(
        self,
        file_id: String,
    ) -> impl Future<Item = (Self, FileInfo), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getFile"), self)
            .with_query(GetFile::new(file_id))
            .execute()
    }
}
