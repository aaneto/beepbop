use std::error::Error;

#[derive(Debug)]
pub enum APIError {
    TelegramError(String),
    RequestError(reqwest::Error),
    DownloadError(String),
}

impl From<reqwest::Error> for APIError {
    fn from(error: reqwest::Error) -> Self {
        APIError::RequestError(error)
    }
}

impl Into<()> for APIError {
    fn into(self) -> () {
        ()
    }
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            APIError::TelegramError(err) => err.fmt(f),
            APIError::RequestError(err) => err.fmt(f),
            APIError::DownloadError(err) => err.fmt(f),
        }
    }
}

impl Error for APIError {
    fn description(&self) -> &str {
        match self {
            APIError::TelegramError(err) => err,
            APIError::RequestError(err) => err.description(),
            APIError::DownloadError(err) => err,
        }
    }
}
