use crate::commands::games::coin_flip::coin_flip;
use crate::commands::{CommandList, CommandVec};

mod coin_flip;

pub(super) struct GameCommands;

impl CommandList for GameCommands {
    fn get() -> CommandVec {
        vec![coin_flip()]
    }
}
