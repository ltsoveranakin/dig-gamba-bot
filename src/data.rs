use rand::prelude::SmallRng;
use std::sync::{Mutex, MutexGuard};
use surrealdb::Surreal;

pub(crate) struct Data {
    pub(crate) db: Surreal<surrealdb::engine::local::Db>,
    pub(super) rng: Mutex<SmallRng>,
}

impl Data {
    pub(crate) fn rng_mut(&self) -> MutexGuard<'_, SmallRng> {
        self.rng.lock().expect("Data rng mutex poisoned")
    }
}
