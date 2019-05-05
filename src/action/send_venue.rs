use futures::Future;

use crate::error::BotError;
use crate::input::SendVenue;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn send_venue(
        self,
        send_venue: SendVenue,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendVenue"), self)
            .with_query(send_venue)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::SendVenue;
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn send_venue() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let arg =
            SendVenue::new(chat_id, 20.0, 30.5, "Dunno", "Somewhere").with_foursquare_id("sddd");

        if let Err(err) = runtime.block_on(bot.send_venue(arg)) {
            panic!("{:#?}", err);
        }
    }
}
