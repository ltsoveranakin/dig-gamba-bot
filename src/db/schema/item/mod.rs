pub(crate) mod generator;
pub(crate) mod inventory_item;
pub(crate) mod locations;
pub(crate) mod rarity;

use derive_enum_all_values::AllValues;
use std::fmt::{Display, Formatter};
use surrealdb::types::SurrealValue;

const RARITY_POW: f64 = 2.5;
const MAX_RARITY_ADDITIONAL_MUL: f64 = 10.0;
pub(crate) const ITEM_TABLE: &str = "item";

pub(crate) trait ItemValue {
    fn get_item_value(&self) -> u64;
}

#[derive(SurrealValue, poise::ChoiceParameter, AllValues, Copy, Clone)]
pub(crate) enum Item {
    Garbage,
    OldCoin,
    BrokenTool,
    Diamond,
    Ruby,
    MetalScraps,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Garbage => "Garbage",
            Self::OldCoin => "Old Coin",
            Self::BrokenTool => "Broken Tool",
            Self::Diamond => "Diamond",
            Self::Ruby => "Ruby",
            Self::MetalScraps => "Metal Scraps",
        };

        f.write_str(s)
    }
}

impl ItemValue for Item {
    fn get_item_value(&self) -> u64 {
        match self {
            Self::Garbage => 3,
            Self::OldCoin => 8,
            Self::BrokenTool => 20,
            Self::Diamond => 85,
            Self::Ruby => 80,
            Self::MetalScraps => 5,
        }
    }
}

impl Item {
    pub(crate) fn get_description(&self) -> &str {
        match self {
            Self::Garbage => "Some garbage left behind",
            Self::OldCoin => "An old coin, dulled from its age",
            Self::BrokenTool => "A once working tool, now left in pieces",
            Self::Diamond => "A shiny diamond",
            Self::Ruby => "A beautifully vibrant red ruby",
            Self::MetalScraps => "Some metal scraps, a bit more useful than garbage at least",
        }
    }

    fn get_asset_name(&self) -> &str {
        match self {
            Self::Garbage => "garbage",
            Self::MetalScraps | Self::Ruby | Self::Diamond | Self::BrokenTool | Self::OldCoin => {
                "missing_texture"
            }
        }
    }

    pub(crate) fn is_multiple(&self) -> bool {
        match self {
            Self::Garbage | Self::MetalScraps => true,
            Self::Ruby | Self::Diamond | Self::BrokenTool | Self::OldCoin => false,
        }
    }

    pub(crate) fn get_asset_file_name(&self) -> String {
        format!("{}.png", self.get_asset_name())
    }
}
