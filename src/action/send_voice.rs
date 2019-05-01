use futures::Future;

use crate::input::SendVoice;
use crate::objects::Message;
use crate::error::BotError;
use crate::input::Uploader;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    pub fn send_voice<T: Uploader + Default>(
        self,
        send_voice: SendVoice<T>,
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
    use crate::input::SendVoice;
    use crate::input::FileUploader;
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

        let voice = FileUploader::new("res/voice.ogg").unwrap();

        let arg = SendVoice::new(chat_id, voice);

        if let Err(err) = runtime.block_on(bot.send_voice(arg)) {
            panic!("{:#?}", err);
        }
    }

}
