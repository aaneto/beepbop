use crate::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use reqwest::r#async::multipart::Form;
use reqwest::r#async::RequestBuilder;
use reqwest::r#async::Response;

pub enum Method {
    GET,
    POST,
}

pub struct TelegramRequest {
    builder: RequestBuilder,
    bot: Bot,
}

impl TelegramRequest {
    pub fn new(method: Method, route: String, bot: Bot) -> Self {
        let client = &bot.connection.client;

        let request = match method {
            Method::GET => client.get(&route),
            Method::POST => client.post(&route),
        };

        TelegramRequest {
            builder: request,
            bot,
        }
    }

    pub fn with_body<B: Serialize + Sized>(mut self, body_data: B) -> Self {
        self.builder = self.builder.json(&body_data);

        self
    }

    pub fn with_multipart(mut self, form: Form) -> Self {
        self.builder = self.builder.multipart(form);

        self
    }

    pub fn with_query<Q: Serialize + Sized>(mut self, query_data: Q) -> Self {
        self.builder = self.builder.query(&query_data);

        self
    }

    pub fn with_uploader<U: Uploader>(self, tag: &str, file_uploader: U) -> Self {
        file_uploader.upload_into(tag, self)
    }

    pub fn execute<O: DeserializeOwned + std::fmt::Debug>(
        self,
    ) -> impl Future<Item = (Bot, O), Error = APIError> {
        let bot = self.bot;

        self.builder
            .send()
            .and_then(|mut response: Response| response.json())
            .map_err(APIError::from)
            .and_then(|api_response: APIResponse<O>| {
                let api_result: APIResult<O> = api_response.into();

                api_result
            })
            .map(move |data: O| (bot, data))
    }
}

pub mod delete_chat_photo;
pub mod delete_chat_sticker_set;
pub mod get_chat;
pub mod get_chat_admins;
pub mod get_chat_member;
pub mod get_file;
pub mod get_me;
pub mod get_updates;
pub mod leave_chat;
pub mod pin_message;
pub mod promote_chat_member;
pub mod restrict_chat_member;
pub mod send_contact;
pub mod send_location;
pub mod send_message;
pub mod send_photo;
pub mod set_chat_description;
pub mod set_chat_photo;
pub mod set_chat_sticker_set;
pub mod set_chat_title;
pub mod unpin_message;
