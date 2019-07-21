//! The Stream module provides utilities for
//! dealing with streams of bot updates.
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use futures::Future;
use tokio::runtime::Runtime;

use crate::bot::Bot;
use crate::object::Message;
use crate::object::Update;
use crate::object::UpdateKind;
use crate::input::GetUpdates;


type UpdateSender = mpsc::Sender<Vec<Update>>;

pub struct Stream {
    bot: Bot,
    senders: Vec<UpdateSender>,
}

impl Stream {
    pub fn new(bot: &Bot) -> Stream {
        Stream {
            bot: bot.clone(),
            senders: vec![],
        }
    }

    pub fn for_each_message<'a, Fut, Func>(&mut self, closure: Func)
    where
        Fut: Future<Item = (), Error = ()> + 'static + Send,
        Func: Fn(Bot, Message) -> Fut + 'static + Send
    {
        let (sender, receiver) = mpsc::channel();

        let thread_bot = self.bot.clone();

        self.senders.push(sender);

        thread::spawn(move || {
            let mut runtime = Runtime::new().expect("Unable to create a runtime");

            for updates in receiver {
                for update in updates {
                    if let UpdateKind::Message(message) = update.data {
                        runtime.block_on(closure(thread_bot.clone(), message)).is_ok();
                    }
                }
            }
        });
    }

    pub fn run(self) {
        let mut update_offset: Option<i64> = None;
        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        loop {
            let mut get_updates_args = GetUpdates::new();
            get_updates_args.offset = update_offset;
            println!("FETCHING UPDATES OFFSET: {:?}", get_updates_args);

            let (_, updates) = runtime.block_on(self.bot.to_owned().get_updates(get_updates_args)).unwrap();

            update_offset = updates.last().map(|last_update: &Update| {
                last_update.update_id + 1
            });

            for sender in self.senders.iter() {
                sender.send(updates.clone()).unwrap();
            }

            std::thread::sleep(Duration::from_secs(1));
        }
    }
}