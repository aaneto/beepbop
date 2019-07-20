#![deny(missing_docs)]
//! The Bot module defines the Bot object. The Bot
//! object has all Telegram Actions available to him,
//! as long as you have the required arguments.

use std::sync::Arc;

use futures::stream::Stream;

use reqwest::r#async::Chunk;
use reqwest::r#async::Client;
use reqwest::r#async::Response;

use crate::error::BotError;
use crate::object::FileBuffer;

use crate::input::*;
use crate::object::*;
use crate::telegram_request::{Method, TelegramRequest};

use futures::Future;

/// A wrapper for Bot related Results
pub type BotResult<T> = Result<T, BotError>;

/// The Connection struct holds data required for
/// the Bot to communicate with the Telegram API.
pub(crate) struct Connection {
    pub(crate) client: Client,
    api_key: String,
}

/// The Bot is a ARC over a connection, so piping is possible
/// between Futures
#[derive(Clone)]
pub struct Bot {
    pub(crate) connection: Arc<Connection>,
}

impl std::fmt::Debug for Bot {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Bot:{}", self.connection.api_key)
    }
}

impl Bot {
    /// Create a new Telegram Bot using an API_KEY
    pub fn new(api_key: &str) -> Self {
        let connection = Connection {
            api_key: api_key.to_string(),
            client: Client::new(),
        };

        Bot {
            connection: Arc::new(connection),
        }
    }

    /// Download a file at telegram using it's ID
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

    /// Construct an URI using the telegram domain
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

    /// Send a video note to a telegram chat
    pub fn send_video_note(
        self,
        send_video_note: SendVideoNote,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_video_note.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendVideoNote"), self)
            .with_query(query)
            .with_uploader("video_note", uploader)
            .execute()
    }

    /// Send a chat action, that is a special action that a bot can take for
    /// some time, like displaying 'sending photo' for a few seconds.
    pub fn send_chat_action<ID: Into<ChatID>>(
        self,
        id: ID,
        action: Action,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let arg = SendChatAction::new(id.into(), action);

        TelegramRequest::new(Method::GET, self.get_route(&"sendChatAction"), self)
            .with_query(arg)
            .execute()
    }

    /// Send a document to a telegram chat
    pub fn send_document(
        self,
        send_document: SendDocument,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_document.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendDocument"), self)
            .with_query(query)
            .with_uploader("document", uploader)
            .execute()
    }

    /// Forward a message to a telegram chat by id
    pub fn forward_message<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        from_chat_id: ID,
        message_id: i64,
        disable_notification: bool,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let args = ForwardMessage {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification,
        };

        TelegramRequest::new(Method::GET, self.get_route(&"forwardMessage"), self)
            .with_query(args)
            .execute()
    }

    /// Send a new message to a telegram chat
    pub fn send_message(
        self,
        send_message: SendMessage,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendMessage"), self)
            .with_query(send_message)
            .execute()
    }

    /// Leave a chat by id
    pub fn leave_chat<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"leaveChat"), self)
            .with_query(id.into())
            .execute()
    }

    /// Unban a chat member of a telegram chat
    pub fn unban_chat_member<ID: Into<ChatID>>(
        self,
        id: ID,
        user_id: i64,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let arg = UnbanChatMember {
            chat_id: id.into(),
            user_id,
        };

        TelegramRequest::new(Method::GET, self.get_route(&"unbanChatMember"), self)
            .with_query(arg)
            .execute()
    }

    /// Send a contact to a telegram chat
    pub fn send_contact(
        self,
        send_contact: SendContact,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendContact"), self)
            .with_query(send_contact)
            .execute()
    }

    /// Get the count of members on a telegram chat
    pub fn get_chat_members_count<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, u64), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getChatMembersCount"), self)
            .with_query(id.into())
            .execute()
    }

    /// Send an audio to a telegram chat
    pub fn send_audio(
        self,
        send_audio: SendAudio,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, voice) = send_audio.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendAudio"), self)
            .with_query(query)
            .with_uploader("audio", voice)
            .execute()
    }

    /// Send a media group to a telegram chat, that is, a group of photos and videos
    pub fn send_media_group(
        self,
        media_group: MediaGroup,
    ) -> impl Future<Item = (Self, Vec<Message>), Error = BotError> {
        if !media_group.attachments.is_empty() {
            let mut req =
                TelegramRequest::new(Method::POST, self.get_route(&"sendMediaGroup"), self)
                    .with_query(media_group.query)
                    // Media is encoded as a string with a json inside
                    .with_form_text("media", &media_group.media_encoded);

            for attachment in media_group.attachments {
                req = req.with_uploader(&attachment.name, attachment.uploader);
            }

            req.execute()
        } else {
            TelegramRequest::new(Method::POST, self.get_route(&"sendMediaGroup"), self)
                .with_body(media_group.query)
                .execute()
        }
    }

    /// Get information about a chat by chat id
    pub fn get_chat<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, Chat), Error = BotError> {
        let get_chat = GetChat::new(id.into());

        TelegramRequest::new(Method::GET, self.get_route(&"getChat"), self)
            .with_query(get_chat)
            .execute()
    }

    /// Kick a member of a telegram chat. Require kicking privileges
    pub fn kick_chat_member<ID: Into<ChatID>>(
        self,
        id: ID,
        user_id: i64,
        until_date: Option<u64>,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let arg = KickChatMember {
            chat_id: id.into(),
            user_id,
            until_date,
        };

        TelegramRequest::new(Method::GET, self.get_route(&"kickChatMember"), self)
            .with_query(arg)
            .execute()
    }

    /// Get updates for your bot, those updates can be of many kinds, check the Update struct
    /// for more information
    pub fn get_updates(
        self,
        get_updates: GetUpdates,
    ) -> impl Future<Item = (Self, Vec<Update>), Error = BotError> {
        TelegramRequest::new(Method::POST, self.get_route(&"getUpdates"), self)
            .with_body(get_updates)
            .execute()
    }

    /// Send a location on a telegram chat
    pub fn send_location(
        self,
        send_location: SendLocation,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendLocation"), self)
            .with_query(send_location)
            .execute()
    }

    /// Send a venue on a telegram chat
    pub fn send_venue(
        self,
        send_venue: SendVenue,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"sendVenue"), self)
            .with_query(send_venue)
            .execute()
    }

    /// Get the profile photos of an user
    pub fn get_user_profile_photos(
        self,
        get_user_profile_photos: GetUserProfilePhotos,
    ) -> impl Future<Item = (Self, UserProfilePhotos), Error = BotError> {
        TelegramRequest::new(Method::POST, self.get_route(&"getUserProfilePhotos"), self)
            .with_body(get_user_profile_photos)
            .execute()
    }

    /// Edit an on going live location
    pub fn edit_live_location(
        self,
        edit_live_location: EditLiveLocation,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(
            Method::GET,
            self.get_route(&"editMessageLiveLocation"),
            self,
        )
        .with_query(edit_live_location)
        .execute()
    }

    /// Delete sticker set for a group or chat.
    ///
    /// Will only work if there is a sticker set defined.
    pub fn delete_chat_sticker_set<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"deleteChatStickerSet"), self)
            .with_query(chat_id.into())
            .execute()
    }

    /// Get information of a file on telegram
    pub fn get_file(
        self,
        file_id: String,
    ) -> impl Future<Item = (Self, FileInfo), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getFile"), self)
            .with_query(GetFile::new(file_id))
            .execute()
    }

    /// Unpin a message on a telegram chat
    pub fn unpin_message<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"unpinChatMessage"), self)
            .with_query(id.into())
            .execute()
    }

    /// Set the textual description of a telegram chat
    pub fn set_chat_description<ID: Into<ChatID>>(
        self,
        id: ID,
        description: String,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let set_chat_description = SetChatDescription::new(id.into(), description);

        TelegramRequest::new(Method::GET, self.get_route(&"setChatDescription"), self)
            .with_query(set_chat_description)
            .execute()
    }

    /// Restrict a chat member by some defined parameters
    pub fn restrict_chat_member(
        self,
        restrict_member: RestrictChatMember,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"restrictChatMember"), self)
            .with_query(restrict_member)
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
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"promoteChatMember"), self)
            .with_query(promote_member)
            .execute()
    }

    /// Get information about a chat member on a telegram chat
    pub fn get_chat_member<ID>(
        self,
        chat_id: ID,
        user_id: i64,
    ) -> impl Future<Item = (Self, ChatMember), Error = BotError>
    where
        ID: Into<ChatID>,
    {
        let get_chat_member = GetChatMember::new(chat_id.into(), user_id);

        TelegramRequest::new(Method::GET, self.get_route(&"getChatMember"), self)
            .with_query(get_chat_member)
            .execute()
    }

    /// Delete the photo of a telegram chat
    pub fn delete_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"deleteChatPhoto"), self)
            .with_query(chat_id.into())
            .execute()
    }

    /// Send a voice to a telegram chat
    pub fn send_voice(
        self,
        send_voice: SendVoice,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, voice) = send_voice.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendVoice"), self)
            .with_query(query)
            .with_uploader("voice", voice)
            .execute()
    }

    /// Get all admins of a telegram chat
    pub fn get_chat_admins<ID: Into<ChatID>>(
        self,
        chat_id: ID,
    ) -> impl Future<Item = (Self, Vec<ChatMember>), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getChatAdministrators"), self)
            .with_query(chat_id.into())
            .execute()
    }

    /// Set sticker set for a group or chat.
    ///
    /// Will only work for groups of at least
    /// 100 members.
    pub fn set_chat_sticker_set<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        sticker_set: &str,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let set_chat_sticker_set = SetChatStickerSet {
            sticker_set_name: sticker_set.to_string(),
            chat_id: chat_id.into(),
        };

        TelegramRequest::new(Method::GET, self.get_route(&"setChatStickerSet"), self)
            .with_query(set_chat_sticker_set)
            .execute()
    }

    /// Send a photo in telegram.
    ///
    /// Photos can be uploaded by Id, Url and Post
    /// methods. Note that chat photo id's are only
    /// usable for downloading a chat photo, not here.
    pub fn send_photo(
        self,
        send_photo: SendPhoto,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_photo.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendPhoto"), self)
            .with_query(query)
            .with_uploader("photo", uploader)
            .execute()
    }

    /// Export an invite link for the chat
    pub fn export_chat_invite_link<ID: Into<ChatID>>(
        self,
        id: ID,
    ) -> impl Future<Item = (Self, String), Error = BotError> {
        let arg = ExportChatInviteLink::new(id.into());

        TelegramRequest::new(Method::GET, self.get_route(&"exportChatInviteLink"), self)
            .with_query(arg)
            .execute()
    }

    /// Get user information for your bot
    pub fn get_me(self) -> impl Future<Item = (Self, User), Error = BotError> {
        TelegramRequest::new(Method::GET, self.get_route(&"getMe"), self).execute()
    }

    /// Send a video on a telegram chat
    pub fn send_video(
        self,
        send_video: SendVideo,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_video.split();

        TelegramRequest::new(Method::POST, self.get_route(&"SendVideo"), self)
            .with_query(query)
            .with_uploader("video", uploader)
            .execute()
    }

    /// Set the title of a particular telegram chat
    pub fn set_chat_title<ID: Into<ChatID>>(
        self,
        id: ID,
        title: String,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let set_chat_title = SetChatTitle::new(id.into(), title);

        TelegramRequest::new(Method::GET, self.get_route(&"setChatTitle"), self)
            .with_query(set_chat_title)
            .execute()
    }

    /// Send an animation in a telegram chat
    pub fn send_animation(
        self,
        send_animation: SendAnimation,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        let (query, uploader) = send_animation.split();

        TelegramRequest::new(Method::POST, self.get_route(&"sendAnimation"), self)
            .with_query(query)
            .with_uploader("animation", uploader)
            .execute()
    }

    /// Stop an on going live location
    pub fn stop_live_location(
        self,
        stop_live_location: StopLiveLocation,
    ) -> impl Future<Item = (Self, Message), Error = BotError> {
        TelegramRequest::new(
            Method::GET,
            self.get_route(&"stopMessageLiveLocation"),
            self,
        )
        .with_query(stop_live_location)
        .execute()
    }

    /// Pin a message on a particular telegram chat
    pub fn pin_message<ID: Into<ChatID>>(
        self,
        id: ID,
        message_id: i64,
        disable_notification: bool,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        let pin_message = PinMessage::new(id.into(), message_id, disable_notification);

        TelegramRequest::new(Method::GET, self.get_route(&"pinChatMessage"), self)
            .with_query(pin_message)
            .execute()
    }

    /// Set the photo of a particular telegram chat
    pub fn set_chat_photo<ID: Into<ChatID>>(
        self,
        chat_id: ID,
        photo: FileUploader,
    ) -> impl Future<Item = (Self, bool), Error = BotError> {
        TelegramRequest::new(Method::POST, self.get_route(&"setChatPhoto"), self)
            .with_query(chat_id.into())
            .with_uploader("photo", photo.into())
            .execute()
    }
}

#[cfg(test)]
mod bot_tests {
    use crate::bot::Bot;
    use crate::error::BotError;
    use crate::input::*;
    use crate::object::message::Message;

    use futures::Future;
    use std::env::var;
    use std::error::Error;
    use tokio::runtime::Runtime;

    #[test]
    fn send_video_note() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let video_note = file("res/video.mp4").unwrap();

        let arg = SendVideoNote::new(chat_id, video_note);

        if let Err(err) = runtime.block_on(bot.send_video_note(arg)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn send_animation() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let animation = file("res/anim.gif").unwrap();

        let arg = SendAnimation::new(chat_id, animation);

        if let Err(err) = runtime.block_on(bot.send_animation(arg)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn send_video() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let puppy_photo = file("res/puppy.jpg").unwrap();

        let video = file("res/video.mp4")
            .and_then(add_mime("video/mp4"))
            .map(add_thumbnail(puppy_photo))
            .unwrap();

        let arg = SendVideo::new(chat_id, video);

        if let Err(err) = runtime.block_on(bot.send_video(arg)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn get_me() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.get_me()) {
            panic!(error.description().to_owned());
        }
    }

    #[test]
    fn send_chat_action() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.send_chat_action(chat_id, Action::RecordAudio)) {
            panic!(error.description().to_owned());
        }
    }

    #[test]
    fn forward_message() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

        let message_id: i64 = var("MESSAGE_ID")
            .expect("Cannot find MESSAGE_ID in ENV")
            .parse()
            .expect("MESSAGE_ID is not an valid ID.");

        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) = runtime.block_on(bot.forward_message(chat_id, chat_id, message_id, false))
        {
            panic!("{:#?}", err);
        }
    }

    #[test]
    #[ignore]
    fn kick_chat_member() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let user_id: i64 = var("USER_ID")
            .expect("Cannot find USER_ID in ENV")
            .parse()
            .expect("USER_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.kick_chat_member(chat_id, user_id, None)) {
            panic!(error.description().to_owned());
        }
    }

    #[test]
    fn document_upload_thumbnail() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let pupper_thumbnail = file("res/puppy.jpg")
            .and_then(add_mime("image/jpg"))
            .unwrap();

        let text_file = file("res/some_text")
            .and_then(add_mime("text/plain"))
            .map(add_thumbnail(pupper_thumbnail))
            .unwrap();

        let arg = SendDocument::new(chat_id, text_file);

        if let Err(err) = runtime.block_on(bot.send_document(arg)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    #[ignore]
    fn unban_chat_member() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let user_id: i64 = var("USER_ID")
            .expect("Cannot find USER_ID in ENV")
            .parse()
            .expect("USER_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(error) = runtime.block_on(bot.unban_chat_member(chat_id, user_id)) {
            panic!(error.description().to_owned());
        }
    }

    #[test]
    fn send_audio() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let voice = file("res/sound.mp3").unwrap();

        let arg = SendAudio::new(chat_id, voice);

        if let Err(err) = runtime.block_on(bot.send_audio(arg)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn get_chat() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) = runtime.block_on(bot.get_chat(chat_id)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn send_media_group() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let pupper_file = file("res/brownpuppy.png").unwrap();
        let pupper_two = file("res/puppy.jpg").unwrap();
        let gif = file("res/anim.gif").unwrap();

        let group = MediaGroup::build(chat_id)
            .with_disable_notification(true)
            .add_photo(pupper_file)
            .add_photo(pupper_two)
            .add_video_with(gif, None, |video| video.with_caption("MyCaption"))
            .finish()
            .unwrap();

        if let Err(err) = runtime.block_on(bot.send_media_group(group)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn send_venue() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let arg =
            SendVenue::new(chat_id, 20.0, 30.5, "Dunno", "Somewhere").with_foursquare_id("sddd");

        if let Err(err) = runtime.block_on(bot.send_venue(arg)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn get_user_profile_photos() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let user_id: i64 = var("USER_ID")
            .expect("Cannot find USER_ID in ENV")
            .parse()
            .expect("USER_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let arg = GetUserProfilePhotos::new(user_id).with_limit(2);

        if let Err(err) = runtime.block_on(bot.get_user_profile_photos(arg)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn send_voice() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        let voice = file("res/voice.ogg").unwrap();

        let arg = SendVoice::new(chat_id, voice);

        if let Err(err) = runtime.block_on(bot.send_voice(arg)) {
            panic!("{:#?}", err);
        }
    }

    fn send_photo_future(
        file_name: &str,
        mime_string: Option<&str>,
    ) -> impl Future<Item = (Bot, Message), Error = BotError> {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut uploader_res = file(file_name);

        if let Some(mime_str) = mime_string {
            uploader_res = uploader_res.and_then(add_mime(mime_str));
        }

        let arg = SendPhoto::new(chat_id, uploader_res.unwrap());
        bot.send_photo(arg)
    }

    #[test]
    fn send_photo() {
        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) =
            runtime.block_on(send_photo_future("res/brownpuppy.png", Some("image/png")))
        {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn send_photo_without_mime() {
        let mut runtime = Runtime::new().expect("Unable to create a runtime");
        if let Err(err) = runtime.block_on(send_photo_future("res/brownpuppy.png", None)) {
            panic!("{:#?}", err);
        }
    }

    #[test]
    fn export_chat_invite_link() {
        let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

        let chat_id: i64 = var("CHAT_ID")
            .expect("Cannot find CHAT_ID in ENV")
            .parse()
            .expect("CHAT_ID is not an valid ID.");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        if let Err(err) = runtime.block_on(bot.export_chat_invite_link(chat_id)) {
            panic!("{:#?}", err);
        }
    }
}
