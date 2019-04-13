use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<RawMessage>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawMessage {
    pub message_id: i64,
    pub date: i64,
    pub from: User,
    pub text: Option<String>,
    pub new_chat_members: Option<Vec<User>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub date: i64,
    pub from: User,
    pub chat: Chat,
    pub text: Option<String>,
    pub new_chat_members: Option<Vec<User>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: u64,
    pub invoice_payload: String,
    pub shipping_option_id: String,
    pub order_info: OrderInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInfo {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateData {
    #[serde(rename = "message")]
    Message(Message),
    #[serde(rename = "edited_message")]
    EditedMessage(Message),
    #[serde(rename = "channel_post")]
    ChannelPost(Message),
    #[serde(rename = "edited_channel_post")]
    EditChannelPost(Message),
    #[serde(rename = "inline_query")]
    InlineQuery(InlineQuery),
    #[serde(rename = "chosen_inline_result")]
    ChosenInlineResult(ChosenInlineResult),
    #[serde(rename = "callback_query")]
    CallbackQuery(CallbackQuery),
    #[serde(rename = "shipping_query")]
    ShippingQuery(ShippingQuery),
    #[serde(rename = "pre_checkout_query")]
    PrecheckoutQuery(PrecheckoutQuery),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(flatten)]
    pub data: UpdateData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct FileInfo {
    pub file_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}

pub struct FileBuffer {
    pub data: Vec<u8>,
    pub name: String,
}

impl FileBuffer {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        FileBuffer { name, data }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatMember {
    user: User,
    status: String,
    until_date: Option<u64>,
    can_be_edited: Option<bool>,
    can_change_info: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_invite_users: Option<bool>,
    can_restrict_members: Option<bool>,
    can_pin_messages: Option<bool>,
    can_promote_members: Option<bool>,
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
}
