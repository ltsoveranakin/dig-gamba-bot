use crate::commands::{CommandContext, DigCommandError};
use crate::db::schema::item::locations::DiggingLocation;
use crate::db::schema::item::{Item, ITEM_TABLE};
use crate::db::schema::users::{UserData, USER_TABLE};
use rand::RngExt;
use surrealdb::types::{RecordId, SurrealValue};

const RARITY_POW: f64 = 2.5;
const MAX_RARITY_ADDITIONAL_MUL: f64 = 10.0;

#[derive(SurrealValue)]
pub(crate) struct InventoryItem {
    pub(crate) item_type: u16,
    /// Rarity value from 0 to 1
    /// The higher the rarity, the more the item is worth
    pub(crate) rarity: f64,
    pub(crate) owner: RecordId,
    pub(crate) id: Option<RecordId>,
}

impl InventoryItem {
    pub(crate) async fn create_new(
        ctx: CommandContext<'_>,
        digging_location: DiggingLocation,
    ) -> Result<Self, DigCommandError> {
        UserData::get_user(ctx).await?;

        let item_generator = ctx.data().digging_locations.get(&digging_location).ok_or("No clue how this error happened but it did, congrats!\nSomehow this digging location isn't registered for the item generator even tho it is for a valid digging location.")?;

        let item_type = item_generator.random_item();

        let rarity_lin: f64 = item_generator.rng_mut().random_range(0.0..1.0);
        let rarity = rarity_lin.powf(RARITY_POW);

        let owner = RecordId::new(USER_TABLE, ctx.author().id.get().to_string());

        let item = ctx
            .data()
            .db
            .create(ITEM_TABLE)
            .content(InventoryItem {
                item_type: item_type.item_id(),
                rarity,
                owner,
                id: None,
            })
            .await?;

        Ok(item.unwrap())
    }

   pub(crate) fn calculate_item_value(&self) -> Option<u64> {
        let item = Item::from_item_id(self.item_type)?;
        let base_item_value = item.get_item_value();

        let rarity_mul = self.rarity * MAX_RARITY_ADDITIONAL_MUL;

        let item_value = base_item_value + (base_item_value * (rarity_mul as u64));

        Some(item_value)
    }
}
