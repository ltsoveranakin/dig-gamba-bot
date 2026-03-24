use crate::commands::{
    default_embed, default_reply, default_reply_msg, CommandContext, CreateTime, DigCommandError,
};
use crate::db::schema::item::inventory_item::InventoryItem;
use crate::db::schema::item::locations::DiggingLocation;
use crate::db::schema::item::rarity::Rarity;
use rand::RngExt;
use serenity::all::CreateAttachment;
use surrealdb::types::Datetime;

const DIG_COOLDOWN_TIME_SECS: i64 = 60;

static DROP_TEXTS: [&str; 2] = [
    "After some digging you found $article $rarity $item!",
    "It was hard work, but you found $article $rarity $item!",
];

/// Digs at your current location
///
/// You can use /dig to dig up items in the current channel
#[poise::command(slash_command, category = "digging")]
pub(super) async fn dig(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let db = &ctx.data().db;

    let Some(digging_location) = DiggingLocation::get_location_from_channel(ctx).await? else {
        let advice_text = if let Some(beach_channel_id) = ctx
            .guild()
            .expect("Checked if in server")
            .channels
            .values()
            .find_map(|channel| {
                (channel.name == DiggingLocation::Beach.get_channel_name()).then(|| channel.id)
            }) {
            format!("Try the <#{beach_channel_id}>, I heard it's a good place to start out!")
        } else {
            "I would recommend the beach to dig, but I can't find the channel!\nHave an admin run `/setup`".to_string()
        };

        ctx.send(default_reply_msg(format!(
            "You can't dig here!\n{advice_text}"
        )))
        .await?;

        return Ok(());
    };

    let user_id = ctx.author().id.get() as i64;
    let location_ord = digging_location as u16;

    let last_dig_used_cur_loc: Option<Datetime> = db
        .query("SELECT VALUE ldt FROM last_dug:[$id, $loc]")
        .bind(("id", user_id))
        .bind(("loc", location_ord))
        .await?
        .take(0)?;

    let time_now = Datetime::now();

    let next_allowed_dig_time = last_dig_used_cur_loc
        .map_or(0, |last_dig_time| last_dig_time.timestamp())
        + DIG_COOLDOWN_TIME_SECS;

    if time_now.timestamp() < next_allowed_dig_time {
        ctx.send(
            default_reply_msg(format!(
                "You cant dig here yet, try another location or here again {}",
                CreateTime::new(next_allowed_dig_time),
            ))
            .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    let inventory_item = InventoryItem::create_new(ctx, digging_location).await?;

    db.query("UPSERT last_dug:[$id, $loc] SET ldt = time::now();")
        .bind(("id", user_id))
        .bind(("loc", location_ord))
        .await?;

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
