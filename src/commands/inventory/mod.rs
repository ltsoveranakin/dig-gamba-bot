use crate::commands::{
    default_embed, default_reply, default_reply_msg, CommandContext, CommandList, CommandVec,
    DigCommandError,
};
use crate::db::schema::item::rarity::Rarity;
use crate::db::schema::item::schema::InventoryItem;
use crate::db::schema::item::{ItemValue, ITEM_TABLE};
use crate::db::schema::users::{UserData, USER_TABLE};
use serenity::all::*;
use std::time::Duration;
use surrealdb::types::{RecordId, ToSql};

pub(super) struct InventoryCommands;

impl CommandList for InventoryCommands {
    fn get() -> CommandVec {
        vec![inventory()]
    }
}

const INVENTORY_RETURN_LIMIT: u32 = 5;
static EMOJI_NUMBERS: [&str; INVENTORY_RETURN_LIMIT as usize] = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣"];

#[poise::command(slash_command)]
async fn inventory(
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

    if items.is_empty() {
        ctx.send(default_reply_msg("Invalid page number provided!"))
            .await?;

        return Ok(());
    }

    let mut fields = Vec::with_capacity(items.len());
    let mut options = Vec::with_capacity(items.len());

    for inventory_item in items.into_iter() {
        let item_type_str = inventory_item.item_type.to_string();
        let item_value = inventory_item.get_item_value();
        let item_record_key_str = inventory_item.id.unwrap().key.to_sql();
        let rarity = inventory_item.rarity;

        options.push(CreateSelectMenuOption::new(
            format!(
                "{} ({}) ${}",
                item_type_str,
                Rarity::from_float(rarity),
                item_value
            ),
            item_record_key_str,
        ));

        fields.push((
            item_type_str,
            format!(
                "Rarity: ({})\nFloat: {:.5}\nValue: ${}",
                Rarity::from_float(inventory_item.rarity),
                inventory_item.rarity,
                item_value
            ),
            false,
        ));
    }

    let components = vec![CreateActionRow::SelectMenu(
        CreateSelectMenu::new("sell_select", CreateSelectMenuKind::String { options })
            .placeholder("Choose items to sell")
            .min_values(1)
            .max_values(5),
    )];

    let embed = default_embed()
        .title(format!("{}'s Inventory", ctx.author().name))
        .thumbnail(ctx.author().face())
        .footer(CreateEmbedFooter::new(format!(
            "Page {} of {}",
            page_number,
            ((count + INVENTORY_RETURN_LIMIT) - 1) / INVENTORY_RETURN_LIMIT
        )))
        .fields(fields);

    let m = ctx
        .send(default_reply().embed(embed).components(components))
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

        let items: Vec<InventoryItem> = match mci.data.kind {
            ComponentInteractionDataKind::StringSelect { ref values } => db
                .query("DELETE item WHERE id IN $values RETURN BEFORE;")
                .bind((
                    "values",
                    values
                        .iter()
                        .map(|record_key| RecordId::new(ITEM_TABLE, record_key.clone()))
                        .collect::<Vec<RecordId>>(),
                ))
                .await?
                .take(0)?,

            _ => {
                ctx.send(default_reply_msg(
                    "Unknown interaction, your item has not been sold",
                ))
                .await?;

                continue;
            }
        };

        if items.is_empty() {
            mci.create_response(
                ctx.serenity_context(),
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("An error occurred (could not find item(s) to sell). Your items have not been sold."),
                ),
            )
            .await?;

            continue;
        };

        let plural_str = if items.len() != 1 { "s" } else { "" };

        for item in items {
            user.balance += item.get_item_value();
        }

        db.update::<Option<UserData>>(UserData::user_resource(&ctx))
            .content(user)
            .await?;

        mci.create_response(
            ctx.serenity_context(),
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!(
                        "Item{} sold successfully! Your balance is now ${}",
                        plural_str, user.balance
                    ))
                    .ephemeral(true),
            ),
        )
        .await?;
    }

    Ok(())
}
