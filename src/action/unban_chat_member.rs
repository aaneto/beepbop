use futures::Future;

use crate::error::BotError;
use crate::input::{ChatID, UnbanChatMember};
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn unban_chat_member<ID: Into<ChatID>>(
        self,
        id: ID,
        user_id: i64,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let arg = UnbanChatMember {
            chat_id: id.into(),
            user_id,
        };

        TelegramRequest::new(Method::GET, self.get_route(&"unbanChatMember"), self)
            .with_query(arg)
            .execute()
    }
}


#[cfg(test)]
mod tests {
    use crate::Bot;
    use std::env::var;
    use std::error::Error;
    use tokio::runtime::Runtime;

    #[test]
    #[ignore]
    fn test_unban_chat_member() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let user_id: i64 = var("USER_ID")
            .expect("Cannot find USER_ID in ENV")
            .parse()
            .expect("USER_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.unban_chat_member(chat_id, user_id)) {
            panic!(error.description().to_owned());
        }
    }
}
