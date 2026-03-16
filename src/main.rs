mod commands;
mod db;
mod handler;

use crate::commands::COMMAND_LIST;
use anyhow::Context;

use poise::serenity_prelude::*;
use surrealdb::engine::local::SurrealKv;
use surrealdb::Surreal;
// pub(crate) struct CollectionsData {
//     users: Collection<UserSchema>,
// }

pub(crate) struct Data {
    pub(crate) db: Surreal<surrealdb::engine::local::Db>,
}

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

                Ok(Data { db })
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged();

    let mut client = ClientBuilder::new(&token, intents)
        .framework(framework)
        .await
        .context("Failed to create client")?;

    client.start().await?;

    Ok(())
}
