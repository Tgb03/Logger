use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::seed_gen::{consumers::{base_consumer::Consumer, key_id_consumer::KeyIDConsumer, OutputSeedIndexer}, ConsumerOutput};




#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ObjectiveConsumer {
    objectives_in_zones: VecDeque<Vec<KeyIDConsumer>>,

    #[serde(skip_serializing, default)]
    picked_zones: VecDeque<KeyIDConsumer>,
}

impl ObjectiveConsumer {

    fn calc_zone(&mut self, seed: f32) {
        if let Some(mut objectives_in_zone) = self.objectives_in_zones.pop_front() {
            let picked = (objectives_in_zone.len() as f32 * seed) as usize;
            println!("Selected: {picked} from {}", objectives_in_zone.len());
            self.picked_zones.push_back(objectives_in_zone.swap_remove(picked));
        }
    }

}

impl<O> Consumer<O> for ObjectiveConsumer
where
    O: ConsumerOutput<Output = OutputSeedIndexer>, {
        
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        if self.objectives_in_zones.len() > 0 {
            self.calc_zone(seed);
            return false;
        }

        if self.picked_zones.len() > 0 {
            println!("Zone picking {seed}");
            self.picked_zones.pop_front()
                .map(|mut z| z.take(seed, output));
        }

        self.objectives_in_zones.is_empty() && self.picked_zones.is_empty()
    }

}

