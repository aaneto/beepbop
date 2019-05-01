use futures::Future;

use crate::input::ChatID;
use crate::input::ExportChatInviteLink;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    pub fn export_chat_invite_link<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, String), Error = BotError>
    {
        let arg = ExportChatInviteLink::new(id.into());

        TelegramRequest::new(Method::GET, self.get_route(&"exportChatInviteLink"), self)
            .with_query(arg)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn test_export_chat_invite_link() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) = runtime.block_on(bot.export_chat_invite_link(chat_id)) {
            panic!("{:#?}", err);
        }
    }
}
