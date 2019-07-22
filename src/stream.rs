//! The Stream module provides utilities for
//! dealing with streams of bot updates.
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use tokio::runtime::Runtime;

use crate::bot::Bot;
use crate::error::StreamError;
use crate::input::GetUpdates;
use crate::object::Message;
use crate::object::Update;
use crate::object::UpdateKind;

#[cfg(feature = "stream-logging")]
use log::info;

type MessageSender = mpsc::Sender<Message>;

pub struct Stream {
    bot: Bot,
    message_sender: Option<MessageSender>,
}

impl Stream {
    pub fn new(bot: &Bot) -> Stream {
        Stream {
            bot: bot.clone(),
            message_sender: None,
        }
    }

    pub fn for_each_message<Func>(&mut self, closure: Func)
    where
        Func: Send + 'static + Fn(&mut Runtime, Bot, Message) -> (),
    {
        let (sender, receiver) = mpsc::channel();

        let thread_bot = self.bot.clone();

        self.message_sender = Some(sender);

        thread::spawn(move || {
            let mut runtime = Runtime::new().expect("Unable to create a runtime");

            for message in receiver {
                #[cfg(feature = "stream-logging")]
                info!("Message streamer received: {:?}", message);

                closure(&mut runtime, thread_bot.clone(), message);
            }
        });
    }

    pub fn run(self) -> Result<(), StreamError> {
        #[cfg(feature = "stream-logging")]
        env_logger::init();

        let mut update_offset: Option<i64> = None;
        let mut runtime = Runtime::new().expect("Unable to create a runtime");

        loop {
            let mut get_updates_args = GetUpdates::new();
            get_updates_args.offset = update_offset;

            #[cfg(feature = "stream-logging")]
            info!("Fetching {:?}", get_updates_args);

            let (_, updates) =
                runtime.block_on(self.bot.to_owned().get_updates(get_updates_args))?;

            update_offset = updates
                .last()
                .map(|last_update: &Update| last_update.update_id + 1);

            for update in updates {
                if let (UpdateKind::Message(message), Some(sender)) =
                    (update.data, self.message_sender.as_ref())
                {
                    sender.send(message)?;
                }
            }

            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
