use futures::Future;

use crate::error::BotError;
use crate::input::SendVideoNote;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_video_note(
        self,
        send_video_note: SendVideoNote,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_video_note.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendVideoNote"), self)
            .with_query(query)
            .with_uploader("video_note", uploader)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::file;
    use crate::input::SendVideoNote;
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn send_video_note() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let video_note = file("res/video.mp4").unwrap();

        let arg = SendVideoNote::new(chat_id, video_note);

        if let Err(err) = runtime.block_on(bot.send_video_note(arg)) {
            panic!("{:#?}", err);
        }
    }

}
