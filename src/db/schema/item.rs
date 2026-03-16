use serde::{Deserialize, Serialize};

const MAX_RARITY_ADDITIONAL_VALUE: f64 = 1000.;

pub(crate) trait ItemValue {
    fn get_item_value(&self) -> u64;
}

#[derive(Serialize, Deserialize)]
enum Item {
    Garbage,
}

impl ItemValue for Item {
    fn get_item_value(&self) -> u64 {
        match self {
            Self::Garbage => 10,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct InventoryItem {
    item_type: Item,
    /// Rarity value from 0 to 1
    /// The higher the rarity, the more the item is worth
    rarity: f64,
}

impl ItemValue for InventoryItem {
    fn get_item_value(&self) -> u64 {
        let base_item_value = self.item_type.get_item_value();

        let rarity_mul = self.rarity * MAX_RARITY_ADDITIONAL_VALUE;

        let item_value = base_item_value + (base_item_value * (rarity_mul as u64));

        item_value
    }
}
