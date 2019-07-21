use serde_derive::Deserialize;

use crate::object::UpdateKind;

#[derive(Clone, Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(flatten)]
    pub data: UpdateKind,
}
