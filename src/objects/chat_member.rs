use serde_derive::Deserialize;

use crate::objects::User;

#[derive(Debug, Deserialize)]
pub struct ChatMember {
    pub user: User,
    pub status: String,
    pub until_date: Option<u64>,
    pub can_be_edited: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_post_messages: Option<bool>,
    pub can_edit_messages: Option<bool>,
    pub can_delete_messages: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_restrict_members: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_promote_members: Option<bool>,
    pub can_send_messages: Option<bool>,
    pub can_send_media_messages: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
}
