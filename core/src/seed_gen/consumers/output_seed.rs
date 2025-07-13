use serde::{Deserialize, Serialize};

use crate::seed_gen::{consumers::{base_consumer::Consumer, OutputSeedIndexer}, ConsumerOutput};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OutputSeed;

impl<O> Consumer<O> for OutputSeed
where
    O: ConsumerOutput<Output = OutputSeedIndexer>,
{
    fn take(&mut self, seed: f32, output: &mut O) -> bool {
        output.output(OutputSeedIndexer::Seed(seed));

        true
    }
}
