use crate::db::schema::item::rarity::Rarity;
use crate::db::schema::item::InventoryItem;
use crate::db::schema::users::{UserData, USER_TABLE};
use crate::Data;
use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter};
use surrealdb::types::RecordId;

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

#[poise::command(slash_command)]
pub(super) async fn dig(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let item = InventoryItem::create_new(&ctx).await?;

    let rarity = Rarity::from(item.rarity);
    let mut rarity_variant_str = rarity.to_string();

    rarity_variant_str.make_ascii_lowercase();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::default()
                .title("Dig")
                .color(rarity.get_rarity_color())
                .description(format!(
                    "After some digging you found some {} {}!",
                    rarity_variant_str, item.item_type
                )),
        ),
    )
    .await?;

    Ok(())
}

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

    let mut embed = CreateEmbed::default()
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

#[poise::command(slash_command)]
pub(super) async fn sell(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let mut embed = CreateEmbed::default();

    Ok(())
}
