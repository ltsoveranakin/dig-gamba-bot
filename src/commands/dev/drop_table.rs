use crate::commands::{default_reply_msg, CommandContext, DigCommandError};

/// Drops a table in the database
#[poise::command(slash_command, category = "dev", owners_only)]
pub(super) async fn drop_table(
    ctx: CommandContext<'_>,
    table_name: String,
) -> Result<(), DigCommandError> {
    ctx.data()
        .db
        .query("DELETE $table")
        .bind(("table", table_name.clone()))
        .await?;

    ctx.send(default_reply_msg(format!(
        "Dropped table {table_name} successfully"
    )))
    .await?;

    Ok(())
}
