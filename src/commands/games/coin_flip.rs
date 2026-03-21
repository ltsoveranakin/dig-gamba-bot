use crate::commands::{
    default_embed, default_reply_msg, CommandContext, CreateTime, DigCommandError,
};
use crate::db::schema::users::UserData;
use poise::CreateReply;
use rand::RngExt;
use serenity::all::{Mentionable, User};
use std::fmt::{Display, Formatter};
use std::time::Duration;
use surrealdb::types::Datetime;

const OPPONENT_ACCEPT_TIMEOUT: i64 = 60;

/// Performs a coin flip with a specified amount to bet and an optional user to bet against
///
/// To bet $100 you can use /coinflip 100
/// This will bet $100 against the bot
///
/// You can also specify which user to bet against, for example you can bet $20 against a user with
/// /coinflip 20 <user>
#[poise::command(slash_command, category = "games", rename = "coinflip")]
pub(super) async fn coin_flip(
    ctx: CommandContext<'_>,
    #[description = "The amount to bet in the coin flip"]
    #[min = 0]
    #[max = 10_000]
    amount: u64,
    #[description = "The target user to bet against"] opponent: Option<User>,
) -> serenity::Result<(), DigCommandError> {
    let mut command_user_data = UserData::get_user(ctx).await?;

    if command_user_data.balance < amount {
        ctx.send(default_reply_msg(format!(
            "You don't have enough balance to cover this! Your balance is: {}",
            command_user_data.balance
        )))
        .await?;

        return Ok(());
    }

    if let Some(opp) = opponent {
        if &opp == ctx.author() {
            ctx.send(default_reply_msg(
                "C'mon man you can't bet against yourself :(\nI almost didn't catch this bug lol",
            ))
            .await?;

            return Ok(());
        }

        let cmd_user_mention = ctx.author().mention();
        let opp_mention = opp.mention();

        let secs_now = Datetime::now().timestamp();

        let m = ctx
            .send(
                CreateReply::default()
                    .content(opp_mention.to_string())
                    .embed(default_embed().title("Coin Flip").description(format!(
                        "\
                        {cmd_user_mention} would like to coinflip ${amount} against {opp_mention}.\
                        They may accept before the timer ends ({})\
                    ",
                        CreateTime::new(secs_now + OPPONENT_ACCEPT_TIMEOUT)
                    ))),
            )
            .await?;

        let Some(mci) = m
            .message()
            .await?
            .await_component_interaction(&ctx.serenity_context().shard)
            .author_id(opp.id)
            .timeout(Duration::from_secs(OPPONENT_ACCEPT_TIMEOUT as u64))
            .await
        else {
            m.edit(
                ctx,
                default_reply_msg(format!("{opp_mention} did not respond in time")),
            )
            .await?;

            return Ok(());
        };

        ctx.send(default_reply_msg("opponent responded")).await?;
        // Err("Not yet supported!")?
    }

    let end_game_text = if ctx.data().rng_mut().random() {
        command_user_data.balance += amount;

        "You won the coin flip and just got $amt!\nYour balance is now $bal"
    } else {
        command_user_data.balance -= amount;

        "You lost the coin flip and $amt :(\nYour balance is now $bal"
    };

    ctx.send(default_reply_msg(
        end_game_text
            .replace("$amt", &format!("${}", &amount.to_string()))
            .replace(
                "$bal",
                &format!("${}", command_user_data.balance.to_string()),
            ),
    ))
    .await?;

    Ok(())
}

#[derive(poise::ChoiceParameter)]
enum CoinFlipResult {
    #[name = "heads"]
    Heads,
    #[name = "tails"]
    Tails,
}

impl Display for CoinFlipResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result_str = match self {
            Self::Heads => "heads",

            Self::Tails => "tails",
        };

        f.write_str(result_str)
    }
}
