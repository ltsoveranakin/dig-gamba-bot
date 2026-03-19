mod digging;
mod economy;
mod games;
mod help;
mod inventory;
mod setup;

use crate::commands::digging::DiggingCommands;
use crate::commands::economy::EconomyCommands;
use crate::commands::games::GameCommands;
use crate::commands::help::HelpCommands;
use crate::commands::inventory::InventoryCommands;
use crate::commands::setup::SetupCommands;
use crate::Data;
use poise::CreateReply;
use serenity::all::{Color, CreateEmbed};
use std::iter::Extend;

pub(crate) type CommandContext<'a> = poise::Context<'a, Data, DigCommandError>;
pub(crate) type DigCommandError = Box<dyn std::error::Error + Send + Sync>;

type CommandVec = Vec<poise::Command<Data, DigCommandError>>;

pub(super) trait CommandList {
    fn get() -> CommandVec;
}

pub(super) struct AllCommands;

impl CommandList for AllCommands {
    fn get() -> CommandVec {
        let command_lists = vec![
            InventoryCommands::get(),
            GameCommands::get(),
            DiggingCommands::get(),
            SetupCommands::get(),
            EconomyCommands::get(),
            HelpCommands::get(),
        ];

        command_lists.into_iter().flatten().collect()
    }
}

pub(super) fn default_embed() -> CreateEmbed {
    CreateEmbed::default()
        .title("/DIG")
        .color(Color::from_rgb(112, 50, 2))
}

fn default_reply() -> CreateReply {
    CreateReply::default()
}

fn default_reply_msg(message: impl Into<String>) -> CreateReply {
    default_reply().embed(default_embed().description(message))
}
