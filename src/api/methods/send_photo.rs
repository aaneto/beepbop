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
        send_photo: SendPhoto<U>,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        let SendPhoto {
            query: query_data,
            photo_uploader,
        } = send_photo;

        TelegramRequest::new(Method::POST, self.get_route(&"sendPhoto"), self)
            .with_query(query_data)
            .with_uploader("photo", photo_uploader)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::args::SendPhoto;
    use crate::api::datatypes::Message;
    use crate::api::error::APIError;
    use crate::api::uploaders::add_mime;
    use crate::api::uploaders::FileUploader;
    use crate::api::Bot;
    use crate::util::get_argv;
    use futures::Future;
    use tokio::runtime::Runtime;

    fn send_photo_future(
        file_name: &str,
        mime_string: Option<&str>,
    ) -> impl Future<Item = (Bot, Message), Error = APIError> {
        let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = get_argv("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut uploader_res = FileUploader::new(file_name);

        if let Some(mime_str) = mime_string {
            uploader_res = uploader_res.and_then(add_mime(mime_str));
        }

        let arg = SendPhoto::new(chat_id, uploader_res.unwrap());
        bot.send_photo(arg)
    }

    #[test]
    fn send_photo() {
        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) =
            runtime.block_on(send_photo_future("res/brownpuppy.png", Some("image/png")))
        {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn send_photo_without_mime() {
        let mut runtime = Runtime::new().expect("Unable to create a runtime");
        if let Err(err) = runtime.block_on(send_photo_future("res/brownpuppy.png", None)) {
            panic!("{:#?}", err);
        }
    }
}
