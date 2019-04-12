pub mod macros;
pub mod api;

pub mod prelude {
    pub use reqwest;
    pub use tokio;
    pub use futures;
    pub use futures::Future;
    pub use crate::api::Bot;
    pub use crate::api::args::*;
    pub use crate::api::APIResult;
    pub use crate::api::APIResponse;
    pub use crate::api::datatypes::*;
    pub use crate::api::error::APIError;
}


#[cfg(test)]
mod tests {
    //! General tests for the telegrambot wrapper.
    //! 
    //! Here should be included functionality tests
    //! for bots. That includes running actual API
    //! calls and expecting an return.
    static API_KEY: &str = include_str!("../res/API_KEY");

    use std::error::Error;

    use crate::api::Bot;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_me() {
        let bot = Bot::new(API_KEY);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        match runtime.block_on(bot.get_me()) {
            Ok((_, user)) => (assert!(user.is_bot, "getMe should return a bot.")),
            Err(error) => {
                assert!(false, error.description().to_owned());
            }
        };
    }

}