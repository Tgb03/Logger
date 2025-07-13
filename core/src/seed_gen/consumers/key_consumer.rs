
use serde::{Deserialize, Serialize};

use crate::seed_gen::{consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer, OutputSeedIndexer}, ConsumerOutput};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum KeyType {
    #[default]
    ColoredKey,
    BulkheadKey,
    Other
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KeyConsumer {
    key_type: KeyType,
    zones: Vec<KeyIDConsumer>,

    #[serde(skip_serializing, default)]
    seed_index: usize,
    #[serde(skip_serializing, default)]
    zone: usize,
}

impl KeyConsumer {
    pub fn get_first_id(&self) -> usize {
        match self.key_type {
            KeyType::ColoredKey => 3,
            KeyType::BulkheadKey => 2,
            KeyType::Other => 1,
        }
    }

    pub fn get_second_id(&self) -> usize {
        self.get_first_id() + 1
    }
}

impl<O> Consumer<O> for KeyConsumer
where
    O: ConsumerOutput<Output = OutputSeedIndexer>,
{
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        self.seed_index += 1;

        if self.seed_index == self.get_first_id() {
            self.zone = (seed * self.zones.len() as f32) as usize;
        }

        if self.seed_index == self.get_second_id() {
            return self.zones[self.zone].take(seed, output);
        }

        false
    }
}
