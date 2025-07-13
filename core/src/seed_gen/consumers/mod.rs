use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::seed_gen::{consumers::{
        base_consumer::Consumer, consumable_consumer::ConsumableConsumer, ignore_consumer::IgnoreConsumer, key_consumer::KeyConsumer, key_id_consumer::KeyIDConsumer, objective_consumer::ObjectiveConsumer, output_seed::OutputSeed, resource_generation::{ResourceGeneration, ResourceType}, zone_consumer::ZoneConsumer
    }, ConsumerOutput};

pub mod base_consumer;

pub mod ignore_consumer;
pub mod key_consumer;
pub mod key_id_consumer;
pub mod output_seed;
pub mod resource_generation;
pub mod zone_consumer;
pub mod objective_consumer;
pub mod consumable_consumer;

#[enum_dispatch(Consumer)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ConsumerEnum {
    Ignore(IgnoreConsumer),
    KeyIDConsumer(KeyIDConsumer),
    OutputSeed(OutputSeed),
    ResourceGeneration(ResourceGeneration),
    KeyConsumer(KeyConsumer),
    ZoneConsumer(ZoneConsumer),
    ObjectiveConsumer(ObjectiveConsumer),
    ConsumableConsumer(ConsumableConsumer),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputSeedIndexer {
    Seed(f32),
    Key(String, i32, i32),           // zone, id
    ResourcePack(ResourceType, i32), // count
    ConsumableFound(i32, bool),      // id of box, found or not
    GenerationEnd,
    GenerationStart,
    ZoneGenEnded(u32),
}

impl<O> Consumer<O> for ConsumerEnum
where
    O: ConsumerOutput<Output = OutputSeedIndexer>,
{
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        match self {
            ConsumerEnum::Ignore(ignore_consumer) => ignore_consumer.take(seed, output),
            ConsumerEnum::KeyIDConsumer(key_idconsumer) => key_idconsumer.take(seed, output),
            ConsumerEnum::OutputSeed(output_seed) => output_seed.take(seed, output),
            ConsumerEnum::ResourceGeneration(resource_generation) => {
                                        resource_generation.take(seed, output)
                                    }
            ConsumerEnum::KeyConsumer(key_eater) => key_eater.take(seed, output),
            ConsumerEnum::ZoneConsumer(zone_consumer) => zone_consumer.take(seed, output),
            ConsumerEnum::ObjectiveConsumer(objective_consumer) => objective_consumer.take(seed, output),
            ConsumerEnum::ConsumableConsumer(consumable_consumer) => consumable_consumer.take(seed, output),
        }
    }
}
