use optional_builder::optional_builder;

use serde_derive::Serialize;

use crate::api::args::ChatID;

#[optional_builder]
#[derive(Default, Debug, Serialize)]
pub struct PromoteChatMember {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub user_id: i64,
    pub can_change_info: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_promote_members: Option<bool>,
}

impl PromoteChatMember {
    pub fn new<ID>(chat_id: ID, user_id: i64) -> Self
    where
        ID: Into<ChatID>,
    {
        PromoteChatMember {
            chat_id: chat_id.into(),
            user_id,
            ..Default::default()
        }
    }
}
