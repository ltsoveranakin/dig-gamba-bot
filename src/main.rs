mod commands;
mod data;
mod db;

use crate::commands::help::help_command;
use crate::commands::{AllCommands, CommandList, COMMAND_NAMES};
use crate::data::Data;
use crate::db::setup_db;
use anyhow::Context;
use clap::Parser;
use poise::serenity_prelude::*;
use std::env;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    token: Option<String>,
    #[arg(long)]
    drop_users: bool,
    #[arg(long)]
    drop_items: bool,
    #[arg(long)]
    drop_last_dug: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let token = if let Some(token) = args.token {
        println!("Using argument token");

        token
    } else {
        println!("No argument token, trying environment token");

        env::var("bot_token").context("token not set")?
    };

    let mut commands = AllCommands::get();
    let mut command_name_strs: Vec<&'static str> = Vec::with_capacity(commands.len() + 1);

    command_name_strs.push("help");

    for command in &commands {
        let str = command.name.clone().leak();

        command_name_strs.push(str);
    }

    COMMAND_NAMES
        .set(command_name_strs)
        .expect("Command name list to not be set");

    commands.push_front(help_command());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            owners: vec![UserId::new(846173524469219358)].into_iter().collect(),
            commands: commands.into_iter().collect(),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                println!("commands registered");

                let db = setup_db().await?;

                println!("Db set up");

                if args.drop_users {
                    // Holay Molay
                    println!("Dropping users table");
                    db.query("DELETE user").await?;
                }

                if args.drop_items {
                    // Holay Molay
                    println!("Dropping items table");
                    db.query("DELETE item").await?;
                }

                if args.drop_last_dug {
                    // Holay Molay
                    println!("Dropping last_dug table");
                    db.query("DELETE last_dug").await?;
                }

                println!("Bot ready");
                Ok(Data::new(db))
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILDS;

    let mut client = ClientBuilder::new(&token, intents)
        .framework(framework)
        .await
        .context("Failed to create client")?;

    println!("Client created, starting connection");

    client.start().await?;

    Ok(())
}
