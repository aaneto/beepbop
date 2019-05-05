use futures::Future;

use crate::error::BotError;
use crate::input::MediaGroup;
use crate::object::Message;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
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
}

#[cfg(test)]
mod tests {
    use crate::input::file;
    use crate::input::MediaGroup;
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

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

}