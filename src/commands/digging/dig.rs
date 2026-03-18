use rand::RngExt;
use serenity::all::CreateAttachment;
use crate::commands::{default_embed, default_reply, CommandContext, DigCommandError};
use crate::db::schema::item::rarity::Rarity;
use crate::db::schema::item::schema::InventoryItem;

static DROP_TEXTS: [&str; 2] = [
    "After some digging you found $article $rarity $item!",
    "It was hard work, but you found $article $rarity $item!",
];

#[poise::command(slash_command)]
pub(super) async fn dig(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let inventory_item = InventoryItem::create_new(&ctx).await?;

    let rarity = Rarity::from_float(inventory_item.rarity);
    let mut rarity_variant_str = rarity.to_string();

    rarity_variant_str.make_ascii_lowercase();

    let item_type = inventory_item.item_type;

    let item_file_name = item_type.get_asset_file_name();

    let attachment =
        CreateAttachment::path(format!("./assets/images/items/{}", item_file_name)).await?;

    let article = if item_type.is_multiple() {
        "some"
    } else {
        if rarity.starts_with_vowel() {
            "an"
        } else {
            "a"
        }
    };

    let drop_text = DROP_TEXTS[ctx.data().rng_mut().random_range(0..DROP_TEXTS.len())]
        .replace("$article", article)
        .replace("$rarity", &rarity.to_string().to_ascii_lowercase())
        .replace("$item", &item_type.to_string());

    ctx.send(
        default_reply()
            .embed(
                default_embed()
                    .image(format!("attachment://{}", item_file_name))
                    .color(rarity.get_rarity_color())
                    .description(drop_text),
            )
            .attachment(attachment),
    )
        .await?;

    Ok(())
}