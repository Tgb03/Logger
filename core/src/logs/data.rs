#[derive(Default)]
#[repr(u8)]
pub enum Rundown {
    #[default]
    Modded,
    R7 = 31,
    R1 = 32,
    R2 = 33,
    R3 = 34,
    R8 = 35,
    R4 = 37,
    R5 = 38,
    Tutorial = 39,
    R6 = 41,
}

pub enum GatherItem {}
