mod dig;
mod drop_pool;

use crate::commands::digging::dig::dig;
use crate::commands::digging::drop_pool::drop_pool;
use crate::commands::{CommandList, CommandVec};
use rand::prelude::IndexedRandom;
use rand::RngExt;

pub(super) struct DiggingCommands;

impl CommandList for DiggingCommands {
    fn get() -> CommandVec {
        vec![dig(), drop_pool()]
    }
}
