use std::fmt::Display;

use strum_macros::{Display, FromRepr};

use super::{data::KeyDescriptor, token::Token};

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

pub trait LocationGenerator {
    fn accept_token(&mut self, token: &Token) -> Option<Location>;
}

/// generates ColoredKey & BulkheadKey
#[derive(Default)]
pub struct KeyGenerator {
    first_iteration: Option<KeyDescriptor>,
}

impl LocationGenerator for KeyGenerator {
    fn accept_token(&mut self, token: &Token) -> Option<Location> {
        match token {
            Token::ItemAllocated(key_descriptor) => {
                self.first_iteration = Some(key_descriptor.clone());

                None
            }
            Token::ItemSpawn(zone, id) => match self.first_iteration.take() {
                Some(key_descriptor) => Some(key_descriptor.into_location(*zone, *id as u64)),
                None => None,
            },
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct ObjectiveItemGenerator {
    dimension: usize,
    buffer_names: Vec<ItemIdentifier>,
    buffer_zones: Vec<(usize, u64)>,
}

impl LocationGenerator for ObjectiveItemGenerator {
    fn accept_token(&mut self, token: &Token) -> Option<Location> {
        match token {
            Token::CollectableAllocated(zone) => {
                self.buffer_zones.push((self.dimension, *zone));
                
                self.buffer_zones.sort_by(|(d1, z1), (d2, z2)| {
                    let c = d1.cmp(d2);
                    match c {
                        std::cmp::Ordering::Equal => z1.cmp(z2),
                        _ => c,
                    }
                });

                None
            }
            // found an item that does not have a seed
            Token::ObjectiveSpawnedOverride(id, name) => {
                // unwrap should never fail since we always know we have collectable allocated
                let (_, zone) = self.buffer_zones.pop().unwrap_or((9999, 9999));

                Some(Location::BigObjective(Into::<&str>::into(name).to_owned(), zone, *id))
            }
            Token::CollectableItemID(id) => {
                let repr = ItemIdentifier::from_repr(*id).unwrap_or(ItemIdentifier::Unknown(*id));

                match repr {
                    ItemIdentifier::Cryo | ItemIdentifier::Cargo => {
                        // should never fail since we have collectable zone allocated
                        let (_, zone) = self.buffer_zones.remove(0);

                        Some(Location::BigCollectable(repr, zone))
                    }
                    _ => {
                        self.buffer_names.push(repr);

                        None
                    }
                }
            }
            Token::CollectableItemSeed(seed) => {
                let id = self.buffer_names.remove(0);
                let (_, zone) = self.buffer_zones.remove(0);

                Some(Location::Gatherable(id, zone, *seed))
            }
            Token::DimensionIncrease => {
                self.dimension += 1;

                None
            }
            Token::DimensionReset => {
                self.dimension = 0;

                None
            }
            _ => None,
        }
    }
}
