use crate::commands::{default_reply_msg, CommandContext, DigCommandError};
use crate::db::schema::users::UserData;
use rand::RngExt;
use std::fmt::{Display, Formatter};

const MAX_COIN_FLIP_AMOUNT: u64 = 10_000;

#[derive(poise::ChoiceParameter)]
enum CoinFlipResult {
    Heads,
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

#[poise::command(slash_command, category = "games")]
pub(super) async fn coin_flip(
    ctx: CommandContext<'_>,
    #[description = "The amount to bet in the coin flip"] amount: u64,
    #[description = "The result to bet on"] result_bet_on: Option<CoinFlipResult>,
) -> serenity::Result<(), DigCommandError> {
    if amount == 0 {
        ctx.send(default_reply_msg("You can't bet $0!")).await?;

        return Ok(());
    }

    if amount > MAX_COIN_FLIP_AMOUNT {
        ctx.send(default_reply_msg(format!(
            "The maximum amount you can bet is: {}",
            MAX_COIN_FLIP_AMOUNT
        )))
        .await?;

        return Ok(());
    }

    let mut user = UserData::get_user(&ctx).await?;

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
