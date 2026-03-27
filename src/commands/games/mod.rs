use crate::commands::games::coin_flip::coin_flip;
use crate::commands::{CommandContext, CommandList, CommandVec, DigCommandError};
use derive_enum_all_values::AllValues;
use poise::ChoiceParameter;

mod coin_flip;

pub(super) struct GameCommands;

impl CommandList for GameCommands {
    fn get() -> CommandVec {
        vec![coin_flip()]
    }
}

#[derive(AllValues, poise::ChoiceParameter)]
enum CasinoRoom {
    CoinFlip,
}

impl CasinoRoom {
    fn get_channel_name(&self) -> String {
        self.name().to_lowercase()
    }
}

trait CheckCasinoRoom {
    async fn check_casino_room(ctx: Self, room: CasinoRoom) -> Result<bool, DigCommandError>;
}

impl<'a> CheckCasinoRoom for CommandContext<'a> {
    async fn check_casino_room(ctx: Self, room: CasinoRoom) -> Result<bool, DigCommandError> {
        let channel = ctx.guild_channel().await.ok_or("Must be in server")?;

        Ok(false)
    }
}
