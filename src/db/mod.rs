use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;

pub(crate) mod schema;

pub(super) async fn setup_db() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<SurrealKv>("data/dig_bot.db").await?;

    db.use_ns("dig_bot").use_db("slash_dig").await?;
    db.query(
        vec![
            "DEFINE TABLE user SCHEMAFULL;",
            "DEFINE FIELD balance ON TABLE user TYPE int;",
            "DEFINE TABLE item SCHEMAFULL;",
            "DEFINE FIELD item_type ON TABLE item TYPE int;",
            "DEFINE FIELD rarity ON TABLE item TYPE float;",
            "DEFINE FIELD owner ON TABLE item TYPE record;",
            "REMOVE FIELD ldt ON TABLE item;",
            "DEFINE TABLE last_dug SCHEMAFULL;",
            "DEFINE FIELD ldt ON TABLE last_dug TYPE datetime;",
        ]
        .join("\n"),
    )
    .await?;

    // def("user", vec![("balance", FieldTy::Int)]),
    // def(
    //     "item",
    //     vec![
    //         ("item_type", FieldTy::Obj),
    //         ("rarity", FieldTy::Float),
    //         ("owner", FieldTy::Id),
    //     ],
    // ),
    // def("last_dug", vec![("ldt", FieldTy::Date)]),

    Ok(db)
}
