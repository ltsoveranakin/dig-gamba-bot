use crate::commands::gambling::coin_flip::coin_flip;
use crate::commands::{CommandList, CommandVec};

mod coin_flip;

pub(super) struct GamblingCommands;

impl CommandList for GamblingCommands {
    fn get() -> CommandVec {
        vec![coin_flip()]
    }
}
