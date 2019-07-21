use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: u64,
}
