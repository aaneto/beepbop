use serde_derive::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum ChatID {
    Integer(i64),
    String(String),
}

impl Default for ChatID {
    fn default() -> Self {
        ChatID::Integer(0i64)
    }
}

impl From<i64> for ChatID {
    fn from(id: i64) -> Self {
        ChatID::Integer(id)
    }
}

impl From<i32> for ChatID {
    fn from(id: i32) -> Self {
        ChatID::Integer(i64::from(id))
    }
}

impl From<u32> for ChatID {
    fn from(id: u32) -> Self {
        ChatID::Integer(i64::from(id))
    }
}

impl From<String> for ChatID {
    fn from(id: String) -> Self {
        ChatID::String(id)
    }
}

impl ToString for ChatID {
    fn to_string(&self) -> String {
        match self {
            ChatID::Integer(int) => int.to_string(),
            ChatID::String(string) => string.to_string(),
        }
    }
}
