use serde_derive::Deserialize;

use crate::api::datatypes::ShippingAddress;
use crate::api::datatypes::User;

#[derive(Debug, Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}
