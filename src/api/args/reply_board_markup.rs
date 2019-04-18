use serde_derive::Serialize;

use crate::api::args::KeyboardButton;

#[derive(Debug, Default, Serialize)]
pub struct ReplyKeboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: bool,
    pub one_time_keyboard: bool,
    pub selective: bool,
}
