use crate::commands::{default_embed, CommandContext, DigCommandError};
use crate::db::schema::item::rarity::Rarity;
use crate::db::schema::item::InventoryItem;
use crate::db::schema::users::USER_TABLE;
use poise::CreateReply;
use serenity::all::CreateEmbedFooter;
use surrealdb::types::RecordId;

#[poise::command(slash_command)]
pub(super) async fn inventory(
    ctx: CommandContext<'_>,
    #[description = "The page number of the inventory to display"] page_number: Option<u32>,
) -> Result<(), DigCommandError> {
    const INVENTORY_RETURN_LIMIT: u32 = 5;

    let db = &ctx.data().db;
    let owner = RecordId::new(USER_TABLE, ctx.author().id.get().to_string());

    let page_number = page_number.unwrap_or(1);
    let page_index = page_number - 1;
    let start = page_index * INVENTORY_RETURN_LIMIT;

    let mut results = db
        .query(
            r#"
        SELECT * FROM item WHERE owner = $owner LIMIT $limit START $start;
        SELECT VALUE count() FROM item WHERE owner = $owner GROUP ALL;
        "#,
        )
        .bind(("owner", owner))
        .bind(("limit", INVENTORY_RETURN_LIMIT))
        .bind(("start", start))
        .await?;

    let items: Vec<InventoryItem> = results.take(0)?;
    let count = results.take::<Option<u32>>(1)?.unwrap_or(0);

    let mut embed = default_embed()
        .title(format!("{}'s Inventory", ctx.author().name))
        .thumbnail(ctx.author().face())
        .footer(CreateEmbedFooter::new(format!(
            "Page {} of {}",
            page_number,
            (count / INVENTORY_RETURN_LIMIT) + 1
        )));

    embed = embed.fields(items.iter().map(|item| {
        (
            item.item_type.to_string(),
            format!("Rarity: {:.5} ({})", item.rarity, Rarity::from(item.rarity)),
            false,
        )
    }));

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
