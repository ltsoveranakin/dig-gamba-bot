use poise::ChoiceParameter;
use std::collections::HashMap;
use std::sync::OnceLock;

struct ChannelBasedEnum<E>
where
    E: 'static,
{
    all_values: &'static [E],
    category: &'static str,
    inner: OnceLock<HashMap<String, E>>,
}

impl<E> ChannelBasedEnum<E>
where
    E: ChoiceParameter + Copy,
{
    fn new(all_values: &'static [E], category: &'static str) -> Self {
        Self {
            all_values,
            category,
            inner: OnceLock::new(),
        }
    }

    fn get_map(&self) -> &HashMap<String, E> {
        self.inner.get_or_init(|| {
            self.all_values
                .iter()
                .map(|e| (e.name().to_lowercase(), *e))
                .collect()
        })
    }

    fn get_by_channel_name(&self, name: &str) -> Option<E> {
        self.get_map().get(name).copied()
    }
}
