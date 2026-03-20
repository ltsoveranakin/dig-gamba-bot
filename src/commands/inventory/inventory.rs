use crate::commands::{
    default_embed, default_reply, default_reply_msg, CommandContext, DigCommandError,
};
use crate::db::schema::item::inventory_item::InventoryItem;
use crate::db::schema::item::rarity::Rarity;
use crate::db::schema::item::{ItemValue, ITEM_TABLE};
use crate::db::schema::users::{UserData, USER_TABLE};
use serenity::all::{
    ComponentInteractionDataKind, CreateActionRow, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, User,
};
use std::time::Duration;
use surrealdb::types::{RecordId, ToSql};

const INVENTORY_RETURN_LIMIT: u32 = 5;

/// Displays your inventory (by default) or another user's inventory if specified at a given page number
///
/// To display the first 5 items of your inventory, you can use /inventory
///
/// You can view other pages of your inventory with /inventory <page number>
/// For example: viewing inventory page 3 cna be done with /inventory 3
///
/// The same applies for inventories of other users.
/// You can view another user's inventory with /inventory <page number> default <user>
///
/// You can also sort the inventory in a few ways, such as Item type and Rarity
/// To sort the inventory display you can use /inventory <page number> <sort mode>
/// For example: viewing page 2 of your inventory sorted by item rarity can be done with /inventory 2 rarity
#[poise::command(slash_command, category = "inventory")]
pub(super) async fn inventory(
    ctx: CommandContext<'_>,
    #[description = "The page number of the inventory to display"] page_number: Option<u32>,
    #[description = "The order to return the data"] sort_mode: Option<SortMode>,
    #[description = "The user to display inventory of"] target_user: Option<User>,
) -> serenity::Result<(), DigCommandError> {
    let db = &ctx.data().db;

    let target_user = target_user.as_ref().unwrap_or_else(|| ctx.author());

    let owner = RecordId::new(USER_TABLE, target_user.id.get().to_string());

    let page_number = page_number.unwrap_or(1);
    let page_index = page_number - 1;
    let start = page_index * INVENTORY_RETURN_LIMIT;

    let order_by_text = if let Some(sort_mode) = sort_mode {
        if matches!(sort_mode, SortMode::Default) {
            String::new()
        } else {
            let sort_field = match sort_mode {
                SortMode::Rarity => "rarity",

                SortMode::Item => "item_type",

                SortMode::Default => unreachable!(),
            };

            format!(" ORDER BY {} DESC", sort_field)
        }
    } else {
        String::new()
    };

    let mut results = db
        .query(format!(
            r#"
        SELECT * FROM item WHERE owner = $owner{order_by_text} LIMIT $limit START $start;
        SELECT VALUE count() FROM item WHERE owner = $owner GROUP ALL;
        "#,
        ))
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

    let empty_inventory = fields.is_empty();

    let mut embed = default_embed()
        .title(format!("{}'s Inventory", target_user.name))
        .thumbnail(target_user.face())
        .footer(CreateEmbedFooter::new(format!(
            "Page {} of {}",
            page_number,
            ((count + INVENTORY_RETURN_LIMIT) - 1) / INVENTORY_RETURN_LIMIT
        )));

    if !empty_inventory {
        embed = embed
            .description("Empty inventory, get to digging son!")
            .fields(fields);
    }

    let mut create_reply = default_reply().embed(embed);

    if empty_inventory {
        let components = vec![CreateActionRow::SelectMenu(
            CreateSelectMenu::new("sell_select", CreateSelectMenuKind::String { options })
                .placeholder("Sell items")
                .min_values(1)
                .max_values(5),
        )];

        create_reply = create_reply.components(components);
    }

    let m = ctx.send(create_reply).await?;

    while let Some(mci) = m
        .message()
        .await?
        .await_component_interaction(&ctx.serenity_context().shard)
        .author_id(target_user.id)
        .timeout(Duration::from_secs(60))
        .await
    {
        let mut user = UserData::get_user(ctx).await?;

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

        let user_balance = user.balance;

        db.update::<Option<UserData>>(UserData::user_resource(ctx))
            .content(user)
            .await?;

        mci.create_response(
            ctx.serenity_context(),
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!(
                        "Item{} sold successfully! Your balance is now ${}",
                        plural_str, user_balance
                    ))
                    .ephemeral(true),
            ),
        )
        .await?;
    }

    Ok(())
}

#[derive(poise::ChoiceParameter)]
enum SortMode {
    #[name = "item"]
    Item,
    #[name = "rarity"]
    Rarity,
    #[name = "default"]
    Default,
}
