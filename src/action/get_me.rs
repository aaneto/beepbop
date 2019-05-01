use futures::Future;

use crate::error::BotError;
use crate::object::User;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn get_me(self) -> impl Future<Item = (Self, User), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getMe"), self).execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::Bot;
    use std::env::var;
    use std::error::Error;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_me() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.get_me()) {
            panic!(error.description().to_owned());
        }
    }
}
