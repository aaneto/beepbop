use futures::Future;

use crate::api::datatypes::User;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn get_me(self) -> impl Future<Item = (Self, User), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getMe"), self).execute()
    }
}
