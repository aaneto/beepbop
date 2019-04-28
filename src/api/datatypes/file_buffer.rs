use std::error::Error;
use std::path::Path;

use futures::Future;
use tokio::io::AsyncWrite;

use crate::api::error::APIError;

/// A FileBuffer is a struct
/// representing a file returned
/// from telegram. It has a filename
/// and a byte stream.
///
/// There are no guarantees about meta data
/// from telegram, so name can be ignored when
/// saving the file locally.
#[derive(Debug)]
pub struct FileBuffer {
    pub data: Vec<u8>,
    pub name: String,
}

impl FileBuffer {
    /// Create a new file buffer from a string and data.
    pub fn new(name: String, data: Vec<u8>) -> Self {
        FileBuffer { name, data }
    }

    /// Save the file buffer to a file path.
    pub fn save_as<P>(self, path: P) -> impl Future<Item = (), Error = APIError>
    where
        P: AsRef<Path> + Send + 'static,
    {
        tokio::fs::File::create(path)
            .and_then(move |mut file| file.poll_write(&self.data))
            .map(|_| ())
            .map_err(|err| APIError::DownloadError(err.description().to_owned()))
    }
}
