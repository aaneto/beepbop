use futures::Future;

use crate::api::args::SendDocument;
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
    pub fn send_document<U: Uploader>(
        self,
        send_document: SendDocument<U>,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        let SendDocument {
            query: query_data,
            document: document_uploader,
            thumbnail: thumbnail_uploader_option,
        } = send_document;

        let mut request = TelegramRequest::new(Method::POST, self.get_route(&"sendDocument"), self);

        if let Some(thumbnail_uploader) = thumbnail_uploader_option {
            request = request.with_uploader("thumb", thumbnail_uploader);
        }

        request
            .with_query(query_data)
            .with_uploader("document", document_uploader)
            .execute()
    }
}
