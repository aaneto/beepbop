use futures::Future;

use crate::api::datatypes::User;
use crate::api::error::APIError;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    pub fn get_me(self) -> impl Future<Item = (Self, User), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getMe"), self).execute()
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use tokio::runtime::Runtime;
    use crate::util::get_argv;
    use crate::api::Bot;


    #[test]
    fn test_get_me() {
        let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.get_me()) {
            panic!(error.description().to_owned());
        }
    }
}