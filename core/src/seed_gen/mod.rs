use crate::seed_gen::consumers::OutputSeedIndexer;

pub mod unity_random;
pub mod levels;

mod seed_generator;
pub mod load_seed_consumers;

pub mod consumers;

pub trait ConsumerOutput {
    type Output;

    fn output(&mut self, data: Self::Output);

}

impl ConsumerOutput for Vec<OutputSeedIndexer> {
    type Output = OutputSeedIndexer;

    fn output(&mut self, data: Self::Output) {
        self.push(data);
    }
}
