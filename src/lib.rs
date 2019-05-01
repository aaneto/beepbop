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
//! use beepbop::Bot;
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
//! use beepbop::Bot;
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
pub mod prelude {
    pub use crate::error::BotError;
    pub use crate::futures;
    pub use crate::futures::Future;
    pub use crate::input::*;
    pub use crate::object::*;
    pub use crate::reqwest;
    pub use crate::tokio;
    pub use crate::Bot;
}

pub mod action;
pub mod error;
pub mod input;
pub mod object;
pub mod telegram_request;

mod macros;

#[cfg(test)]
pub mod tests;

pub use futures;
pub use reqwest;
pub use tokio;

use std::sync::Arc;

use futures::stream::Stream;
use futures::Future;

use reqwest::r#async::Chunk;
use reqwest::r#async::Client;
use reqwest::r#async::Response;

use crate::error::BotError;
use crate::object::FileBuffer;

pub type BotResult<T> = Result<T, BotError>;

pub struct Connection {
    pub client: Client,
    pub api_key: String,
}

#[derive(Clone)]
pub struct Bot {
    pub connection: Arc<Connection>,
}

impl std::fmt::Debug for Bot {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Bot:{}", self.connection.api_key)
    }
}

impl Bot {
    pub fn new(api_key: &str) -> Self {
        let connection = Connection {
            api_key: api_key.to_string(),
            client: Client::new(),
        };

        Bot {
            connection: Arc::new(connection),
        }
    }

    pub fn download_file(
        self,
        file_id: String,
    ) -> impl Future<Item = (Self, FileBuffer), Error = BotError> {
        self.get_file(file_id).and_then(move |(bot, file_info)| {
            let file_path = file_info
                .file_path
                .expect("API download file without file_path");

            let uri = bot.get_file_uri(&file_path);

            bot.connection
                .client
                .get(&uri)
                .send()
                .and_then(|response: Response| response.into_body().concat2())
                .map(move |chunks: Chunk| {
                    let file_buffer = FileBuffer::new(file_path, chunks.to_vec());

                    (bot, file_buffer)
                })
                .map_err(std::convert::Into::into)
        })
    }

    #[inline]
    fn compose_url(&self, mut base: String, extra: &str) -> String {
        base.push_str(&self.connection.api_key);
        base.push('/');
        base.push_str(extra);

        base
    }

    #[inline]
    pub fn get_route(&self, route: &str) -> String {
        let url = "https://api.telegram.org/bot".to_string();

        self.compose_url(url, route)
    }

    #[inline]
    fn get_file_uri(&self, path: &str) -> String {
        let url = "https://api.telegram.org/file/bot".to_string();

        self.compose_url(url, path)
    }
}
