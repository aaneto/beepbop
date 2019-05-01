## Beepbop

# Overview

Beepbop is yet another telegram BOT API wrapper, this lib
aims to cover most methods and also provide some streaming
infrastructure based on the get_updates method.

## Actions

Every action is supposed to be a function performed by the bot,
returning a future with a reference to the bot and the data
of the last action.

```rust
use std::env::var;
use telegrambot::prelude::*;

let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");

let bot = Bot::new(&api_key);

tokio::run(
    bot
        .get_me()
        .map(|(_bot, me): (Bot, User)| println!("{:?}", me))
        .map_err(|err| println("{:?}", err))
);
```

## Multiple Actions

Actions can be chained with the use of the and_then combinator, since they are futures.


```rust
use std::env::var;
use telegrambot::prelude::*;

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

            file_buffer.save_as(format("res/{}", save_name))
        })
        .map_err(|err| println("{:?}", err))
);

```

### Methods

There are some methods implemented and others in need of an implementation, here is the list:

- [x] getMe
- [x] getUpdates
- [x] sendMessage
- [x] forwardMessage
- [x] sendPhoto
- [x] sendAudio
- [x] sendDocument
- [x] sendVideo
- [ ] sendAnimation
- [x] sendVoice
- [ ] sendVideoNote
- [ ] sendMediaGroup
- [x] sendLocation
- [ ] editMessageLiveLocation
- [ ] stopMessageLiveLocation
- [ ] sendVenue
- [x] sendContact
- [ ] sendChatAction
- [ ] getUserProfilePhotos
- [x] getFile
- [ ] kickChatMember
- [ ] unbanChatMember
- [x] restrictChatMember
- [x] promoteChatMember
- [ ] exportChatInviteLink
- [x] setChatPhoto
- [x] deleteChatPhoto
- [x] setChatTitle
- [x] setChatDescription
- [x] pinChatMessage
- [x] unpinChatMessage
- [x] leaveChat
- [x] getChat
- [x] getChatAdministrators
- [x] getChatMembersCount
- [x] getChatMember
- [x] setChatStickerSet
- [x] deleteChatStickerSet
- [ ] answerCallbackQuery

There is also functionalities not currently implemented, such as:

- [ ] setWebhook
- [ ] deleteWebhook
- [ ] getWebhookInfo

These are for getting updates via a webhook and involve setting up a tcp listener that reacts to responses with new updates. 

Since this lib aims to be disjoint of the robot actual logic, only creating, deleting and getting info about the webhook are necessary for an implementation to be considered done.



And last there are the additional stuff that involve:



- [ ] Payments
- [ ] Passport
- [ ] Stickers
- [ ] Games
- [ ] Inline Handling



### Contributing

All kinds of Pull Requests are welcome, from implementations to design suggestions. Just fork the code and create a PR named after the feature to be added.


### Running Tests

There are not a lot of tests right now, but you can run the tests by building the docker image after filling the important metadata (API_KEY only, for now).

You can also set the env variables yourself and run cargo test.