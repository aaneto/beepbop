use futures::Future;

use crate::api::args::RestrictChatMember;
use crate::api::error::APIError;
use crate::api::methods::Method;
use crate::api::methods::TelegramRequest;
use crate::api::Bot;

impl Bot {
    pub fn restrict_chat_member(
        self,
        restrict_member: RestrictChatMember,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest::new(Method::GET, self.get_route(&"restrictChatMember"), self)
            .with_query(restrict_member)
            .execute()
    }
}
