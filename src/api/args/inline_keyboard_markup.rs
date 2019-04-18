use serde_derive::Serialize;

use crate::api::args::InlineKeyboardButton;

#[derive(Debug, Default, Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}