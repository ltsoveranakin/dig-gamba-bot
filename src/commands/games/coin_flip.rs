use crate::commands::{
    default_embed, default_reply_msg, CommandContext, CreateTime, DigCommandError,
};
use crate::db::schema::users::UserData;
use poise::CreateReply;
use rand::RngExt;
use serenity::all::{CreateActionRow, CreateButton, Mentionable, User};
use std::time::Duration;
use surrealdb::types::Datetime;
use tokio::time::sleep;

const OPPONENT_ACCEPT_TIMEOUT: i64 = 60;
const ACCEPT_ID: &str = "a";
const DENY_ID: &str = "d";

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

    if let Some(opponent) = opponent {
        if &opponent == ctx.author() {
            ctx.send(default_reply_msg(
                "C'mon man you can't bet against yourself :(\nI almost didn't catch this bug lol",
            ))
            .await?;

            return Ok(());
        }

        let opponent_user_data = UserData::get_user_by_id(ctx, opponent.id).await?;

        let opponent_mention = opponent.mention();

        if opponent_user_data.balance < amount {
            ctx.send(default_reply_msg(format!(
                "{} doesn't have enough balance to cover the coin flip!",
                opponent_mention
            )))
                .await?;
            return Ok(());
        }

        let cmd_user_mention = ctx.author().mention();


        let secs_now = Datetime::now().timestamp();

        let m = ctx
            .send(
                CreateReply::default()
                    .content(opponent_mention.to_string())
                    .embed(default_embed().title("Coin Flip").description(format!(
                        "\
                        {cmd_user_mention} would like to coin flip ${amount} against {opponent_mention}.\
                        They may accept before the timer ends ({})\
                    ",
                        CreateTime::new(secs_now + OPPONENT_ACCEPT_TIMEOUT)
                    ))).components(vec![CreateActionRow::Buttons(vec![CreateButton::new(ACCEPT_ID).label("Accept"), CreateButton::new(DENY_ID).label("Deny")])]),
            )
            .await?;

        let Some(mci) = m
            .message()
            .await?
            .await_component_interaction(&ctx.serenity_context().shard)
            .author_id(opponent.id)
            .timeout(Duration::from_secs(OPPONENT_ACCEPT_TIMEOUT as u64))
            .await
        else {
            m.edit(
                ctx,
                default_reply_msg(format!("{opponent_mention} did not respond in time")),
            )
            .await?;

            return Ok(());
        };

        let accepted = mci.data.custom_id == ACCEPT_ID;

        if !accepted {
            ctx.send(default_reply_msg(format!("{opponent_mention} denied the coin flip"))).await?;
            return Ok(());
        }

        let flip_message = ctx.send(default_reply_msg("Flipping!")).await?;

        sleep(Duration::from_millis(500)).await;

        let ((winner, mut winner_user_data), (loser,mut loser_user_data)) = if ctx.data().rng_mut().random() {
            ((ctx.author(), command_user_data), (&opponent, opponent_user_data))
        } else {
            ((&opponent, opponent_user_data), (ctx.author(), command_user_data))
        };

        winner_user_data.balance += amount;
        loser_user_data.balance -= amount;

        let winner_mention = winner.mention();
        let loser_mention = loser.mention();

        let winner_user_data = winner_user_data.update_user(ctx).await?;
        let loser_user_data = loser_user_data.update_user(ctx).await?;

        flip_message.edit(ctx, default_reply_msg(format!("{winner_mention} won the coin flip!!\n{winner_mention}'s new balance is: {}\n{loser_mention}'s balance is {}", winner_user_data.balance, loser_user_data.balance,))).await?;

        return Ok(());
    }

    let end_game_text = if ctx.data().rng_mut().random() {
        command_user_data.balance += amount;

        "You won the coin flip and just got $amt!\nYour balance is now $bal"
    } else {
        command_user_data.balance -= amount;

        "You lost the coin flip and $amt :(\nYour balance is now $bal"
    };

    let command_user_data = command_user_data.update_user(ctx).await?;

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

