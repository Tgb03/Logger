use std::num::Wrapping;

const MT19937: Wrapping<u32> = Wrapping(1812433253);

pub struct SeedGenerator {
    x: Wrapping<u32>,
    y: Wrapping<u32>,
    z: Wrapping<u32>,
    w: Wrapping<u32>,
}

impl From<i32> for SeedGenerator {
    fn from(value: i32) -> Self {
        let x = Wrapping(value as u32);
        let y = MT19937 * x + Wrapping(1);
        let z = MT19937 * y + Wrapping(1);
        let w = MT19937 * z + Wrapping(1);

        SeedGenerator { x, y, z, w }
    }
}

impl SeedGenerator {
    /// returns a value from [0 to 2^32 - 1]
    pub fn get_next_u32(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        return self.w.0;
    }

    /// returns a value from [0.0 to 1.0)
    pub fn get_next_f32(&mut self) -> f32 {
        let u = self.get_next_u32();
        return (u << 9) as f32 / 0xFFFFFFFFu32 as f32;
    }
}
