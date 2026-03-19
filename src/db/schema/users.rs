use crate::commands::{CommandContext, DigCommandError};
use serenity::all::UserId;

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
        Self::get_user_by_id(ctx, ctx.author().id).await
    }

    // pub(crate) async fn get_user_by_serenity_user(
    //     ctx: &CommandContext<'_>,
    //     user: Option<User>,
    // ) -> Result<Self, DigCommandError> {
    //     Self::get_user_by_id(ctx, user.map(|user| user.id))
    // }

    pub(crate) async fn get_user_by_id(
        ctx: &CommandContext<'_>,
        user_id: UserId,
    ) -> Result<Self, DigCommandError> {
        if let Ok(user) = ctx
            .data()
            .db
            .select::<Option<Self>>(Self::user_resource_by_id(user_id))
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
        Err("User profile not yet created, please setup your user with `/create`".into())
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
        Self::user_resource_by_id(ctx.author().id)
    }

    fn user_resource_by_id(user_id: UserId) -> (&'static str, String) {
        (USER_TABLE, user_id.get().to_string())
    }
}
