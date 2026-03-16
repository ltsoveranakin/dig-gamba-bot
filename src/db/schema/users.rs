use crate::commands::{CommandContext, DigCommandError};

use std::error::Error;
use std::fmt::{Display, Formatter};
use surrealdb::types::SurrealValue;

pub(crate) const USER_TABLE: &str = "user";

#[derive(SurrealValue, Debug)]
pub(crate) struct UserData {
    pub(crate) balance: u64,
}

impl Default for UserData {
    fn default() -> Self {
        Self { balance: 0 }
    }
}

#[derive(Debug)]
struct DataFetchError;

impl Display for DataFetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Data fetch err")
    }
}

impl Error for DataFetchError {}

impl UserData {
    pub(crate) async fn get_user(ctx: &CommandContext<'_>) -> Result<Self, DigCommandError> {
        if let Ok(user) = ctx
            .data()
            .db
            .select::<Option<Self>>(Self::user_resource(ctx))
            .await
        {
            if let Some(user) = user {
                Ok(user)
            } else {
                ctx.reply("User profile not yet created, please setup your user with /create")
                    .await?;

                Err(Box::new(DataFetchError))
            }
        } else {
            ctx.reply("User profile not yet created, please setup your user with /create")
                .await?;

            Err(Box::new(DataFetchError))
        }
    }

    pub(crate) async fn create_user(ctx: &CommandContext<'_>) -> Result<Self, DigCommandError> {
        let user = ctx
            .data()
            .db
            .create(Self::user_resource(ctx))
            .content(UserData::default())
            .await?;

        Ok(user.unwrap())
    }

    pub(crate) fn user_resource(ctx: &CommandContext) -> (&'static str, String) {
        (USER_TABLE, ctx.author().id.get().to_string())
    }
}
