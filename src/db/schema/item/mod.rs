pub(crate) mod generator;
pub(crate) mod inventory_item;
pub(crate) mod locations;
pub(crate) mod rarity;

use derive_enum_all_values::AllValues;
use poise::ChoiceParameter;
use std::fmt::{Display, Formatter};
use std::sync::LazyLock;
use surrealdb::types::SurrealValue;

static ITEM_IDS: LazyLock<Vec<Option<Item>>> = LazyLock::new(|| {
    let mut ids = vec![None; Item::all_values().len()];

    for item in Item::all_values() {
        ids[item.item_id() as usize] = Some(*item);
    }

    ids
});

pub(crate) const ITEM_TABLE: &str = "item";

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
        let mut string_buf = String::with_capacity(self.name().len());

        for (i, c) in self.name().chars().enumerate() {
            if c.is_ascii_uppercase() && i > 0 {
                string_buf.push(' ');
            }

            string_buf.push(c);
        }

        f.write_str(&string_buf)
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

    fn get_asset_name(&self) -> String {
        match self {
            Self::Garbage
            | Self::OldCoin
            | Self::Diamond
            | Self::Ruby
            | Self::BrokenTool
            | Self::MetalScraps => self.to_string().replace(' ', "_"),
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

    pub(super) fn item_id(&self) -> u16 {
        match self {
            Self::Garbage => 0,
            Self::OldCoin => 1,
            Self::Diamond => 2,
            Self::Ruby => 3,
            Self::BrokenTool => 4,
            Self::MetalScraps => 5,
        }
    }

    pub(crate) fn get_item_value(&self) -> u64 {
        match self {
            Self::Garbage => 3,
            Self::OldCoin => 8,
            Self::BrokenTool => 20,
            Self::Diamond => 85,
            Self::Ruby => 80,
            Self::MetalScraps => 5,
        }
    }

    pub(crate) fn from_item_id(id: u16) -> Option<Self> {
        ITEM_IDS.get(id as usize).copied().unwrap_or_default()
    }
}
