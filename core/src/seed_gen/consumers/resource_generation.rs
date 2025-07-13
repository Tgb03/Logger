use serde::{Deserialize, Serialize};

use crate::seed_gen::{consumers::{base_consumer::Consumer, OutputSeedIndexer}, ConsumerOutput};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum ResourceType {
    #[default]
    Healthpack,
    DisinfectPack,
    Ammopack,
    ToolRefillpack,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceGeneration {
    left: f32,
    res_type: ResourceType,

    #[serde(skip_serializing, default)]
    seed_index: usize,
    #[serde(skip_serializing, default)]
    is_setup: bool,
    #[serde(skip_serializing, default)]
    counter: i32,
}

impl ResourceGeneration {
    fn setup(&mut self) {
        if self.is_setup {
            return;
        }

        self.left = match self.res_type {
            ResourceType::Ammopack => self.left * 0.8f32,
            ResourceType::ToolRefillpack => self.left * 0.7f32,
            _ => self.left,
        };

        self.is_setup = true;
    }

    fn try_remove(&mut self, value: f32) {
        if self.left <= value {
            self.left = 0f32;
        } else {
            self.left -= value;
        }
    }

    pub fn new(left: f32, res_type: ResourceType) -> Self {
        Self {
            left: left,
            res_type: res_type,
            ..Default::default()
        }
    }
}

impl<O> Consumer<O> for ResourceGeneration
where
    O: ConsumerOutput<Output = OutputSeedIndexer>,
{
    fn take(&mut self, seed: f32, _: &mut O) -> bool {
        self.setup();
        // println!("left: {}", self.left);
        
        self.seed_index += 1;

        if self.seed_index == 2 {
            if seed < 0.333333f32 {
                self.try_remove(0.6f32)
            } else if seed < 0.6666666f32 {
                self.try_remove(1.0f32)
            } else {
                self.try_remove(0.4f32)
            }
        }

        if self.seed_index == 3 && self.left <= 0.2f32 {
            return true;
        }

        if self.seed_index == 3 {
            self.seed_index = 0;
            self.counter += 1;
        }

        false
    }
}
