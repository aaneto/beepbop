use serde_derive::Serialize;

#[derive(Clone, Debug, Serialize)]
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
