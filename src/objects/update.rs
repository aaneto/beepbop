use serde_derive::Deserialize;

use crate::objects::UpdateKind;

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(flatten)]
    pub data: UpdateKind,
}
