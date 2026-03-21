use crate::commands::{
    default_embed, default_reply, default_reply_msg, CommandContext, DigCommandError,
};
use crate::db::schema::item::locations::DiggingLocation;

/// Gives information of the drop pool at your current location
///
/// You can use /droppool
#[poise::command(slash_command, category = "digging", rename = "droppool")]
pub(super) async fn drop_pool(ctx: CommandContext<'_>) -> Result<(), DigCommandError> {
    let Some(digging_location) = DiggingLocation::get_location_from_channel(ctx).await? else {
        ctx.send(default_reply_msg("This is not a valid digging location!"))
            .await?;

        return Ok(());
    };

    let generator = ctx
        .data()
        .digging_locations
        .get(&digging_location)
        .ok_or("Digging location not registered, no clue how")?;

    ctx.send(
        default_reply().embed(default_embed().fields(generator.items.iter().map(
            |generator_item| {
                (
                    generator_item.item.to_string(),
                    format!(
                        "Chance: {:.2}%",
                        (generator_item.drop_weight as f32 / generator.total_drop_weight as f32)
                            * 100.0
                    ),
                    false,
                )
            },
        ))),
    )
    .await?;

    Ok(())
}
