use crate::commands::{default_reply_msg, CommandContext, DigCommandError};
use crate::db::schema::users::UserData;
use serenity::all::User;

#[poise::command(slash_command, category = "economy")]
pub(super) async fn balance(
    ctx: CommandContext<'_>,
    #[description = "The user's balance to display"] target_user: Option<User>,
) -> Result<(), DigCommandError> {
    let target_user = target_user.as_ref().unwrap_or_else(|| ctx.author());

    let user = UserData::get_user_by_id(&ctx, target_user.id).await?;

    ctx.send(default_reply_msg(format!(
        "{}'s balance is: {}",
        target_user.name, user.balance
    )))
    .await?;

    Ok(())
}
