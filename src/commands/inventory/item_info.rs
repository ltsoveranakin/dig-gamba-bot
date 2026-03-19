use crate::commands::{default_embed, default_reply, CommandContext, DigCommandError};
use crate::db::schema::item::{Item, ItemValue};
use serenity::all::{CreateAttachment, CreateEmbedAuthor};

/// Gives information on a specified item
///
/// For example, to get information on the ruby item you can use /iteminfo Ruby
#[poise::command(slash_command, category = "inventory", rename = "iteminfo", ephemeral)]
pub(super) async fn item_info(
    ctx: CommandContext<'_>,
    #[description = "The item to give information on"] item: Item,
) -> serenity::Result<(), DigCommandError> {
    let item_file_name = item.get_asset_file_name();
    let item_name = item.to_string();

    let attachment =
        CreateAttachment::path(format!("./assets/images/items/{}", item_file_name)).await?;

    ctx.send(
        default_reply()
            .embed(
                default_embed()
                    .author(
                        CreateEmbedAuthor::new(item_name.clone())
                            .icon_url(format!("attachment://{}", item_file_name)),
                    )
                    .title(item_name)
                    .description(format!(
                        "{}\nBase item value: ${}",
                        item.get_description(),
                        item.get_item_value()
                    )),
            )
            .attachment(attachment),
    )
    .await?;

    Ok(())
}
