use optional_builder::optional_builder;

use serde_derive::Serialize;

use crate::api::args::ChatID;

#[optional_builder]
#[derive(Default, Debug, Serialize)]
pub struct RestrictChatMember {
    pub chat_id: ChatID,
    pub user_id: i64,
    pub until_date: Option<u64>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
}

impl RestrictChatMember {
    pub fn new<ID>(chat_id: ID, user_id: i64) -> Self
    where
        ID: Into<ChatID>,
    {
        RestrictChatMember {
            chat_id: chat_id.into(),
            user_id,
            ..Default::default()
        }
    }
}
