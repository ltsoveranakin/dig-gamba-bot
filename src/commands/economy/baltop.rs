use crate::commands::{CommandContext, DigCommandError};

#[poise::command(slash_command, category = "games")]
pub(super) async fn bal_top(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    Ok(())
}
