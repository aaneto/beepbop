use serde_derive::Deserialize;

use crate::api::datatypes::CallbackQuery;
use crate::api::datatypes::ChosenInlineResult;
use crate::api::datatypes::InlineQuery;
use crate::api::datatypes::Message;
use crate::api::datatypes::PrecheckoutQuery;
use crate::api::datatypes::ShippingQuery;

#[derive(Debug, Deserialize)]
pub enum UpdateKind {
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
