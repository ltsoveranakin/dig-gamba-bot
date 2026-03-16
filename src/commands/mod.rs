use crate::db::schema::UserData;
use crate::Data;

pub(crate) type CommandContext<'a> = poise::Context<'a, Data, DigCommandError>;
pub(crate) type DigCommandError = Box<dyn std::error::Error + Send + Sync>;

#[poise::command(slash_command)]
pub(super) async fn add(
    ctx: CommandContext<'_>,
    #[description = "First number"] a: i64,
    #[description = "Second number"] b: i64,
) -> Result<(), DigCommandError> {
    ctx.say(format!("{} + {} = {}", a, b, a + b)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub(super) async fn balance(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let Ok(user) = UserData::get_user(&ctx).await else {
        return Ok(());
    };

    ctx.say(format!("Your balance is: {}", user.balance))
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
pub(super) async fn create(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let user = UserData::create_user(&ctx).await?;

    ctx.reply(format!(
        "Your user profile has been created! Your starting balance is: {}",
        user.balance
    ))
    .await?;

    Ok(())
}
