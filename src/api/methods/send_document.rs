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
    pub fn send_document<U: Uploader + Default>(
        self,
        send_document: SendDocument<U>,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        let (query, uploader) = send_document.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendDocument"), self)
            .with_query(query)
            .with_uploader("document", uploader)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::args::SendDocument;
    use crate::api::uploaders::add_mime;
    use crate::api::uploaders::add_thumbnail;
    use crate::api::uploaders::FileUploader;
    use crate::api::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn document_upload_thumbnail() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let pupper_thumbnail = FileUploader::new("res/puppy.jpg")
            .and_then(add_mime("image/jpg"))
            .unwrap();

        let text_file = FileUploader::new("res/some_text")
            .and_then(add_mime("text/plain"))
            .map(add_thumbnail(pupper_thumbnail))
            .unwrap();

        let arg = SendDocument::new(chat_id, text_file);

        if let Err(err) = runtime.block_on(bot.send_document(arg)) {
            panic!("{:#?}", err);
        }
    }

}
