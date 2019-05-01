use beepbop::Bot;
use beepbop::tokio;
use beepbop::futures::Future;
use beepbop::log_all;

fn main() {
    let bot = Bot::new("692340343:AAGYYLD1sJt8ELxLd5jR9zI5X6vivNdvDAY");

    tokio::run(
        log_all!(bot.get_updates(Default::default()))
    )
}