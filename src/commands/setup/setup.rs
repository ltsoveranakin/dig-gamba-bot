use crate::commands::{default_reply_msg, CommandContext, DigCommandError};
use crate::db::schema::item::locations::DiggingLocation;
use serenity::all::{ChannelType, CreateChannel};
use std::collections::HashSet;

const DIGGING_CATEGORY_NAME: &str = "Digging";

/// Automatically sets up the required channels needed to use this bot
#[poise::command(
    slash_command,
    category = "setup",
    required_permissions = "MANAGE_CHANNELS",
    ephemeral
)]
pub(super) async fn setup(ctx: CommandContext<'_>) -> serenity::Result<(), DigCommandError> {
    ctx.send(default_reply_msg("Setting up /Dig for this server"))
        .await?;

    let guild_id = ctx.guild_id().ok_or("Must be in a server")?;

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
    let mut created_channels = Vec::new();

    let digging_category_id = if let Some(existing_category) = existing_digging_category_id {
        existing_category
    } else {
        created_digging_category = true;

        let channel_builder = CreateChannel::new(DIGGING_CATEGORY_NAME).kind(ChannelType::Category);

        let channel = guild_id.create_channel(ctx, channel_builder).await?;

        channel.id
    };

    let channel_name_map: HashSet<String> = ctx
        .guild()
        .ok_or("Must be in a server")?
        .channels
        .values()
        .map(|channel| channel.name.clone())
        .collect();

    for digging_location in DiggingLocation::all_values() {
        let channel_name = digging_location.get_channel_name();

        if !channel_name_map.contains(channel_name) {
            let channel_builder = CreateChannel::new(channel_name)
                .kind(ChannelType::Text)
                .category(digging_category_id);

            guild_id.create_channel(ctx, channel_builder).await?;

            created_channels.push(channel_name);
        }
    }

    ctx.send(default_reply_msg(format!(
        "\
            Successfully created necessary channels!\n\
            Created digging category: {created_digging_category}\n\
            Created channels: {}\
        ",
        created_channels.join(", ")
    )))
    .await?;

    Ok(())
}
