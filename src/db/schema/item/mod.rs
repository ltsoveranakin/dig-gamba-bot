pub(crate) mod generator;
pub(crate) mod rarity;
pub(crate) mod schema;

use std::fmt::{Display, Formatter};
use surrealdb::types::SurrealValue;

const RARITY_POW: f64 = 2.0;
const MAX_RARITY_ADDITIONAL_MUL: f64 = 100.0;
pub(crate) const ITEM_TABLE: &str = "item";

pub(crate) trait ItemValue {
    fn get_item_value(&self) -> u64;
}

#[derive(SurrealValue, Copy, Clone)]
pub(crate) enum Item {
    Garbage,
    OldCoin,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Garbage => "Garbage",
            Self::OldCoin => "Old Coin",
        };

        f.write_str(s)
    }
}

impl ItemValue for Item {
    fn get_item_value(&self) -> u64 {
        match self {
            Self::Garbage => 5,
            Self::OldCoin => 8,
        }
    }
}
