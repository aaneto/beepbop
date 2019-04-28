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
        } = send_document;

        TelegramRequest::new(Method::POST, self.get_route(&"sendDocument"), self)
            .with_query(query_data)
            .with_uploader("document", document_uploader)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use futures::Future;
    use tokio::runtime::Runtime;
    use crate::util::get_argv;
    use crate::api::Bot;
    use crate::api::uploaders::FileUploader;
    use crate::api::uploaders::add_mime;
    use crate::api::uploaders::add_thumbnail;
    use crate::api::args::SendDocument;

    #[test]
    fn document_upload_thumbnail() {
        let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = get_argv("CHAT_ID")
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

    #[test]
    fn photo_reupload() {
        let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = get_argv("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let fut = bot.get_chat(chat_id.clone())
            .and_then(|(bot, chat)| {
                let file_id = chat.photo.unwrap().big_file_id;

                bot.download_file(file_id)
            })
            .and_then(move |(bot, file_buffer)| {
                let dll_file = FileUploader::from_file(file_buffer);
                let send_doc = SendDocument::new(chat_id, dll_file);

                bot.send_document(send_doc)
            })
            .map(|(_, message)| println!("{:?}", message))
            .map_err(|data| println!("{:?}", data));

        if let Err(err) = runtime.block_on(fut) {
            panic!("{:#?}", err);
        }
    }

}