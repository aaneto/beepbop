use futures::Future;

use crate::error::BotError;
use crate::input::GetUserProfilePhotos;
use crate::object::UserProfilePhotos;
use crate::telegram_request::{Method, TelegramRequest};
use crate::Bot;

impl Bot {
    pub fn get_user_profile_photos(
        self,
        get_user_profile_photos: GetUserProfilePhotos,
    ) -> impl Future<Item = (Self, UserProfilePhotos), Error = BotError> {
        TelegramRequest::new(Method::POST, self.get_route(&"getUserProfilePhotos"), self)
            .with_body(get_user_profile_photos)
            .execute()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::GetUserProfilePhotos;
    use crate::Bot;
    use std::env::var;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_user_profile_photos() {
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

}
