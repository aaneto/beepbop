use serde_derive::Serialize;

use crate::api::args::InlineKeyboardButtonMessage;

#[derive(Clone, Debug, Serialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(flatten)]
    pub inline_message: InlineKeyboardButtonMessage,
}
