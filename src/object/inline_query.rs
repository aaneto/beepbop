use serde_derive::Deserialize;

use crate::object::{Location, User};

#[derive(Debug, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: Box<User>,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String,
}
