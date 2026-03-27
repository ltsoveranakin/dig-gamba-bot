mod channel_based_enum;
mod dev;
mod digging;
mod economy;
mod games;
pub(super) mod help;
mod inventory;
mod setup;

use crate::commands::digging::DiggingCommands;
use crate::commands::economy::EconomyCommands;
use crate::commands::games::GameCommands;
use crate::commands::inventory::InventoryCommands;
use crate::commands::setup::SetupCommands;
use crate::Data;
use poise::CreateReply;
use serenity::all::{Color, CreateEmbed};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::iter::Extend;
use std::sync::OnceLock;
use crate::commands::dev::DevCommands;

// Using no rwlock on the vec here since we need a &str with a static lifetime for the help command.
// Great work poise making this bullshit need unsafe rust!!!
pub(super) static COMMAND_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();

pub(crate) type CommandContext<'a> = poise::Context<'a, Data, DigCommandError>;
pub(crate) type DigCommandError = Box<dyn std::error::Error + Send + Sync>;
type DigCommand = poise::Command<Data, DigCommandError>;
type CommandVec = Vec<DigCommand>;
type CommandVecDeque = VecDeque<DigCommand>;

pub(super) trait CommandList<CV = CommandVec> {
    fn get() -> CV;
}

pub(super) struct AllCommands;

impl CommandList<CommandVecDeque> for AllCommands {
    fn get() -> CommandVecDeque {
        let command_lists = vec![
            InventoryCommands::get(),
            GameCommands::get(),
            DiggingCommands::get(),
            SetupCommands::get(),
            EconomyCommands::get(),
            DevCommands::get(),
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

enum TimeVariant {
    Rel,
}

impl Display for TimeVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Rel => "R",
        };

        f.write_str(s)
    }
}

struct CreateTime {
    secs: i64,
    /// Unused currently
    variant: TimeVariant,
}

impl CreateTime {
    fn new(secs: i64) -> Self {
        Self {
            secs,
            variant: TimeVariant::Rel,
        }
    }
}

impl Display for CreateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<t:{}:{}>", self.secs, self.variant)
    }
}
