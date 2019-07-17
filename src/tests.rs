//! Integration tests for telegrambot.
use std::time::Duration;

use crate::bot::Bot;
use crate::input::FileUploader;
use crate::input::SendDocument;
use futures::Future;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
#[ignore]
fn photo_reupload() {
    let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
    let chat_id: i64 = var("CHAT_ID")
        .expect("Cannot find CHAT_ID in ENV")
        .parse()
        .expect("CHAT_ID is not an valid ID.");

    let bot = Bot::new(&api_key);

    let mut runtime = Runtime::new().expect("Unable to create a runtime");

    let fut = bot
        .get_chat(chat_id.clone())
        .and_then(|(bot, chat)| {
            let file_id = chat.photo.unwrap().big_file_id;

            bot.download_file(file_id)
        })
        .and_then(move |(bot, file_buffer)| {
            let dll_file = FileUploader::from_file(file_buffer);
            let send_doc = SendDocument::new(chat_id, dll_file);

            bot.send_document(send_doc)
        });

    if let Err(err) = runtime.block_on(fut) {
        panic!("{:#?}", err);
    }
}

use crate::input::EditLiveLocation;
use crate::input::SendLocation;
use crate::input::StopLiveLocation;

#[test]
fn live_location() {
    let api_key = var("API_KEY").expect("Cannot find API_KEY in ENV");
    let chat_id: i64 = var("CHAT_ID")
        .expect("Cannot find CHAT_ID in ENV")
        .parse()
        .expect("CHAT_ID is not an valid ID.");

    let bot = Bot::new(&api_key);

    let mut runtime = Runtime::new().expect("Unable to create a runtime");

    let send_location = SendLocation::new(chat_id, 12.0, 22.0);

    let fut = bot
        .send_location(send_location)
        .and_then(move |(bot, message)| {
            std::thread::sleep(Duration::from_millis(500));

            let edit_args = EditLiveLocation::new(message.chat.id, message.message_id, 12.0, 24.0);

            bot.edit_live_location(edit_args)
        })
        .and_then(|(bot, message)| {
            std::thread::sleep(Duration::from_millis(500));

            let stop_args = StopLiveLocation::new(message.chat.id, message.message_id);

            bot.stop_live_location(stop_args)
        });

    if let Err(err) = runtime.block_on(fut) {
        panic!("{:#?}", err);
    }
}

include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));
