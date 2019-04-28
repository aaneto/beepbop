//! Integration tests for telegrambot.
use futures::Future;
use tokio::runtime::Runtime;
use crate::util::get_argv;
use crate::api::Bot;
use crate::api::uploaders::FileUploader;
use crate::api::args::SendDocument;

#[test]
fn photo_reupload() {
    let api_key = get_argv("API_KEY").expect("Cannot find API_KEY in ENV");
    let chat_id: i64 = get_argv("CHAT_ID")
        .expect("Cannot find CHAT_ID in ENV")
        .parse()
        .expect("CHAT_ID is not an valid ID.");

    let bot = Bot::new(&api_key);

    let mut runtime = Runtime::new().expect("Unable to create a runtime");

    let fut = bot.get_chat(chat_id.clone())
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