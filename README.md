## Telegrambot

Yet another Telegram API Wrapper in Rust. This is intended to be a
less opinionated API Raw Wrapper with support for common methods and
connections.



### Methods



There are some methods implemented and others in need of an implementation, here is the list:



- [x] getMe
- [x] getUpdates
- [x] sendMessage
- [x] forwardMessage
- [x] sendPhoto
- [x] sendAudio
- [x] sendDocument
- [ ] sendVideo
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