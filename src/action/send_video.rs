use futures::Future;

use crate::error::BotError;
use crate::input::SendVideo;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_video(
        self,
        send_video: SendVideo,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_video.split();

        TelegramRequest::new(Method::POST, self.get_route(&"SendVideo"), self)
            .with_query(query)
            .with_uploader("video", uploader)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::add_mime;
    use crate::input::add_thumbnail;
    use crate::input::file;
    use crate::input::SendVideo;
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn send_video() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let puppy_photo = file("res/puppy.jpg").unwrap();

        let video = file("res/video.mp4")
            .and_then(add_mime("video/mp4"))
            .map(add_thumbnail(puppy_photo))
            .unwrap();

        let arg = SendVideo::new(chat_id, video);

        if let Err(err) = runtime.block_on(bot.send_video(arg)) {
            panic!("{:#?}", err);
        }
    }

}
