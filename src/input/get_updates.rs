use optional_builder::optional_builder;
use serde_derive::Serialize;

#[optional_builder]
#[derive(Default, Debug, Serialize)]
pub struct GetUpdates {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub timeout: Option<i64>,
    pub allowed_updates: Vec<String>,
}
