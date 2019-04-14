use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct GetUpdateArgs {
    pub offset: i64,
    pub limit: i64,
    pub timeout: i64,
    pub allowed_updates: Vec<String>,
}

impl Default for GetUpdateArgs {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 100,
            timeout: 0,
            allowed_updates: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SetChatDescription {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub description: String,
}

impl SetChatDescription {
    pub fn new(chat_id: ChatID, description: String) -> Self {
        Self {
            chat_id,
            description,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SetChatTitle {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub title: String,
}

impl SetChatTitle {
    pub fn new(chat_id: ChatID, title: String) -> Self {
        Self { chat_id, title }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ChatID {
    Integer { chat_id: i64 },
    String { chat_id: String },
}

impl Default for ChatID {
    fn default() -> Self {
        ChatID::Integer { chat_id: 0i64 }
    }
}

impl From<i64> for ChatID {
    fn from(id: i64) -> Self {
        ChatID::Integer { chat_id: id }
    }
}

impl From<i32> for ChatID {
    fn from(id: i32) -> Self {
        ChatID::Integer { chat_id: id as i64 }
    }
}

impl From<u32> for ChatID {
    fn from(id: u32) -> Self {
        ChatID::Integer { chat_id: id as i64 }
    }
}

impl From<String> for ChatID {
    fn from(id: String) -> Self {
        ChatID::String { chat_id: id }
    }
}

#[derive(Debug, Serialize)]
pub struct GetChat {
    #[serde(flatten)]
    pub chat_id: ChatID,
}

impl GetChat {
    pub fn new(chat_id: ChatID) -> Self {
        Self { chat_id }
    }
    pub fn from_integer(id: i64) -> Self {
        Self {
            chat_id: ChatID::Integer { chat_id: id },
        }
    }

    pub fn from_string(id: String) -> Self {
        Self {
            chat_id: ChatID::String { chat_id: id },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ForceReply {
    pub force_reply: bool,
    pub selective: bool,
}

impl Default for ForceReply {
    fn default() -> Self {
        Self {
            force_reply: true,
            selective: false,
        }
    }
}

impl ForceReply {
    pub fn new(selective: bool) -> Self {
        Self {
            force_reply: true,
            selective,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ReplyKeboardRemove {
    pub remove_keyboard: bool,
    pub selective: bool,
}

impl Default for ReplyKeboardRemove {
    fn default() -> Self {
        Self {
            remove_keyboard: true,
            selective: false,
        }
    }
}

impl ReplyKeboardRemove {
    pub fn new(selective: bool) -> Self {
        Self {
            remove_keyboard: true,
            selective,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: bool,
    pub request_location: bool,
}

#[derive(Debug, Serialize)]
pub enum InlineKeyboardButtonMessage {
    #[serde(rename = "url")]
    Url(String),
    #[serde(rename = "callback_data")]
    CallbackData(String),
    #[serde(rename = "switch_inline_query")]
    SwitchInlineQuery(String),
    #[serde(rename = "switch_inline_query_current_chat")]
    SwitchInlineQueryCurrentChat(String),
    #[serde(rename = "pay")]
    Pay(bool),
}

#[derive(Debug, Serialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(flatten)]
    pub inline_message: InlineKeyboardButtonMessage,
}

#[derive(Debug, Default, Serialize)]
pub struct ReplyKeboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: bool,
    pub one_time_keyboard: bool,
    pub selective: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    ForceReply(ForceReply),
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeboardMarkup(ReplyKeboardMarkup),
    ReplyKeboardRemove(ReplyKeboardRemove),
}

impl From<ForceReply> for ReplyMarkup {
    fn from(force_reply: ForceReply) -> Self {
        ReplyMarkup::ForceReply(force_reply)
    }
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(inline_markup: InlineKeyboardMarkup) -> Self {
        ReplyMarkup::InlineKeyboardMarkup(inline_markup)
    }
}

impl From<ReplyKeboardMarkup> for ReplyMarkup {
    fn from(reply_markup: ReplyKeboardMarkup) -> Self {
        ReplyMarkup::ReplyKeboardMarkup(reply_markup)
    }
}

impl From<ReplyKeboardRemove> for ReplyMarkup {
    fn from(markup_remove: ReplyKeboardRemove) -> Self {
        ReplyMarkup::ReplyKeboardRemove(markup_remove)
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SendMessage {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub text: String,
    pub parse_mode: Option<String>,
    pub disable_web_page_preview: bool,
    pub disable_notification: bool,
    pub reply_to_message_id: Option<i64>,
    #[serde(flatten)]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    pub fn new(chat_id: ChatID, text: String) -> Self {
        Self {
            chat_id,
            text,
            ..Default::default()
        }
    }

    pub fn with_force_reply(mut self, force_reply: ForceReply) -> Self {
        self.reply_markup = Some(force_reply.into());

        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct PinMessage {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub message_id: i64,
    pub disable_notification: bool,
}

impl PinMessage {
    pub fn new(chat_id: ChatID, message_id: i64, disable_notification: bool) -> Self {
        Self {
            chat_id,
            message_id,
            disable_notification,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SendLocation {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub latitude: f64,
    pub longitude: f64,
    pub live_period: Option<u32>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    #[serde(flatten)]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendLocation {
    pub fn new(chat_id: ChatID, latitude: f64, longitude: f64) -> Self {
        SendLocation {
            chat_id,
            latitude,
            longitude,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn with_live_period(mut self, live_period: u32) -> Self {
        if live_period > 86400 {
            self.live_period = Some(86400);
        } else if live_period < 60 {
            self.live_period = Some(60);
        } else {
            self.live_period = Some(live_period);
        }

        self
    }
}

#[derive(Debug, Serialize)]
pub struct GetFile {
    pub file_id: String,
}

impl GetFile {
    pub fn new(file_id: String) -> Self {
        GetFile { file_id }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct SendContact {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub vcard: Option<String>,
    pub disable_notification: bool,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendContact {
    pub fn new(chat_id: ChatID, phone_number: String, first_name: String) -> Self {
        SendContact {
            chat_id,
            phone_number,
            first_name,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct GetChatMember {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub user_id: i64,
}

#[derive(Default, Debug, Serialize)]
pub struct SetChatStickerSet {
    #[serde(flatten)]
    pub chat_id: ChatID,
    pub sticker_set_name: String,
}
