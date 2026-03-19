use crate::db::schema::item::generator::ItemGenerator;
use crate::db::schema::item::locations::DiggingLocation;
use rand::make_rng;
use rand::prelude::SmallRng;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use surrealdb::Surreal;

pub(crate) struct Data {
    pub(crate) db: Surreal<surrealdb::engine::local::Db>,
    rng: Mutex<SmallRng>,
    pub(crate) digging_locations: HashMap<DiggingLocation, ItemGenerator>,
}

impl Data {
    pub(super) fn new(db: Surreal<surrealdb::engine::local::Db>) -> Self {
        let digging_locations = DiggingLocation::all_values().iter().map(|&location| {
            let item_generator = ItemGenerator::new(location);

            (location, item_generator)
        });

        Self {
            db,
            rng: Mutex::new(make_rng()),
            digging_locations: digging_locations.collect(),
        }
    }

    pub(crate) fn rng_mut(&self) -> MutexGuard<'_, SmallRng> {
        self.rng.lock().expect("Data rng mutex poisoned")
    }
}
