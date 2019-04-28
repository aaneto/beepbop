#![forbid(unsafe_code)]
//! # Overview
//!
//! The telegrambot crate is meant to be an unopinionated telegram
//! Bot API wrapper. Providing common functionality involving
//! telegram and futures to be used with telegram, such as file
//! uploaders and the like.
//!
//! ## Actions
//!
//! Every action is supposed to be a function performed by the bot,
//! returning a future with a reference to the bot and the data
//! of the last action.
//!
//! ```rust
//! use telegrambot::prelude::*;
//!
//! let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
//!
//! let bot = Bot::new(&api_key);
//!
//! tokio::run(
//!     bot
//!         .get_me()
//!         .map(|(_bot, me): (Bot, User)| println!("{:?}", me))
//!         .map_err(|err| println!("{:?}", err))
//! );
//! ```
//!
//! ## Multiple Actions
//!
//! Actions can be chained with the use of the and_then combinator.
//!
//!
//! ```rust
//! use telegrambot::prelude::*;
//!
//! let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
//! let chat_id = get_argv("CHAT_ID").expect("Cannot find CHAT_ID in ENV");
//!
//! let bot = Bot::new(&api_key);
//!
//! tokio::run(
//!     bot
//!         .get_chat(chat_id)
//!         .and_then(|(bot, chat)| {
//!             let file_id = chat.photo.unwrap().big_file_id;
//!
//!             bot.download_file(file_id)
//!         })
//!         .and_then(|(_, file_buffer)| {
//!             let save_name = file_buffer.name.replace("/", "_");
//!
//!             file_buffer.save_as(format!("res/{}", save_name))
//!         })
//!         .map_err(|err| println!("{:?}", err))
//! );
//!
//! ```

pub mod api;
mod macros;
pub mod util;

/// Common types and functions used to build
/// telegram bots.
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
