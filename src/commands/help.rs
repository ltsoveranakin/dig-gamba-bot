use crate::commands::{CommandContext, CommandList, CommandVec, DigCommandError};
use poise::builtins::HelpConfiguration;

pub(super) struct HelpCommands;

impl CommandList for HelpCommands {
    fn get() -> CommandVec {
        vec![help()]
    }
}

#[poise::command(slash_command, category = "help")]
async fn help(
    ctx: CommandContext<'_>,
    #[description = "The command to get help for"] mut command: Option<String>,
) -> Result<(), DigCommandError> {
    if ctx.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c)),
            None => Some(ctx.invoked_command_name().to_string()),
        };
    }

    let extra_text_at_bottom = "Type `/help <command>` for more info on a command";

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        include_description: true,
        extra_text_at_bottom,
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
