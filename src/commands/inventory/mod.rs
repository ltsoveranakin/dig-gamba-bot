mod inventory;
mod item_info;

use crate::commands::inventory::inventory::inventory;
use crate::commands::inventory::item_info::item_info;
use crate::commands::{CommandList, CommandVec};
use serenity::all::*;

pub(super) struct InventoryCommands;

impl CommandList for InventoryCommands {
    fn get() -> CommandVec {
        vec![inventory(), item_info()]
    }
}
