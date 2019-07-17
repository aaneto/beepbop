use optbuilder::OptionalBuilder;
use serde_derive::Serialize;

#[derive(OptionalBuilder, Default, Debug, Serialize)]
pub struct GetUpdates {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub timeout: Option<i64>,
    pub allowed_updates: Vec<String>,
}

impl GetUpdates {
    pub fn new() -> Self {
        GetUpdates::default()
    }
}
