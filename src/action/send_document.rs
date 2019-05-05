use futures::Future;

use crate::error::BotError;
use crate::input::SendDocument;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_document(
        self,
        send_document: SendDocument,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_document.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendDocument"), self)
            .with_query(query)
            .with_uploader("document", uploader)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::add_mime;
    use crate::input::add_thumbnail;
    use crate::input::file;
    use crate::input::SendDocument;
    use crate::Bot;
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

        let pupper_thumbnail = file("res/puppy.jpg")
            .and_then(add_mime("image/jpg"))
            .unwrap();

        let text_file = file("res/some_text")
            .and_then(add_mime("text/plain"))
            .map(add_thumbnail(pupper_thumbnail))
            .unwrap();

        let arg = SendDocument::new(chat_id, text_file);

        if let Err(err) = runtime.block_on(bot.send_document(arg)) {
            panic!("{:#?}", err);
        }
    }

}
