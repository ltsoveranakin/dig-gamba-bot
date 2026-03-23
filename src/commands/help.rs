use crate::commands::{CommandContext, CommandList, DigCommandError, COMMAND_NAMES};
use poise::builtins::HelpConfiguration;
use poise::{ChoiceParameter, CommandParameterChoice};
use std::collections::HashMap;

/// Gives help for all the commands of this bot
///
/// To show a list of commands you can run /help
///
/// To show help with a specific command you can run /help <command>
/// For example: to get help with the inventory command you can run /help inventory
#[poise::command(slash_command, category = "help", rename = "help")]
pub(crate) async fn help_command(
    ctx: CommandContext<'_>,
    #[description = "The command to get help for"] mut command: Option<CommandName>,
) -> Result<(), DigCommandError> {
    // if ctx.invoked_command_name() != "help" {
    //     command = match command {
    //         Some(c) => Some(format!("{} {}", ctx.invoked_command_name(), c.command_name)),
    //         None => Some(ctx.invoked_command_name().to_string()),
    //     };
    // }

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        include_description: true,
        extra_text_at_bottom: "Type `/help <command>` for more information on a command",
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_ref().map(|c| &*c.command_name), config).await?;

    Ok(())
}

struct CommandName {
    command_name: String,
    command_name_index: usize,
}

impl CommandName {
    fn get_names() -> &'static Vec<&'static str> {
        COMMAND_NAMES
            .get().expect("Command names to be set")
    }
}

impl ChoiceParameter for CommandName {
    fn list() -> Vec<CommandParameterChoice> {
        Self::get_names()
            .iter()
            .map(|command_name| CommandParameterChoice {
                name: command_name.to_string(),
                localizations: HashMap::new(),
                __non_exhaustive: (),
            })
            .collect()
    }

    fn from_index(index: usize) -> Option<Self> {
        Some(Self {
            command_name: Self::get_names().get(index)?.to_string(),
            command_name_index: index,
        })
    }

    fn from_name(name: &str) -> Option<Self> {
        for (i, command_name) in Self::get_names().iter().enumerate() {
            if *command_name == name {
                return Some(Self {
                    command_name: name.to_string(),
                    command_name_index: i,
                });
            }
        }

        None
    }

    fn name(&self) -> &'static str {
        &Self::get_names()[self.command_name_index]
    }

    fn localized_name(&self, _locale: &str) -> Option<&'static str> {
        None
    }
}
