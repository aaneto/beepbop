use futures::Future;

use crate::error::BotError;
use crate::input::Action;
use crate::input::SendChatAction;
use crate::input::ChatID;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_chat_action<ID: Into<ChatID>>(
        self,
        id: ID,
        action: Action,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let arg = SendChatAction::new(id.into(), action);

        TelegramRequest::new(Method::GET, self.get_route(&"sendChatAction"), self)
            .with_query(arg)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::Bot;
    use crate::input::Action;
    use std::env::var;
    use std::error::Error;
    use tokio::runtime::Runtime;

    #[test]
    fn test_send_chat_action() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.send_chat_action(chat_id, Action::RecordAudio)) {
            panic!(error.description().to_owned());
        }
    }
}

