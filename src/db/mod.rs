use std::fmt::{Display, Formatter};
use surrealdb::engine::local::{Db, SurrealKv};
use surrealdb::Surreal;

pub(crate) mod schema;

pub(super) async fn setup_db() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<SurrealKv>("data/dig_bot.db").await?;

    db.use_ns("dig_bot").use_db("slash_dig").await?;
    let query_str = vec![
        def("user", vec![("balance", FieldTy::Int)]),
        def(
            "item",
            vec![
                ("item_type", FieldTy::Int),
                ("rarity", FieldTy::Float),
                ("owner", FieldTy::RecordId),
            ],
        ),
        def("last_dug", vec![("ldt", FieldTy::Date)]),
    ]
    .join("\n");

    db.query(query_str).await?;

    // vec![
    //     "DEFINE TABLE user SCHEMAFULL;",
    //     "DEFINE FIELD balance ON TABLE user TYPE int;",
    //     "DEFINE TABLE item SCHEMAFULL;",
    //     "DEFINE FIELD item_type ON TABLE item TYPE int;",
    //     "DEFINE FIELD rarity ON TABLE item TYPE float;",
    //     "DEFINE FIELD owner ON TABLE item TYPE record;",
    //     "REMOVE FIELD ldt ON TABLE item;",
    //     "DEFINE TABLE last_dug SCHEMAFULL;",
    //     "DEFINE FIELD ldt ON TABLE last_dug TYPE datetime;",
    // ]
    //     .join("\n"),

    Ok(db)
}

enum FieldTy {
    Int,
    Float,
    RecordId,
    Date,
}

impl Display for FieldTy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Int => "int",
            Self::Float => "float",
            Self::RecordId => "record",
            Self::Date => "datetime",
        };

        f.write_str(s)
    }
}

fn def(table_name: &str, fields: Vec<(&str, FieldTy)>) -> String {
    let mut v = Vec::with_capacity(fields.len() + 1);
    v.push(format!("DEFINE TABLE {table_name} SCHEMAFULL;"));

    v.extend(fields.into_iter().map(|(field_name, field_ty)| {
        format!("DEFINE FIELD {field_name} ON TABLE {table_name} TYPE {field_ty};")
    }));

    v.join("\n")
}
