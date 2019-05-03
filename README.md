## Beepbop

# Overview

Beepbop is yet another telegram BOT API wrapper, this lib
aims to cover most methods and also provide some streaming
infrastructure based on the get_updates method.

## Actions

Every action is supposed to be a function performed by the bot,
returning a future with a reference to the bot and the data
of the last action.

```rust, no_run
use std::env::var;
use beepbop::prelude::*;

fn main() {
    let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

    let bot = Bot::new(&api_key);

    tokio::run(
        bot
            .get_me()
            .map(|(_bot, me): (Bot, User)| println!("{:?}", me))
            .map_err(|err| println!("{:?}", err))
    );
}
```

## Multiple Actions

Actions can be chained with the use of the and_then combinator, since they are futures.

```rust,no_run
use std::env::var;
use beepbop::prelude::*;

fn main() {

    let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
    let chat_id = var("CHAT_ID").expect("Cannot find CHAT_ID in ENV");

    let bot = Bot::new(&api_key);

    tokio::run(
        bot
            .get_chat(chat_id)
            .and_then(|(bot, chat)| {
                let file_id = chat.photo.unwrap().big_file_id;

                bot.download_file(file_id)
            })
            .and_then(|(_, file_buffer)| {
                let save_name = file_buffer.name.replace("/", "_");

                file_buffer.save_as(format!("res/{}", save_name))
            })
            .map_err(|err| println!("{:?}", err))
    );
}
```

## To Implement

There are some actions/methods in need of implementation, those are:

- sendMediaGroup
- editMessageLiveLocation
- stopMessageLiveLocation
- answerCallbackQuery

The webhook functionality also lacks an implementation:

- setWebhook
- deleteWebhook
- getWebhookInfo

Other functionality includes:

- Payments
- Passport
- Stickers
- Games
- Inline Handling



### Contributing

All kinds of suggestions are accepted, just note that this repository is a work in progress.

### Running Tests

There is a dockerfile with all information needed for tests. Tests are organized as such:

- Unit tests are on the same file as the source file.
- Destructive actions, such as kicking a member or leaving a chat are perferably tested on integration
tests. Those should always be ignored by default.

To execute tests, build and run the container created by the Dockerfile, if docker is not available,
one can just read the file to set the appropriate environment variables and then run cargo test normally.