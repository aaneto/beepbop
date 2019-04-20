use serde_derive::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: bool,
    pub request_location: bool,
}
