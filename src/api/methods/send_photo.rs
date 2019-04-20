use futures::Future;

use crate::api::args::SendPhoto;
use crate::api::datatypes::Message;
use crate::api::error::APIError;
use crate::api::uploaders::Uploader;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    /// Send a photo in telegram.
    ///
    /// Photos can be uploaded by Id, Url and Post
    /// methods. Note that chat photo id's are only
    /// usable for downloading a chat photo, not here.
    pub fn send_photo<U: Uploader>(
        self,
        send_photo: SendPhoto,
        file_uploader: U,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest::new(Method::POST, self.get_route(&"sendPhoto"), self)
            .with_query(send_photo)
            .with_uploader("photo", file_uploader)
            .execute()
    }
}
