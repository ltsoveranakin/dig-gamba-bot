use crate::commands::{default_embed, CommandContext, DigCommandError};
use crate::db::schema::item::rarity::Rarity;
use crate::db::schema::item::InventoryItem;
use poise::CreateReply;

#[poise::command(slash_command)]
pub(super) async fn dig(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let item = InventoryItem::create_new(&ctx).await?;

    let rarity = Rarity::from(item.rarity);
    let mut rarity_variant_str = rarity.to_string();

    rarity_variant_str.make_ascii_lowercase();

    ctx.send(
        CreateReply::default().embed(
            default_embed()
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
