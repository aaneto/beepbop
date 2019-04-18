use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct GetUpdates {
    pub offset: i64,
    pub limit: i64,
    pub timeout: i64,
    pub allowed_updates: Vec<String>,
}

impl Default for GetUpdates {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 100,
            timeout: 0,
            allowed_updates: Vec::new(),
        }
    }
}
