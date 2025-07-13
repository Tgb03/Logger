use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::seed_gen::{consumers::{base_consumer::Consumer, OutputSeedIndexer}, ConsumerOutput};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsumableConsumer {

    tracked_containers: Vec<i32>,
    total_container_count: i32,
    consumable_count: i32,

    #[serde(skip_serializing, default)]
    counter: u32,
    #[serde(skip_serializing, default)]
    found_counters: HashSet<i32>,

}

impl<O> Consumer<O> for ConsumableConsumer
where 
    O: ConsumerOutput<Output = OutputSeedIndexer>, {
        
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        if self.counter % 2 == 0 {
            self.consumable_count -= 1;
            let id = (seed * self.total_container_count as f32) as i32;
            
            self.found_counters.insert(id);
        }

        if self.consumable_count == 0 {
            for id in &self.tracked_containers {
                output.output(OutputSeedIndexer::ConsumableFound(*id, self.found_counters.contains(id)));
            }

            return true
        }
        
        self.counter += 1;
        false
    }

}
