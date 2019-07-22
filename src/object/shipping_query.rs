use serde_derive::Deserialize;

use crate::object::{ShippingAddress, User};

#[derive(Clone, Debug, Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: Box<ShippingAddress>,
}
