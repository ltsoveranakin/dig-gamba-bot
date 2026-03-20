use crate::commands::{default_reply_msg, CommandContext, DigCommandError};
use crate::db::schema::users::UserData;
use rand::RngExt;
use std::fmt::{Display, Formatter};

/// Performs a coin flip with a specified amount to bet and an optional result to bet on
///
/// To bet $100 you can use /coinflip 100
/// This will bet $100 on a random side
///
/// You can also specify which side to bet on, for example you can bet $20 on tails with
/// /coinflip 20 tails
#[poise::command(slash_command, category = "games", rename = "coinflip")]
pub(super) async fn coin_flip(
    ctx: CommandContext<'_>,
    #[description = "The amount to bet in the coin flip"]
    #[min = 0]
    #[max = 10_000]
    amount: u64,
    #[description = "The result to bet on"] result_bet_on: Option<CoinFlipResult>,
) -> serenity::Result<(), DigCommandError> {
    let mut user = UserData::get_user(ctx).await?;

    if user.balance < amount {
        ctx.send(default_reply_msg(format!(
            "You don't have enough balance to cover this! Your balance is: {}",
            user.balance
        )))
        .await?;

        return Ok(());
    }

    let player_bet;
    let is_heads;

    {
        let mut rng = ctx.data().rng_mut();

        player_bet = result_bet_on.unwrap_or_else(|| {
            if rng.random() {
                CoinFlipResult::Heads
            } else {
                CoinFlipResult::Tails
            }
        });

        is_heads = rng.random();
    }

    ctx.send(default_reply_msg(format!(
        "You bet {amount} on {player_bet}"
    )))
    .await?;

    let end_game_text = if is_heads && matches!(player_bet, CoinFlipResult::Heads) {
        user.balance += amount;

        "You won the coin flip and just got $amt!\nYour balance is now $bal"
    } else {
        user.balance -= amount;

        "You lost the coin flip and $amt :(\nYour balance is now $bal"
    };

    ctx.send(default_reply_msg(
        end_game_text
            .replace("$amt", &format!("${}", &amount.to_string()))
            .replace("$bal", &format!("${}", user.balance.to_string())),
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
