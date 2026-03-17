mod commands;
mod data;
mod db;

use crate::commands::COMMAND_LIST;
use anyhow::Context;
use std::sync::Mutex;

use poise::serenity_prelude::*;
use rand::make_rng;
use surrealdb::engine::local::SurrealKv;
use surrealdb::Surreal;
use crate::data::Data;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("bot_token").context("token not set")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: COMMAND_LIST
                .write()
                .expect("RWLock not poisoned")
                .take()
                .expect("Command list initialized"),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                println!("commands registered");

                let db = Surreal::new::<SurrealKv>("data/dig_bot.db").await?;

                db.use_ns("dig_bot").use_db("slash_dig").await?;

                println!("Db set up");

                let rng = Mutex::new(make_rng());

                Ok(Data { db, rng })
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged();

    let mut client = ClientBuilder::new(&token, intents)
        .framework(framework)
        .await
        .context("Failed to create client")?;

    println!("Client created, starting connection");

    client.start().await?;

    Ok(())
}
