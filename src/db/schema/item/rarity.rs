use serenity::all::Color;
use std::fmt::{Display, Formatter};

const COMMON_BOUNDS_UPPER: f64 = 0.4;
const UNCOMMON_BOUNDS_UPPER: f64 = 0.65;
const RARE_BOUNDS_UPPER: f64 = 0.8;
const EPIC_BOUNDS_UPPER: f64 = 0.95;
const LEGENDARY_BOUNDS_UPPER: f64 = 0.99;

#[derive(Copy, Clone)]
pub(crate) enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Perfect,
}

impl Rarity {
    pub(crate) fn get_rarity_color(&self) -> Color {
        match self {
            Self::Common => Color::from_rgb(102, 102, 102),
            Self::Uncommon => Color::from_rgb(98, 117, 48),
            Self::Rare => Color::from_rgb(6, 49, 71),
            Self::Epic => Color::from_rgb(134, 20, 168),
            Self::Legendary => Color::from_rgb(219, 149, 29),
            Self::Perfect => Color::from_rgb(0, 255, 200),
        }
    }

    pub(crate) fn from_float(value: f64) -> Self {
        match value {
            0.0..COMMON_BOUNDS_UPPER => Self::Common,
            COMMON_BOUNDS_UPPER..UNCOMMON_BOUNDS_UPPER => Self::Uncommon,
            UNCOMMON_BOUNDS_UPPER..RARE_BOUNDS_UPPER => Self::Rare,
            RARE_BOUNDS_UPPER..EPIC_BOUNDS_UPPER => Self::Epic,
            EPIC_BOUNDS_UPPER..LEGENDARY_BOUNDS_UPPER => Self::Legendary,

            _ => Self::Perfect,
        }
    }

    pub(crate) fn starts_with_vowel(&self) -> bool {
        match self {
            Self::Uncommon | Self::Epic => true,
            Self::Common | Self::Rare | Self::Legendary | Self::Perfect => false,
        }
    }
}

impl Display for Rarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
            Self::Perfect => "Perfect",
        };

        f.write_str(s)
    }
}
