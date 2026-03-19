mod bal_top;
mod balance;

use crate::commands::economy::bal_top::bal_top;
use crate::commands::economy::balance::balance;
use crate::commands::{CommandList, CommandVec};

pub(super) struct EconomyCommands;

impl CommandList for EconomyCommands {
    fn get() -> CommandVec {
        vec![balance(), bal_top()]
    }
}
