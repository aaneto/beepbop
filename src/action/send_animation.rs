use futures::Future;

use crate::error::BotError;
use crate::input::SendAnimation;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_animation(
        self,
        send_animation: SendAnimation,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_animation.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendAnimation"), self)
            .with_query(query)
            .with_uploader("animation", uploader)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::file;
    use crate::input::SendAnimation;
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    #[ignore]
    fn test_send_animation() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let animation = file("res/anim.gif").unwrap();

        let arg = SendAnimation::new(chat_id, animation);

        if let Err(err) = runtime.block_on(bot.send_animation(arg)) {
            panic!("{:#?}", err);
        }
    }

}
