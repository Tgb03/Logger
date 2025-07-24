use core::{logs::data::LevelDescriptor, run::split::NamedSplit};

use serde::{Deserialize, Serialize};

use crate::dll::timed_run::TimedRun;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum ResourceType {
    #[default]
    Healthpack,
    DisinfectPack,
    Ammopack,
    ToolRefillpack,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputSeedIndexer {
    Seed(f32),
    Key(String, i32, i32),           // zone, id
    ResourcePack(ResourceType, i32), // count
    ConsumableFound(i32, bool),      // id of box, found or not
    GenerationEnd,
    GenerationStart,
    ZoneGenEnded(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RunGeneratorResult {

    GameStarted(LevelDescriptor, u8),
    SplitAdded(NamedSplit),

    SecondaryDone,
    OverloadDone,
    CheckpointUsed,

    LevelRun(TimedRun),

}
