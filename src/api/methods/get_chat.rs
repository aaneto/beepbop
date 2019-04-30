use futures::Future;

use crate::api::args::ChatID;
use crate::api::args::GetChat;
use crate::api::datatypes::Chat;
use crate::api::error::APIError;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    pub fn get_chat<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, Chat), Error = APIError> {
        let get_chat = GetChat::new(id.into());

        TelegramRequest::new(Method::GET, self.get_route(&"getChat"), self)
            .with_query(get_chat)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_chat() {
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
