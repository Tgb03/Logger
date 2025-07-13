use std::fmt::Display;

use strum_macros::{Display, FromRepr};

/// taken from https://github.com/Angry-Maid/rusted-mapper
#[derive(FromRepr, Display, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u8)]
pub enum ItemIdentifier {
    ID = 128,
    PD = 129,
    Cell = 131,
    FogTurbine = 133,
    Neonate = 137,
    Cryo = 148,
    GLP1 = 149,
    OSIP = 150,
    Datasphere = 151,
    PlantSample = 153,
    HiSec = 154,
    MWP = 164,
    DataCubeR8 = 165,
    DataCube = 168,
    GLP2 = 169,
    Cargo = 176,
    Unknown(u8),
}

#[derive(Clone)]
pub enum Location {
    // name, zone, id
    ColoredKey(String, u64, u64),
    BulkheadKey(String, u64, u64),

    // gatherable identifier, zone, id
    Gatherable(ItemIdentifier, u64, u64),

    // hsu/terminal/other: name, zone and XX_area
    BigObjective(String, u64, u64),

    // big collectables (cryo, cargos etc.): only identifier and zone
    BigCollectable(ItemIdentifier, u64),
}

impl Location {
    pub fn get_zone(&self) -> u64 {
        match self {
            Location::ColoredKey(_, zone, _)
            | Location::BulkheadKey(_, zone, _)
            | Location::Gatherable(_, zone, _)
            | Location::BigObjective(_, zone, _)
            | Location::BigCollectable(_, zone) => *zone,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::ColoredKey(name, zone, id)
            | Location::BulkheadKey(name, zone, id)
            | Location::BigObjective(name, zone, id) => {
                write!(f, "{}: ZONE {} at {}", name, zone, id)
            }
            Location::Gatherable(identifier, zone, id) => {
                write!(f, "{}: ZONE {} at {}", identifier, zone, id)
            }
            Location::BigCollectable(name, zone) => write!(f, "{}: ZONE {}", name, zone),
        }
    }
}

