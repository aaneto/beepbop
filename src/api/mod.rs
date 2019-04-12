pub mod error;
pub mod routes;
pub mod datatypes;
pub mod args;

use std::sync::Arc;

use futures::Future;  
use futures::stream::Stream;

use reqwest::r#async::Client;
use reqwest::r#async::Response;
use reqwest::r#async::Chunk;

use serde_derive::{Deserialize, Serialize};

use error::APIError;
use datatypes::FileBuffer;

pub type APIResult<T> = Result<T, error::APIError>;

struct Connection {
    pub client: Client,
    pub api_key: String,
}

#[derive(Clone)]
pub struct Bot {
    connection: Arc<Connection>,
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

    pub fn download_file(self, file: datatypes::FileInfo) -> impl Future<Item = (Self, FileBuffer), Error = APIError> {
        let file_path = file.file_path.expect("API download file without file_path");
        
        let uri = self.get_file_uri(&file_path);
        
        self.connection.client
            .get(&uri)
            .send()
            .and_then(|response: Response| {
                response.into_body().concat2()
            })
            .map(move |chunks: Chunk| {
                let file_buffer = datatypes::FileBuffer::new(file_path, chunks.to_vec());

                (self, file_buffer)
            })
            .map_err(|err| err.into())
    }

    #[inline]
    fn compose_url(&self, mut base: String, extra: &str) -> String {
        base.extend(self.connection.api_key.chars());
        base.push('/');
        base.extend(extra.chars());

        base
    }

    #[inline]
    fn get_route(&self, route: &str) -> String {
        let url = "https://api.telegram.org/bot".to_string();
        
        self.compose_url(url, route)
    }

    #[inline]
    fn get_file_uri(&self, path: &str) -> String {
        let url = "https://api.telegram.org/file/bot".to_string();

        self.compose_url(url, path)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T>
where T: std::fmt::Debug
{
    ok: bool,
    description: Option<String>,
    error_code: Option<i64>,
    result: Option<T>,
}

impl<T> APIResponse<T>
where T: std::fmt::Debug
{
    pub fn as_result(self) -> APIResult<T> {
        if self.ok {
            Ok(self.result.expect("Ok response without data."))
        } else {
            let mut error_message = String::new();
            error_message.push_str(&"Telegram is not ok. \n");

            self.error_code.as_ref().map(|error_code| {
                error_message.push_str(&error_code.to_string());
                error_message.push_str(&": ");
            });

            self.description.as_ref().map(|description| {
                error_message.push_str(description);
            });

            Err(APIError::TelegramError(error_message))
        }
    }
}