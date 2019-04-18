use serde_derive::Deserialize;

use crate::api::datatypes::Location;
use crate::api::datatypes::User;

#[derive(Debug, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}
