mod dig;

use crate::commands::digging::dig::dig;
use crate::commands::setup::setup::THE_BEACH_CHANNEL_NAME;
use crate::commands::{
    default_reply_msg, CommandContext, CommandList, CommandVec, DigCommandError,
};
use rand::prelude::IndexedRandom;
use rand::RngExt;

pub(super) struct DiggingCommands;

impl CommandList for DiggingCommands {
    fn get() -> CommandVec {
        vec![dig()]
    }
}

async fn can_dig_in_channel(ctx: &CommandContext<'_>) -> Result<bool, DigCommandError> {
    let channel = ctx.guild_channel().await.ok_or("Must be in a server")?;

    let can_dig = if channel.name == THE_BEACH_CHANNEL_NAME {
        true
    } else {
        ctx.send(default_reply_msg(
            "You can't dig here!\nTry the beach, I heard there's some good things to find there.",
        ))
        .await?;
        false
    };

    Ok(can_dig)
}
