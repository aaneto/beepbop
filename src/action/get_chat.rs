use futures::Future;

use crate::error::BotError;
use crate::input::ChatID;
use crate::input::GetChat;
use crate::object::Chat;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn get_chat<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, Chat), Error = BotError> {
        let get_chat = GetChat::new(id.into());

        TelegramRequest::new(Method::GET, self.get_route(&"getChat"), self)
            .with_query(get_chat)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn get_chat() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) = runtime.block_on(bot.get_chat(chat_id)) {
            panic!("{:#?}", err);
        }
    }
}
