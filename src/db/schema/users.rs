use crate::commands::{CommandContext, DigCommandError};

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
                Self::profile_not_created_error()
            }
        } else {
            Self::profile_not_created_error()
        }
    }

    fn profile_not_created_error() -> Result<Self, DigCommandError> {
        Err("User profile not yet created, please setup your user with /create".into())
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
