use serde_derive::Serialize;

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
