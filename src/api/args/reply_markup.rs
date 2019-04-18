use serde_derive::Serialize;

use crate::api::args::{ForceReply, InlineKeyboardMarkup, ReplyKeboardMarkup, ReplyKeboardRemove};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    ForceReply(ForceReply),
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeboardMarkup(ReplyKeboardMarkup),
    ReplyKeboardRemove(ReplyKeboardRemove),
}

impl From<ForceReply> for ReplyMarkup {
    fn from(force_reply: ForceReply) -> Self {
        ReplyMarkup::ForceReply(force_reply)
    }
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(inline_markup: InlineKeyboardMarkup) -> Self {
        ReplyMarkup::InlineKeyboardMarkup(inline_markup)
    }
}

impl From<ReplyKeboardMarkup> for ReplyMarkup {
    fn from(reply_markup: ReplyKeboardMarkup) -> Self {
        ReplyMarkup::ReplyKeboardMarkup(reply_markup)
    }
}

impl From<ReplyKeboardRemove> for ReplyMarkup {
    fn from(markup_remove: ReplyKeboardRemove) -> Self {
        ReplyMarkup::ReplyKeboardRemove(markup_remove)
    }
}
