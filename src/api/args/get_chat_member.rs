use serde_derive::Serialize;

use crate::api::args::ChatID;

#[derive(Default, Debug, Serialize)]
pub struct GetChatMember {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub user_id: i64,
}