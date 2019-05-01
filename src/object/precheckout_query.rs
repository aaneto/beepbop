use serde_derive::Deserialize;

use crate::object::{OrderInfo, User};

#[derive(Debug, Deserialize)]
pub struct PrecheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: u64,
    pub invoice_payload: String,
    pub shipping_option_id: String,
    pub order_info: Box<OrderInfo>,
}
