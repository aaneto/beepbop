use futures::Future;

use crate::api::args::GetUpdates;
use crate::api::datatypes::Update;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn get_updates(
        self,
        get_updates: GetUpdates,
    ) -> impl Future<Item = (Self, Vec<Update>), Error = APIError> {
        TelegramRequest::new(Method::POST, self.get_route(&"getUpdates"), self)
            .with_body(get_updates)
            .execute()
    }
}
