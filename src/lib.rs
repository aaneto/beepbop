#![forbid(unsafe_code)]

pub mod api;
pub mod macros;
pub mod util;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::api::args::*;
    pub use crate::api::datatypes::*;
    pub use crate::api::error::APIError;
    pub use crate::api::uploaders::*;
    pub use crate::api::APIResponse;
    pub use crate::api::APIResult;
    pub use crate::api::Bot;
    pub use crate::util::*;
    pub use futures;
    pub use futures::Future;
    pub use reqwest;
    pub use tokio;
}
