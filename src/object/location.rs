use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}
