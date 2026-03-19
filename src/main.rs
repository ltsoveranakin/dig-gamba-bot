mod commands;
mod data;
mod db;

use crate::commands::{AllCommands, CommandList};
use crate::data::Data;
use anyhow::Context;
use clap::Parser;
use poise::serenity_prelude::*;
use std::env;
use surrealdb::engine::local::SurrealKv;
use surrealdb::Surreal;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    token: Option<String>,
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

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: AllCommands::get(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                println!("commands registered");

                let db = Surreal::new::<SurrealKv>("data/dig_bot.db").await?;

                db.use_ns("dig_bot").use_db("slash_dig").await?;

                println!("Db set up");

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
