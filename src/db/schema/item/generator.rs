use crate::db::schema::item::locations::DiggingLocation;
use crate::db::schema::item::Item;
use rand::distr::weighted::WeightedIndex;
use rand::make_rng;
use rand::prelude::{Distribution, SmallRng};
use std::sync::{RwLock, RwLockWriteGuard};

pub(crate) struct GeneratorItem {
    pub(crate) item: Item,
    pub(crate) weight: u32,
}

pub(crate) struct ItemGenerator {
    rng: RwLock<SmallRng>,
    pub(crate) items: Vec<GeneratorItem>,
    distribution: WeightedIndex<u32>,
}

impl ItemGenerator {
    pub(crate) fn new(digging_location: DiggingLocation) -> Self {
        let drop_pool = digging_location.get_drop_pool();

        let mut items = Vec::with_capacity(drop_pool.len());
        let mut weights = Vec::with_capacity(drop_pool.len());

        for (item, drop_weight) in drop_pool {
            let drop_weight = *drop_weight;
            debug_assert!(drop_weight > 0);

            items.push(GeneratorItem {
                item: *item,
                weight: drop_weight,
            });
            weights.push(drop_weight);
        }

        Self {
            rng: RwLock::new(make_rng()),
            items,
            distribution: WeightedIndex::new(weights).expect("Invalid weights for item drops"),
        }
    }

    pub(super) fn random_item(&self) -> Item {
        let item_index = self.distribution.sample(&mut self.rng_mut());

        self.items[item_index].item
    }

    pub(super) fn rng_mut(&self) -> RwLockWriteGuard<'_, SmallRng> {
        self.rng.write().expect("Write RNG random item gen")
    }
}
