use crate::commands::{default_reply_msg, CommandContext, DigCommandError};
use serenity::all::{ChannelType, CreateChannel};

const DIGGING_CATEGORY_NAME: &str = "Digging";
const THE_BEACH_CHANNEL_NAME: &str = "The-Beach";

#[poise::command(
    slash_command,
    category = "setup",
    required_permissions = "MANAGE_CHANNELS"
)]
pub(super) async fn setup(ctx: CommandContext<'_>) -> serenity::Result<(), DigCommandError> {
    ctx.send(default_reply_msg("Setting up /Dig for this server"))
        .await?;

    let guild_id = ctx.guild_id().ok_or("Must be in a server")?;

    let channels = guild_id.channels(ctx).await?;

    let mut created_digging_category = false;
    let mut created_beach = false;

    let existing_digging_category_id = channels.values().find_map(|channel| {
        if channel.name == DIGGING_CATEGORY_NAME && channel.kind == ChannelType::Category {
            Some(channel.id)
        } else {
            None
        }
    });

    let digging_category_id = if let Some(existing_category) = existing_digging_category_id {
        existing_category
    } else {
        created_digging_category = true;

        let channel_builder = CreateChannel::new(DIGGING_CATEGORY_NAME).kind(ChannelType::Category);

        let channel = guild_id.create_channel(ctx, channel_builder).await?;

        channel.id
    };

    if channels
        .values()
        .find(|channel| channel.name == THE_BEACH_CHANNEL_NAME)
        .is_none()
    {
        created_beach = true;
        let channel_builder = CreateChannel::new(THE_BEACH_CHANNEL_NAME)
            .kind(ChannelType::Text)
            .category(digging_category_id);

        guild_id.create_channel(ctx, channel_builder).await?;
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
