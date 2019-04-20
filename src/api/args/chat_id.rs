use serde_derive::Serialize;

#[derive(Clone, Debug, Serialize)]
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
