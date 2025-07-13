use crate::seed_gen::seed_generator::SeedGenerator;

pub struct UnityRandom {
    current_id: usize,
    seeds: [f32; 1024],
}

impl From<i32> for UnityRandom {
    fn from(value: i32) -> Self {
        let mut sg = SeedGenerator::from(value);
        let mut seeds = [0f32; 1024];

        for i in 0..1024 {
            seeds[i] = sg.get_next_f32() * 0.9999f32;
        }

        return UnityRandom {
            current_id: 0,
            seeds,
        };
    }
}

impl Iterator for UnityRandom {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.seeds[self.current_id];

        self.current_id += 1;
        if self.current_id == 1024 {
            self.current_id = 0;
        }

        Some(result)
    }
}
