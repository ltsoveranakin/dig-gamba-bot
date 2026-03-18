use crate::commands::{default_reply_msg, CommandContext, DigCommandError};
use crate::db::schema::users::UserData;

#[poise::command(slash_command)]
pub(super) async fn create(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let user = UserData::create_user(&ctx).await?;

    ctx.send(default_reply_msg(format!(
        "Your user profile has been created! Your starting balance is: {}",
        user.balance
    )))
    .await?;

    Ok(())
}
