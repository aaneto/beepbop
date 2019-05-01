use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u64,
}
