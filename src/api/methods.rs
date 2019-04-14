use std::collections::HashMap;

use crate::prelude::*;

use serde::de::DeserializeOwned;
use serde::Serialize;

use reqwest::r#async::Response;

enum Method {
    GET,
    POST,
}

struct TelegramRequest<F, B>
where
    F: Serialize + Sized,
    B: Serialize + Sized,
{
    method: Method,
    route: String,
    form: Option<F>,
    body: Option<B>,
    bot: Bot,
}

impl<F, B> TelegramRequest<F, B>
where
    F: Serialize + Sized,
    B: Serialize + Sized,
{
    fn execute<O>(self) -> impl Future<Item = (Bot, O), Error = APIError>
    where
        O: DeserializeOwned + std::fmt::Debug,
    {
        let client = &self.bot.connection.client;

        let mut request = match self.method {
            Method::GET => client.get(&self.route),
            Method::POST => client.post(&self.route),
        };

        if self.form.is_some() {
            request = request.form(&self.form);
        }

        if self.body.is_some() {
            request = request.json(&self.body);
        }

        request
            .send()
            .and_then(|mut response: Response| response.json())
            .map_err(|err| err.into())
            .and_then(|api_response: APIResponse<O>| api_response.as_result())
            .map(|data| (self.bot, data))
    }
}

type Map = HashMap<String, String>;

impl Bot {
    pub fn get_me(self) -> impl Future<Item = (Self, User), Error = APIError> {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"getMe"),
            form: None::<Map>,
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn get_updates(
        self,
        input_data: GetUpdateArgs,
    ) -> impl Future<Item = (Self, Vec<Update>), Error = APIError> {
        TelegramRequest {
            method: Method::POST,
            route: self.get_route(&"getUpdates"),
            form: None::<Map>,
            body: Some(input_data),
            bot: self,
        }
        .execute()
    }

    pub fn send_message(
        self,
        input_data: SendMessage,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"sendMessage"),
            form: Some(input_data),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn get_chat<ID>(self, id: ID) -> impl Future<Item = (Self, Chat), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        let get_chat = GetChat::new(id.into());

        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"getChat"),
            form: Some(get_chat),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn set_chat_title<ID>(
        self,
        id: ID,
        title: String,
    ) -> impl Future<Item = (Self, bool), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        let set_chat_title = SetChatTitle::new(id.into(), title);

        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"setChatTitle"),
            form: Some(set_chat_title),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn set_chat_description<ID>(
        self,
        id: ID,
        description: String,
    ) -> impl Future<Item = (Self, bool), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        let set_chat_description = SetChatDescription::new(id.into(), description);

        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"setChatDescription"),
            form: Some(set_chat_description),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn pin_message<ID>(
        self,
        id: ID,
        message_id: i64,
        disable_notification: bool,
    ) -> impl Future<Item = (Self, bool), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        let pin_message = PinMessage::new(id.into(), message_id, disable_notification);

        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"pinChatMessage"),
            form: Some(pin_message),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn unpin_message<ID>(self, id: ID) -> impl Future<Item = (Self, bool), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"unpinChatMessage"),
            form: Some(id.into()),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn leave_chat<ID>(self, id: ID) -> impl Future<Item = (Self, bool), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"leaveChat"),
            form: Some(id.into()),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn get_chat_members_count<ID>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, u64), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"getChatMembersCount"),
            form: Some(id.into()),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn send_location(
        self,
        send_location: SendLocation,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"sendLocation"),
            form: Some(send_location),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn get_file(
        self,
        file_id: String,
    ) -> impl Future<Item = (Self, FileInfo), Error = APIError> {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"getFile"),
            form: Some(GetFile::new(file_id)),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn send_contact(
        self,
        send_contact: SendContact,
    ) -> impl Future<Item = (Self, Message), Error = APIError> {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"sendContact"),
            form: Some(send_contact),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn get_chat_admins<ID>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, Vec<ChatMember>), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"getChatAdministrators"),
            form: Some(chat_id.into()),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn get_chat_member<ID>(
        self,
        chat_id: ID,
        user_id: i64,
    ) -> impl Future<Item = (Self, ChatMember), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        let get_chat_member = GetChatMember {
            user_id,
            chat_id: chat_id.into(),
        };

        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"getChatMember"),
            form: Some(get_chat_member),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    /// Set sticker set for a group or chat.
    ///
    /// Will only work for groups of at least
    /// 100 members.
    pub fn set_chat_sticker_set<ID>(
        self,
        chat_id: ID,
        sticker_set: &str,
    ) -> impl Future<Item = (Self, bool), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        let set_chat_sticker_set = SetChatStickerSet {
            sticker_set_name: sticker_set.to_string(),
            chat_id: chat_id.into(),
        };

        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"setChatStickerSet"),
            form: Some(set_chat_sticker_set),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    /// Delete sticker set for a group or chat.
    ///
    /// Will only work if there is a sticker set defined.
    pub fn delete_chat_sticker_set<ID>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"deleteChatStickerSet"),
            form: Some(chat_id.into()),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    /// Promote a chat member.
    ///
    /// Note that the promotions are of type Option<bool>
    /// because:
    ///
    /// None => Don't modify this permission
    /// Some(false) => Deny this permission
    /// Some(true) => Grant this permission
    ///
    /// 1. Admins cannot demote/promote the creator of the group.
    /// 2. Admins cannot demote/promote other admins.
    pub fn promote_chat_member(
        self,
        promote_member: PromoteChatMember,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"promoteChatMember"),
            form: Some(promote_member),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn restrict_chat_member(
        self,
        restrict_member: RestrictChatMember,
    ) -> impl Future<Item = (Self, bool), Error = APIError> {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"restrictChatMember"),
            form: Some(restrict_member),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }

    pub fn delete_chat_photo<ID>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = APIError>
    where
        ID: Into<ChatID>,
    {
        TelegramRequest {
            method: Method::GET,
            route: self.get_route(&"deleteChatPhoto"),
            form: Some(chat_id.into()),
            body: None::<Map>,
            bot: self,
        }
        .execute()
    }
}
