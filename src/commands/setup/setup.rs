use crate::commands::{default_reply_msg, CommandContext, DigCommandError};
use serenity::all::{ChannelType, CreateChannel};

const DIGGING_CATEGORY_NAME: &str = "Digging";
pub(crate) const THE_BEACH_CHANNEL_NAME: &str = "beach";

#[poise::command(
    slash_command,
    category = "setup",
    required_permissions = "MANAGE_CHANNELS"
)]
pub(super) async fn setup(ctx: CommandContext<'_>) -> serenity::Result<(), DigCommandError> {
    ctx.send(default_reply_msg("Setting up /Dig for this server"))
        .await?;

    let existing_digging_category_id;

    {
        existing_digging_category_id = ctx
            .guild()
            .ok_or("Must be in a server")?
            .channels
            .values()
            .find_map(|channel| {
                if channel.name == DIGGING_CATEGORY_NAME && channel.kind == ChannelType::Category {
                    Some(channel.id)
                } else {
                    None
                }
            });
    }

    let mut created_digging_category = false;
    let mut created_beach = false;

    let digging_category_id = if let Some(existing_category) = existing_digging_category_id {
        existing_category
    } else {
        created_digging_category = true;

        let channel_builder = CreateChannel::new(DIGGING_CATEGORY_NAME).kind(ChannelType::Category);

        let channel = ctx
            .guild_id()
            .ok_or("Must be in a server")?
            .create_channel(ctx, channel_builder)
            .await?;

        channel.id
    };

    if ctx
        .guild()
        .ok_or("Must be in a server")?
        .channels
        .values()
        .find(|channel| channel.name == THE_BEACH_CHANNEL_NAME)
        .is_none()
    {
        created_beach = true;
        let channel_builder = CreateChannel::new(THE_BEACH_CHANNEL_NAME)
            .kind(ChannelType::Text)
            .category(digging_category_id);

        ctx.guild_id()
            .ok_or("Must be in a server")?
            .create_channel(ctx, channel_builder)
            .await?;
    }

    ctx.send(default_reply_msg(format!(
        "\
            Successfully created necessary channels!\n\
            Created digging category: {created_digging_category}\n\
            Created beach channel: {created_beach}\
        "
    )))
    .await?;

    Ok(())
}
