#![forbid(unsafe_code)]

pub mod api;
pub mod macros;

pub mod prelude {
    pub use crate::api::args::*;
    pub use crate::api::datatypes::*;
    pub use crate::api::error::APIError;
    pub use crate::api::APIResponse;
    pub use crate::api::APIResult;
    pub use crate::api::Bot;
    pub use futures;
    pub use futures::Future;
    pub use reqwest;
    pub use tokio;
}

#[cfg(test)]
mod tests {
    //! General tests for the telegrambot wrapper.
    //!
    //! Here should be included functionality tests
    //! for bots. That includes running actual API
    //! calls and expecting an return.
    use crate::api::Bot;
    use std::error::Error;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_me() {
        let (_, api_key) = std::env::vars()
            .find(|(key, _)| key == "API_KEY")
            .expect("Cannot find API_KEY in ENV");

        let bot = Bot::new(&api_key);

        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        match runtime.block_on(bot.get_me()) {
            Ok((_, user)) => {
                assert!(user.is_bot, "getMe should return a bot.")
            },
            Err(error) => {
                panic!(error.description().to_owned());
            }
        };
    }

}
