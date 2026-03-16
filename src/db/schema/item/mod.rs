pub(crate) mod rarity;

use crate::commands::{CommandContext, DigCommandError};
use crate::db::schema::users::{UserData, USER_TABLE};
use rand::distr::weighted::WeightedIndex;
use rand::prelude::{Distribution, SmallRng};
use rand::{make_rng, RngExt};
use std::fmt::{Display, Formatter};
use std::sync::{LazyLock, RwLock, RwLockWriteGuard};
use surrealdb::types::{RecordId, SurrealValue};

pub(crate) static ITEM_GENERATOR: LazyLock<ItemGenerator> = LazyLock::new(|| ItemGenerator::new());

const RARITY_POW: f64 = 2.0;
const MAX_RARITY_ADDITIONAL_VALUE: f64 = 1000.0;
const ITEM_TABLE: &str = "item";

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
            Self::Garbage => 10,
            Self::OldCoin => 100,
        }
    }
}

#[derive(SurrealValue)]
pub(crate) struct InventoryItem {
    pub(crate) item_type: Item,
    /// Rarity value from 0 to 1
    /// The higher the rarity, the more the item is worth
    pub(crate) rarity: f64,
    pub(crate) owner: RecordId,
}

impl ItemValue for InventoryItem {
    fn get_item_value(&self) -> u64 {
        let base_item_value = self.item_type.get_item_value();

        let rarity_mul = self.rarity * MAX_RARITY_ADDITIONAL_VALUE;

        let item_value = base_item_value + (base_item_value * (rarity_mul as u64));

        item_value
    }
}

impl InventoryItem {
    pub(crate) async fn create_new(ctx: &CommandContext<'_>) -> Result<Self, DigCommandError> {
        UserData::get_user(&ctx).await?;

        let item_type = ITEM_GENERATOR.random_item();

        let rarity_lin: f64 = ITEM_GENERATOR.rng_mut().random_range(0.0..1.0);
        let rarity = rarity_lin.powf(RARITY_POW);

        let owner = RecordId::new(USER_TABLE, ctx.author().id.get().to_string());

        let item = ctx
            .data()
            .db
            .create(ITEM_TABLE)
            .content(InventoryItem {
                item_type,
                rarity,
                owner,
            })
            .await?;

        Ok(item.unwrap())
    }
}

pub(crate) struct ItemGenerator {
    rng: RwLock<SmallRng>,
    distribution: WeightedIndex<usize>,
    items: Vec<Item>,
}

impl ItemGenerator {
    fn new() -> Self {
        let drop_table = [(Item::Garbage, 30), (Item::OldCoin, 10)];

        let mut items = Vec::with_capacity(drop_table.len());
        let mut weights = Vec::with_capacity(drop_table.len());

        for (item, weight) in drop_table {
            items.push(item);
            weights.push(weight);
        }

        Self {
            distribution: WeightedIndex::new(weights).expect("Invalid weights for item drops"),
            items,
            rng: RwLock::new(make_rng()),
        }
    }

    fn random_item(&self) -> Item {
        let item_index = self.distribution.sample(&mut self.rng_mut());

        self.items[item_index]
    }

    fn rng_mut(&self) -> RwLockWriteGuard<'_, SmallRng> {
        self.rng.write().expect("Write RNG random item gen")
    }
}
