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
//! use std::env::var;
//! use beepbop::bot::Bot;
//! use beepbop::object::User;
//! use beepbop::tokio;
//! use beepbop::futures::Future;
//!
//! let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
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
//! use std::env::var;
//! use beepbop::bot::Bot;
//! use beepbop::tokio;
//! use beepbop::futures::Future;
//!
//! let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
//! let chat_id = var("CHAT_ID").expect("Cannot find CHAT_ID in ENV");
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

/// The prelude module contains a few commonly used modules and objects.
pub mod prelude {
    pub use crate::bot::Bot;
    pub use crate::error::BotError;
    pub use crate::futures;
    pub use crate::futures::Future;
    pub use crate::input::*;
    pub use crate::object::*;
    pub use crate::reqwest;
    pub use crate::tokio;
}

pub mod bot;
pub mod error;
pub mod input;
pub mod object;
pub mod stream;
pub mod telegram_request;

mod macros;

#[cfg(test)]
pub mod tests;

pub use futures;
pub use reqwest;
pub use tokio;
