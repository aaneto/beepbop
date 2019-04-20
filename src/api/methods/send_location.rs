use futures::Future;

use crate::api::args::SendLocation;
use crate::api::datatypes::Message;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn send_location(
        self,
        send_location: SendLocation,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendLocation"), self)
            .with_query(send_location)
            .execute()
    }
}
