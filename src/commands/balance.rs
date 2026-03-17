use crate::commands::{default_reply_msg, CommandContext, DigCommandError};
use crate::db::schema::users::UserData;

#[poise::command(slash_command)]
pub(super) async fn balance(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let Ok(user) = UserData::get_user(&ctx).await else {
        return Ok(());
    };

    ctx.send(default_reply_msg(format!(
        "Your balance is: {}",
        user.balance
    )))
    .await?;

    Ok(())
}
