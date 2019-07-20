use std::error::Error;

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
