use crate::commands::{default_embed, default_reply, CommandContext, DigCommandError};
use crate::db::schema::item::Item;
use serenity::all::{CreateAttachment, CreateEmbedAuthor};

#[poise::command(slash_command, category = "inventory", rename = "iteminfo")]
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
                        CreateEmbedAuthor::new(item_name)
                            .icon_url(format!("attachment://{}", item_file_name)),
                    )
                    .title(item_name)
                    .description(item.get_description()),
            )
            .attachment(attachment),
    )
    .await?;

    Ok(())
}
