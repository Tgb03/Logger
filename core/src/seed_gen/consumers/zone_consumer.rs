use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::seed_gen::{consumers::{
        base_consumer::Consumer, 
        ignore_consumer::IgnoreConsumer, 
        resource_generation::{
            ResourceGeneration, 
            ResourceType
        }, 
        ConsumerEnum, OutputSeedIndexer
    }, ConsumerOutput};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConsumer {

    zone_index: u8,
    shown_number: u32,

    medi: f32,
    disi: f32,
    ammo: f32,
    tool: f32,

    artifact_count: u32,
    consumable_in_container: u32,
    consumable_in_worldspawn: u32,
    
    #[serde(skip_serializing, default)]
    is_setup: bool,

    #[serde(skip_serializing, default)]
    consumers: VecDeque<ConsumerEnum>,

}

impl ZoneConsumer {

    pub fn setup(&mut self) {
        if self.is_setup { return }

        if self.medi != 0.0 { 
            self.consumers.push_back(ConsumerEnum::ResourceGeneration(ResourceGeneration::new(self.medi, ResourceType::Healthpack)));
        }

        if self.disi != 0.0 {
            self.consumers.push_back(ConsumerEnum::ResourceGeneration(ResourceGeneration::new(self.disi, ResourceType::DisinfectPack)));
        }

        if self.ammo != 0.0 {
            self.consumers.push_back(ConsumerEnum::ResourceGeneration(ResourceGeneration::new(self.ammo, ResourceType::Ammopack)));
        }
        
        if self.tool != 0.0 {
            self.consumers.push_back(ConsumerEnum::ResourceGeneration(ResourceGeneration::new(self.tool, ResourceType::ToolRefillpack)));
        }

        let extra = (self.artifact_count + self.consumable_in_container * 2 + self.consumable_in_worldspawn) as usize;
        if extra != 0 {
            self.consumers.push_back(ConsumerEnum::Ignore(IgnoreConsumer::new(
                extra
            )));
        }

        self.is_setup = true;
    }

}

impl<O> Consumer<O> for ZoneConsumer
where
    O: ConsumerOutput<Output = OutputSeedIndexer>, {
    
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        self.setup();

        let result = self.consumers.take(seed, output);

        result
    }
}