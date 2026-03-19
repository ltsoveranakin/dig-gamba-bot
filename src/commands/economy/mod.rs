mod balance;
mod baltop;

use crate::commands::economy::balance::balance;
use crate::commands::{CommandList, CommandVec};

pub(super) struct EconomyCommands;

impl CommandList for EconomyCommands {
    fn get() -> CommandVec {
        vec![balance()]
    }
}
