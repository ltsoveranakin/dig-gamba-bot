use crate::commands::setup::create::create;
use crate::commands::setup::setup::setup;
use crate::commands::{CommandList, CommandVec};

mod create;
pub(super) mod setup;

pub(super) struct SetupCommands;

impl CommandList for SetupCommands {
    fn get() -> CommandVec {
        vec![create(), setup()]
    }
}
