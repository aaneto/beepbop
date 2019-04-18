use serde_derive::Deserialize;

use crate::api::datatypes::LabeledPrice;

#[derive(Debug, Deserialize)]
pub struct OrderInfo {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}
