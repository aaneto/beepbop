use futures::Future;

use crate::input::PromoteChatMember;
use crate::error::BotError;
use crate::Bot;
use crate::telegram_request::{Method, TelegramRequest};

impl Bot {
    /// Promote a chat member.
    ///
    /// Note that the promotions are of type Option<bool>
    /// because:
    ///
    /// None => Don't modify this permission
    /// Some(false) => Deny this permission
    /// Some(true) => Grant this permission
    ///
    /// 1. Admins cannot demote/promote the creator of the group.
    /// 2. Admins cannot demote/promote other admins.
    pub fn promote_chat_member(
        self,
        promote_member: PromoteChatMember,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"promoteChatMember"), self)
            .with_query(promote_member)
            .execute()
    }
}
