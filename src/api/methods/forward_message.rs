use futures::Future;

use crate::api::args::ChatID;
use crate::api::args::ForwardMessage;
use crate::api::datatypes::Message;
use crate::api::error::APIError;
use crate::api::Bot;
use crate::api::Method;
use crate::api::TelegramRequest;

impl Bot {
    pub fn forward_message<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        from_chat_id: ID,
        message_id: i64,
        disable_notification: bool,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        let args = ForwardMessage {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification,
        };

        TelegramRequest::new(Method::GET, self.get_route(&"forwardMessage"), self)
            .with_query(args)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Bot;
    use crate::util::get_argv;
    use tokio::runtime::Runtime;

    #[test]
    fn resend_message() {
        let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");

        let message_id: i64 = get_argv("MESSAGE_ID")
            .expect("Cannot find MESSAGE_ID in ENV")
            .parse()
            .expect("MESSAGE_ID is not an valid ID.");

        let from_chat_id: i64 = get_argv("FROM_CHAT_ID")
            .expect("Cannot find FROM_CHAT_ID in ENV")
            .parse()
            .expect("FROM_CHAT_ID is not an valid ID.");

        let to_chat_id: i64 = get_argv("TO_CHAT_ID")
            .expect("Cannot find TO_CHAT_ID in ENV")
            .parse()
            .expect("TO_CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) =
            runtime.block_on(bot.forward_message(to_chat_id, from_chat_id, message_id, false))
        {
            panic!("{:#?}", err);
        }
    }
}
