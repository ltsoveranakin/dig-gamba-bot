use crate::commands::{CommandContext, DigCommandError};
use serenity::all::UserId;
use surrealdb::types::{RecordId, SurrealValue};

pub(crate) const USER_TABLE: &str = "user";

type UserResource = (&'static str, i64);

#[derive(SurrealValue, Debug)]
pub(crate) struct UserData {
    pub(crate) balance: u64,
    pub(crate) id: Option<RecordId>,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            balance: 0,
            id: None,
        }
    }
}

impl UserData {
    pub(crate) async fn get_user(ctx: CommandContext<'_>) -> Result<Self, DigCommandError> {
        Self::get_user_by_id(ctx, ctx.author().id).await
    }

    pub(crate) async fn get_user_by_id(
        ctx: CommandContext<'_>,
        user_id: UserId,
    ) -> Result<Self, DigCommandError> {
        let user = match ctx
            .data()
            .db
            .select::<Option<Self>>(Self::user_resource_by_id(user_id))
            .await?
        {
            Some(user) => user,

            None => Self::create_user_by_id(ctx, user_id).await?,
        };

        Ok(user)
    }

    fn profile_not_created_error() -> Result<Self, DigCommandError> {
        Err("User profile not yet created, please setup your user with `/create`".into())
    }

    pub(crate) async fn create_user(ctx: CommandContext<'_>) -> Result<Self, DigCommandError> {
        let user = ctx
            .data()
            .db
            .create(Self::user_resource(ctx))
            .content(UserData::default())
            .await?;

        Ok(user.unwrap())
    }

    pub(crate) async fn create_user_by_id(
        ctx: CommandContext<'_>,
        user_id: UserId,
    ) -> Result<Self, DigCommandError> {
        let user = ctx
            .data()
            .db
            .create(Self::user_resource_by_id(user_id))
            .content(UserData::default())
            .await?;

        Ok(user.unwrap())
    }

    pub(crate) fn user_resource(ctx: CommandContext) -> UserResource {
        Self::user_resource_by_id(ctx.author().id)
    }

    fn user_resource_by_id(user_id: UserId) -> UserResource {
        (USER_TABLE, user_id.get() as i64)
    }
}
