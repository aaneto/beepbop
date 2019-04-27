//! General tests for the telegrambot wrapper.
//!
//! Here should be included functionality tests
//! for bots. That includes running actual API
//! calls and expecting an return.
use crate::prelude::*;
use std::error::Error;
use tokio::runtime::Runtime;

#[test]
fn test_get_me() {
    let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
    let bot = Bot::new(&api_key);

    let mut runtime = Runtime::new().expect("Unable to create a runtime");

    if let Err(error) = runtime.block_on(bot.get_me()) {
        panic!(error.description().to_owned());
    }
}

#[test]
/// Test for uploading chat photo ID as a
/// message. This should not be possible,
/// since this photo cannot be reused as an ID.
fn chat_icon_send_photo_invalid() {
    let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
    let chat_id: i64 = get_argv("CHAT_ID")
        .expect("Cannot find CHAT_ID in ENV")
        .parse()
        .expect("CHAT_ID is not an valid ID.");

    let bot = Bot::new(&api_key);

    let mut runtime = Runtime::new().expect("Unable to create a runtime");

    let future = bot.get_chat(chat_id).and_then(move |(bot, chat)| {
        let file_id = chat
            .photo
            .expect("Chat must have a photo for this test")
            .big_file_id;

        let id_uploader = IdUploader::new(&file_id);
        let send_photo = SendPhoto::new(chat_id, id_uploader);

        bot.send_photo(send_photo)
    });

    match runtime.block_on(future) {
        Ok(_) => panic!("It should not be possible to resend a ChatPhoto by file_id."),
        Err(APIError::TelegramError(_)) => {}
        _ => panic!("Expected error is TelegramError."),
    }
}

#[test]
fn document_upload_thumbnail() {
    let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
    let chat_id: i64 = get_argv("CHAT_ID")
        .expect("Cannot find CHAT_ID in ENV")
        .parse()
        .expect("CHAT_ID is not an valid ID.");

    let bot = Bot::new(&api_key);

    let mut runtime = Runtime::new().expect("Unable to create a runtime");

    let pupper_thumbnail = FileUploader::new("res/puppy.jpg")
        .and_then(add_mime("image/jpg"))
        .unwrap();
    
    let text_file = FileUploader::new("res/some_text")
        .and_then(add_mime("text/plain"))
        .map(add_thumbnail(pupper_thumbnail))
        .unwrap();

    let arg = SendDocument::new(chat_id, text_file);

    if let Err(err) = runtime.block_on(bot.send_document(arg)) {
        panic!("{:#?}", err);
    }
}

fn send_photo_future(file_name: &str, mime_string: Option<&str>) -> impl Future<Item=(Bot, Message), Error=APIError> {
    let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
    let chat_id: i64 = get_argv("CHAT_ID")
        .expect("Cannot find CHAT_ID in ENV")
        .parse()
        .expect("CHAT_ID is not an valid ID.");

    let bot = Bot::new(&api_key);

    let mut uploader_res = FileUploader::new(file_name);

    if let Some(mime_str) = mime_string {
        uploader_res = uploader_res.and_then(add_mime(mime_str));
    }

    let arg = SendPhoto::new(chat_id, uploader_res.unwrap());
    bot.send_photo(arg)
}

#[test]
fn send_photo() {
    let mut runtime = Runtime::new().expect("Unable to create a runtime");

    if let Err(err) = runtime.block_on(send_photo_future("res/brownpuppy.png", Some("image/png"))) {
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
