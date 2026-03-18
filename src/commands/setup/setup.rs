use crate::commands::{CommandContext, DigCommandError};

#[poise::command(slash_command, category = "setup")]
pub(super) async fn setup(ctx: CommandContext<'_>) -> serenity::Result<(), DigCommandError> {
    Ok(())
}
