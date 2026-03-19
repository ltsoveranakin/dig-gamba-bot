use crate::commands::{CommandContext, DigCommandError};
use crate::db::schema::item::locations::beach::BEACH_DROP_POOL;
use crate::db::schema::item::Item;
use derive_enum_all_values::AllValues;
use rand::distr::uniform::SampleUniform;
use rand::distr::weighted::Weight;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

mod beach;

static DIGGING_LOCATIONS: LazyLock<HashMap<&'static str, DiggingLocation>> = LazyLock::new(|| {
    DiggingLocation::all_values()
        .iter()
        .map(|digging_channel| (digging_channel.get_channel_name(), *digging_channel))
        .collect()
});

pub(super) type ItemDropArray = [(Item, u32)];

#[derive(AllValues, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum DiggingLocation {
    Beach,
}

impl DiggingLocation {
    pub(crate) fn get_channel_name(&self) -> &'static str {
        match self {
            Self::Beach => "beach",
        }
    }

    pub(super) fn get_drop_pool(&self) -> &ItemDropArray {
        match self {
            Self::Beach => &BEACH_DROP_POOL,
        }
    }

    pub(super) async fn get_location_from_channel(
        ctx: &CommandContext<'_>,
    ) -> Result<DiggingLocation, DigCommandError> {
        let channel = ctx.guild_channel().await.ok_or("Must be in a server")?;

        let location = DIGGING_LOCATIONS
            .get(&*channel.name)
            .ok_or_else(|| {
                let advice_text = if let Some(beach_channel_id) = ctx.guild().expect("Checked if in server").channels.values().find_map(|channel| {
                    (channel.name == Self::Beach.get_channel_name()).then(|| {
                        channel.id
                    })
                }) {
                    format!("Try the <#{beach_channel_id}>, I heard it's a good place to start.")
                } else {
                    "I would recommend the beach to dig but I can't find the channel!\nHave an admin run `/setup`".to_string()
                };

                format!("You can't dig here!\n{advice_text}")
            })?;

        Ok(*location)
    }
}
