use crate::commands::{
    default_embed, default_reply, default_reply_msg, CommandContext, DigCommandError,
};
use crate::db::schema::users::UserData;
use poise::futures_util::stream;
use poise::futures_util::stream::StreamExt;
use surrealdb::types::SurrealValue;

/// Gets the users with the highest balance in their inventory
#[poise::command(slash_command, category = "economy", rename = "baltop")]
pub(super) async fn bal_top(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let guild_id = ctx.guild_id().ok_or("Must be in server")?;

    let mut results = ctx
        .data()
        .db
        .query("SELECT * FROM user ORDER BY balance ASC LIMIT 10")
        .await?;

    let users: Vec<UserData> = results.take(0)?;

    if users.is_empty() {
        ctx.send(default_reply_msg("No users yet!")).await?;

        return Ok(());
    }

    let fields: Vec<_> = stream::iter(users)
        .filter_map(async |user| {
            let user_id = user
                .id
                .expect("All users have id")
                .key
                .into_value()
                .into_number()
                .expect("User ids stored as numbers")
                .into_int()
                .expect("Id is int") as u64;

            let user_balance = user.balance;

            let member_name = guild_id
                .member(&ctx, user_id)
                .await
                .ok()?
                .display_name()
                .to_string();

            let field_name = member_name;
            let field_content = format!("Balance: {user_balance}");
            let inline = false;

            Some((field_name, field_content, inline))
        })
        .collect()
        .await;

    ctx.send(default_reply().embed(default_embed().fields(fields)))
        .await?;

    Ok(())
}
