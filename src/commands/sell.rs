use crate::commands::{CommandContext, DigCommandError};
use serenity::all::CreateEmbed;

#[poise::command(slash_command)]
pub(super) async fn sell(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let mut embed = CreateEmbed::default();

    Ok(())
}
