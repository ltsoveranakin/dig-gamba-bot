use crate::commands::dev::drop_table::drop_table;
use crate::commands::{CommandList, CommandVec};

mod drop_table;

pub(super) struct DevCommands;

impl CommandList for DevCommands {
    fn get() -> CommandVec {
        vec![drop_table()]
    }
}
