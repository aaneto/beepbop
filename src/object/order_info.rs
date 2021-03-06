use serde_derive::Deserialize;

use crate::object::LabeledPrice;

#[derive(Clone, Debug, Deserialize)]
pub struct OrderInfo {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}
