use rand::make_rng;
use rand::prelude::SmallRng;
use std::sync::{Mutex, MutexGuard};
use surrealdb::Surreal;

pub(crate) struct Data {
    pub(crate) db: Surreal<surrealdb::engine::local::Db>,
    rng: Mutex<SmallRng>,
}

impl Data {
    pub(super) fn new(db: Surreal<surrealdb::engine::local::Db>) -> Self {
        Self {
            db,
            rng: Mutex::new(make_rng()),
        }
    }

    pub(crate) fn rng_mut(&self) -> MutexGuard<'_, SmallRng> {
        self.rng.lock().expect("Data rng mutex poisoned")
    }
}
