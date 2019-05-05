use futures::Future;

use crate::error::BotError;
use crate::input::SendVoice;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_voice(
        self,
        send_voice: SendVoice,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, voice) = send_voice.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendVoice"), self)
            .with_query(query)
            .with_uploader("voice", voice)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::file;
    use crate::input::SendVoice;
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn test_send_voice() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let voice = file("res/voice.ogg").unwrap();

        let arg = SendVoice::new(chat_id, voice);

        if let Err(err) = runtime.block_on(bot.send_voice(arg)) {
            panic!("{:#?}", err);
        }
    }

}
