use crate::db::schema::item::Item;
use rand::distr::weighted::WeightedIndex;
use rand::make_rng;
use rand::prelude::{Distribution, SmallRng};
use std::sync::{LazyLock, RwLock, RwLockWriteGuard};

pub(crate) static ITEM_GENERATOR: LazyLock<ItemGenerator> = LazyLock::new(|| ItemGenerator::new());

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

    pub(super) fn random_item(&self) -> Item {
        let item_index = self.distribution.sample(&mut self.rng_mut());

        self.items[item_index]
    }

    pub(super) fn rng_mut(&self) -> RwLockWriteGuard<'_, SmallRng> {
        self.rng.write().expect("Write RNG random item gen")
    }
}
