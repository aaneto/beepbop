use serde_derive::Serialize;

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