#![deny(missing_docs)]
//! The Error module define telegram bot related errors

use crate::object::Message;
use std::error::Error;
use std::sync::mpsc;

/// Enum representing all variations of errors an bot
/// can return in a Future or otherwise.
#[derive(Debug)]
pub enum BotError {
    /// An BotError related to Telegram related incorrect interactions
    TelegramError(String),
    /// An BotError related to HTTP requests using reqwest
    RequestError(reqwest::Error),
    /// An BotError related to File Downloading on Telegram
    DownloadError(String),
    /// An BotError related to invalid media group formattion
    InvalidMediaGroup(String),
}

impl From<reqwest::Error> for BotError {
    fn from(error: reqwest::Error) -> Self {
        BotError::RequestError(error)
    }
}

impl Into<()> for BotError {
    fn into(self) {}
}

impl std::fmt::Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            BotError::TelegramError(err) => err.fmt(f),
            BotError::RequestError(err) => err.fmt(f),
            BotError::DownloadError(err) => err.fmt(f),
            BotError::InvalidMediaGroup(err) => err.fmt(f),
        }
    }
}

impl Error for BotError {
    fn description(&self) -> &str {
        match self {
            BotError::TelegramError(err) => err,
            BotError::RequestError(err) => err.description(),
            BotError::DownloadError(err) => err,
            BotError::InvalidMediaGroup(err) => err,
        }
    }
}

type SendMessageError = mpsc::SendError<Message>;

/// The StreamError is an error originated from a stream
/// of updates.
#[derive(Debug)]
pub enum StreamError {
    /// An error occurred while trying to fetch the updates
    BotError(BotError),
    /// An error occurred while trying to send an message to a channel
    SendMessageError(SendMessageError),
}

impl From<BotError> for StreamError {
    fn from(bot_error: BotError) -> StreamError {
        StreamError::BotError(bot_error)
    }
}

impl From<SendMessageError> for StreamError {
    fn from(send_error: SendMessageError) -> StreamError {
        StreamError::SendMessageError(send_error)
    }
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            StreamError::BotError(err) => err.fmt(f),
            StreamError::SendMessageError(err) => err.fmt(f),
        }
    }
}

impl Error for StreamError {
    fn description(&self) -> &str {
        match self {
            StreamError::BotError(err) => err.description(),
            StreamError::SendMessageError(err) => err.description(),
        }
    }
}
