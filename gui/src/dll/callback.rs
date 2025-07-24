
#[derive(Clone, Copy, Debug, )]
#[repr(u8)]
pub enum Code {
    Tokenizer = 1,
    RunInfo = 2,
    Mapper = 3,
    SeedIndexer = 4,
}

