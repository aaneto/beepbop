use serde_derive::Serialize;

use crate::input::InlineKeyboardButton;

#[derive(Clone, Debug, Default, Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
