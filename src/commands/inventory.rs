use crate::commands::{default_embed, CommandContext, DigCommandError};
use crate::db::schema::item::rarity::Rarity;
use crate::db::schema::item::schema::InventoryItem;
use crate::db::schema::item::{ItemValue, ITEM_TABLE};
use crate::db::schema::users::{UserData, USER_TABLE};
use poise::CreateReply;
use serenity::all::*;
use std::time::Duration;
use surrealdb::types::{RecordId, ToSql};

const INVENTORY_RETURN_LIMIT: u32 = 5;
static EMOJI_NUMBERS: [&str; INVENTORY_RETURN_LIMIT as usize] = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣"];

#[poise::command(slash_command)]
pub(super) async fn inventory(
    ctx: CommandContext<'_>,
    #[description = "The page number of the inventory to display"] page_number: Option<u32>,
) -> Result<(), DigCommandError> {
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

    let mut fields = Vec::with_capacity(items.len());
    let mut buttons = Vec::with_capacity(items.len());

    for (i, inventory_item) in items.into_iter().enumerate() {
        let item_type_str = inventory_item.item_type.to_string();
        let item_value = inventory_item.get_item_value();
        let item_id = inventory_item.id.unwrap().key.to_sql();

        let emoji_number = EMOJI_NUMBERS[i];

        buttons.push(
            CreateButton::new(item_id.clone())
                .label(format!("{emoji_number} Sell {item_type_str}")),
        );

        fields.push((
            format!("{emoji_number} {item_type_str}",),
            format!(
                "Rarity: ({})\nFloat: {:.5}\nValue: ${}",
                Rarity::from_float(inventory_item.rarity),
                inventory_item.rarity,
                item_value
            ),
            false,
        ));
    }

    let embed = default_embed()
        .title(format!("{}'s Inventory", ctx.author().name))
        .thumbnail(ctx.author().face())
        .footer(CreateEmbedFooter::new(format!(
            "Page {} of {}",
            page_number,
            (count / INVENTORY_RETURN_LIMIT) + 1
        )))
        .fields(fields);

    let components = CreateActionRow::Buttons(buttons);

    let m = ctx
        .send(
            CreateReply::default()
                .embed(embed)
                .components(vec![components]),
        )
        .await?;

    while let Some(mci) = m
        .message()
        .await?
        .await_component_interaction(&ctx.serenity_context().shard)
        .author_id(ctx.author().id)
        .timeout(Duration::from_secs(60))
        .await
    {
        let mut user = UserData::get_user(&ctx).await?;

        let item: Option<InventoryItem> =
            db.delete((ITEM_TABLE, mci.data.custom_id.clone())).await?;

        let Some(item) = item else {
            mci.create_response(
                ctx.serenity_context(),
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("An error occurred (could not find item to delete)"),
                ),
            )
            .await?;

            continue;
        };

        user.balance += item.get_item_value();

        db.update::<Option<UserData>>(UserData::user_resource(&ctx))
            .content(user)
            .await?;

        mci.create_response(
            ctx.serenity_context(),
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Item sold successfully!")
                    .ephemeral(true),
            ),
        )
        .await?;
    }

    Ok(())
}
