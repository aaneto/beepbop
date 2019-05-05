use futures::Future;

use crate::error::BotError;
use crate::input::SendAudio;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_audio(
        self,
        send_audio: SendAudio,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, voice) = send_audio.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendAudio"), self)
            .with_query(query)
            .with_uploader("audio", voice)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::file;
    use crate::input::SendAudio;
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn test_send_audio() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let voice = file("res/sound.mp3").unwrap();

        let arg = SendAudio::new(chat_id, voice);

        if let Err(err) = runtime.block_on(bot.send_audio(arg)) {
            panic!("{:#?}", err);
        }
    }

}
