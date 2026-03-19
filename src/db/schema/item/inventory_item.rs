use crate::commands::{CommandContext, DigCommandError};
use crate::db::schema::item::generator::ITEM_GENERATOR;
use crate::db::schema::item::{Item, ItemValue, ITEM_TABLE, MAX_RARITY_ADDITIONAL_MUL, RARITY_POW};
use crate::db::schema::users::{UserData, USER_TABLE};
use rand::RngExt;
use surrealdb::types::{RecordId, SurrealValue};

#[derive(SurrealValue)]
pub(crate) struct InventoryItem {
    pub(crate) item_type: Item,
    /// Rarity value from 0 to 1
    /// The higher the rarity, the more the item is worth
    pub(crate) rarity: f64,
    pub(crate) owner: RecordId,
    pub(crate) id: Option<RecordId>,
}

impl ItemValue for InventoryItem {
    fn get_item_value(&self) -> u64 {
        let base_item_value = self.item_type.get_item_value();

        let rarity_mul = self.rarity * MAX_RARITY_ADDITIONAL_MUL;

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
                id: None,
            })
            .await?;

        Ok(item.unwrap())
    }
}
