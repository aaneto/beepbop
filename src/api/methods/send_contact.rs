use futures::Future;

use crate::api::args::SendContact;
use crate::api::datatypes::Message;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn send_contact(
        self,
        send_contact: SendContact,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendContact"), self)
            .with_query(send_contact)
            .execute()
    }
}
