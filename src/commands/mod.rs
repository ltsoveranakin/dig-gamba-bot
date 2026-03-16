mod balance;
mod create;
pub mod dig;
mod inventory;
pub mod sell;

use crate::commands::balance::balance;
use crate::commands::create::create;
use crate::commands::dig::dig;
use crate::commands::inventory::inventory;
use crate::commands::sell::sell;
use crate::Data;
use poise::CreateReply;
use serenity::all::CreateEmbed;
use std::sync::{LazyLock, RwLock};

pub(crate) type CommandContext<'a> = poise::Context<'a, Data, DigCommandError>;
pub(crate) type DigCommandError = Box<dyn std::error::Error + Send + Sync>;

pub(super) static COMMAND_LIST: LazyLock<
    RwLock<Option<Vec<poise::Command<Data, DigCommandError>>>>,
> = LazyLock::new(|| {
    let commands = vec![balance(), create(), dig(), inventory(), sell()];

    let cell = RwLock::new(Some(commands));

    cell
});

pub(super) fn default_embed() -> CreateEmbed {
    CreateEmbed::default().title("Dig Bot")
}

fn default_reply(message: impl Into<String>) -> CreateReply {
    CreateReply::default().embed(default_embed().description(message))
}
