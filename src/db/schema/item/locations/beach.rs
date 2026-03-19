use crate::db::schema::item::Item;

pub(super) const BEACH_DROP_POOL: [(Item, u32); 6] = [
    (Item::Garbage, 10),
    (Item::OldCoin, 8),
    (Item::BrokenTool, 5),
    (Item::Diamond, 1),
    (Item::Ruby, 1),
    (Item::MetalScraps, 8),
];
