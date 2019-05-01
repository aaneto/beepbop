use futures::Future;

use crate::error::BotError;
use crate::input::RestrictChatMember;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn restrict_chat_member(
        self,
        restrict_member: RestrictChatMember,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"restrictChatMember"), self)
            .with_query(restrict_member)
            .execute()
    }
}
