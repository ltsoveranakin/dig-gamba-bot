mod dig;

use crate::commands::digging::dig::dig;
use crate::commands::{
    default_reply_msg, CommandContext, CommandList, CommandVec, DigCommandError,
};
use derive_enum_all_values::AllValues;
use rand::prelude::IndexedRandom;
use rand::RngExt;
use std::collections::HashSet;
use std::sync::LazyLock;

const ALLOWED_DIGGING_CHANNELS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut allowed_channels = vec!["dev-testing"];

    allowed_channels.extend(
        DiggingChannels::all_values()
            .iter()
            .map(|digging_channel| digging_channel.get_channel_name()),
    );

    allowed_channels.into_iter().collect()
});

pub(super) struct DiggingCommands;

impl CommandList for DiggingCommands {
    fn get() -> CommandVec {
        vec![dig()]
    }
}

#[derive(AllValues)]
pub(crate) enum DiggingChannels {
    Beach,
}

impl DiggingChannels {
    pub(crate) fn get_channel_name(&self) -> &'static str {
        match self {
            Self::Beach => "beach",
        }
    }
}

async fn can_dig_in_channel(ctx: &CommandContext<'_>) -> Result<bool, DigCommandError> {
    let channel = ctx.guild_channel().await.ok_or("Must be in a server")?;

    if ALLOWED_DIGGING_CHANNELS.contains(&*channel.name) {
        Ok(true)
    } else {
        ctx.send(default_reply_msg(
            "You can't dig here!\nTry the beach, I heard it's a good place to start.",
        ))
        .await?;
        Ok(false)
    }
}
