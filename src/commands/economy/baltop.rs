use crate::commands::{CommandContext, DigCommandError};

#[poise::command(slash_command, category = "economy")]
pub(super) async fn bal_top(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    Ok(())
}
