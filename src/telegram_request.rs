#![deny(missing_docs)]
//! The TelegramRequest module is responsible for
//! wrapping telegram responses into concrete objects and
//! constructing requests to query Telegram.

use serde::de::DeserializeOwned;
use serde::Serialize;

use futures::Future;

use reqwest::r#async::multipart::Form;
use reqwest::r#async::multipart::Part;
use reqwest::r#async::RequestBuilder;
use reqwest::r#async::Response;

use crate::bot::{Bot, BotResult};
use crate::error::BotError;
use crate::input::Uploader;

/// An HTTP method abstraction enum
pub(crate) enum Method {
    /// GET HTTP method variant
    GET,
    /// POST HTTP method variant
    POST,
}

/// A struct encoding a telegram request created by a particular bot.
pub(crate) struct TelegramRequest {
    builder: RequestBuilder,
    form: Option<Form>,
    bot: Bot,
}

impl TelegramRequest {
    /// Create a new telegram request with a method and a URI.
    pub(crate) fn new(method: Method, route: String, bot: Bot) -> Self {
        let client = &bot.connection.client;

        let request = match method {
            Method::GET => client.get(&route),
            Method::POST => client.post(&route),
        };

        TelegramRequest {
            builder: request,
            form: None,
            bot,
        }
    }

    /// Append a json body to the request
    pub(crate) fn with_body<B: Serialize + Sized>(mut self, body_data: B) -> Self {
        self.builder = self.builder.json(&body_data);

        self
    }

    /// Append an optional form into this request
    fn with_optional_form(mut self) -> Self {
        if let Some(form) = self.form.take() {
            self.builder = self.builder.multipart(form);
        }

        self
    }

    /// Append a query to the request
    pub(crate) fn with_query<Q: Serialize + Sized>(mut self, query_data: Q) -> Self {
        self.builder = self.builder.query(&query_data);

        self
    }

    /// Append a form part to the request
    pub(crate) fn with_form_part(mut self, tag: &str, part: Part) -> Self {
        if let Some(form) = self.form.take() {
            self.form = Some(form.part(tag.to_owned(), part));
        } else {
            let form = Form::new();

            self.form = Some(form.part(tag.to_owned(), part));
        }

        self
    }

    /// Append a textual form to the request
    pub(crate) fn with_form_text<S: ToString>(mut self, tag: S, text: S) -> Self {
        if let Some(form) = self.form.take() {
            self.form = Some(form.text(tag.to_string(), text.to_string()));
        } else {
            let form = Form::new();

            self.form = Some(form.text(tag.to_string(), text.to_string()));
        }

        self
    }

    /// Inject an Uploader object into this request
    pub(crate) fn with_uploader(self, tag: &str, uploader: Uploader) -> Self {
        uploader.upload_into(tag, self)
    }

    /// Execute this request returning a Future
    pub(crate) fn execute<O: DeserializeOwned + std::fmt::Debug>(
        mut self,
    ) -> impl Future<Item = (Bot, O), Error = BotError> {
        self = self.with_optional_form();
        let bot = self.bot;

        self.builder
            .send()
            .and_then(|mut response: Response| response.json())
            .map_err(BotError::from)
            .and_then(|api_response: TelegramResponse<O>| {
                let api_result: BotResult<O> = api_response.into();

                api_result
            })
            .map(move |data: O| (bot, data))
    }
}

/// A typed container for a default Telegram request
#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub(crate) struct TelegramResponse<T>
where
    T: std::fmt::Debug,
{
    ok: bool,
    description: Option<String>,
    error_code: Option<i64>,
    result: Option<T>,
}

impl<T: std::fmt::Debug> Into<BotResult<T>> for TelegramResponse<T> {
    fn into(self) -> BotResult<T> {
        if self.ok {
            Ok(self.result.expect("Ok response must have data."))
        } else {
            let mut error_message = String::new();
            error_message.push_str(&"Telegram(");

            if let Some(error_code) = self.error_code.as_ref() {
                error_message.push_str(&error_code.to_string());
                error_message.push_str(&"): ");
            }

            if let Some(description) = self.description.as_ref() {
                error_message.push_str(description);
            }

            Err(BotError::TelegramError(error_message))
        }
    }
}
