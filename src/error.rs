use std::error::Error;

#[derive(Debug)]
pub enum BotError {
    TelegramError(String),
    RequestError(reqwest::Error),
    DownloadError(String),
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
        }
    }
}

impl Error for BotError {
    fn description(&self) -> &str {
        match self {
            BotError::TelegramError(err) => err,
            BotError::RequestError(err) => err.description(),
            BotError::DownloadError(err) => err,
        }
    }
}
