
use std::{collections::{HashMap, HashSet}, fmt, ops::RangeInclusive};

use egui::Color32;
use glr_core::location::Location;
use serde::{de::{self, SeqAccess, Visitor}, Deserialize, Deserializer, Serialize};

pub trait LookUpColor {
    fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32>;
    fn is_valid_zone(&self, zone: &u64) -> bool;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum MapperColor {
    White,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Grey,
    RGB(u8, u8, u8),
}

impl From<&MapperColor> for Color32 {
    fn from(value: &MapperColor) -> Self {
        match value {
            MapperColor::White => Color32::from_rgb(255, 255, 255),
            MapperColor::Blue => Color32::from_rgb(30, 155, 255),
            MapperColor::Green => Color32::from_rgb(45, 255, 30),
            MapperColor::Yellow => Color32::from_rgb(255, 255, 30),
            MapperColor::Orange => Color32::from_rgb(255, 69, 0),
            MapperColor::Red => Color32::from_rgb(255, 15, 15),
            MapperColor::RGB(r, g, b) => Color32::from_rgb(*r, *g, *b),
            MapperColor::Purple => Color32::from_rgb(160, 32, 240),
            MapperColor::Grey => Color32::from_rgb(100, 100, 100),
        }
    }
}

#[derive(Serialize, Debug)]
enum KeyID {
    VecID(u64),
    RangeID(RangeInclusive<u64>)
}

impl KeyID {
    pub fn add_to_set(self, set: &mut HashSet<u64>) {
        match self {
            KeyID::VecID(id) => { set.insert(id); },
            KeyID::RangeID(range) => {
                for id in range {
                    set.insert(id);
                }
            },
        };
    }
}

impl<'de> Deserialize<'de> for KeyID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyIDVisitor;

        impl<'de> Visitor<'de> for KeyIDVisitor {
            type Value = KeyID;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a u64 or a two-element array representing a range")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
                Ok(KeyID::VecID(value))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let start: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let end: u64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(KeyID::RangeID(start..=end))
            }
        }

        deserializer.deserialize_any(KeyIDVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelView {
    key_colors: Vec<HashMap<u64, HashMap<MapperColor, Vec<KeyID>>>>,
    objective_colors: HashMap<String, HashMap<u64, HashMap<MapperColor, Vec<KeyID>>>>,
    ignore_zones: Vec<u64>,
    default_color: MapperColor,
}

pub struct OptimizedLevelView {
    key_colors: Vec<HashMap<u64, Vec<(Color32, HashSet<u64>)>>>,
    objective_colors: HashMap<String, HashMap<u64, Vec<(Color32, HashSet<u64>)>>>,
    ignore_zones: HashSet<u64>,
    default_color: Color32,
}

impl Into<OptimizedLevelView> for LevelView {
    fn into(self) -> OptimizedLevelView {
        let key_colors: Vec<HashMap<u64, Vec<(Color32, HashSet<u64>)>>> = self
            .key_colors
            .into_iter()
            .map(|map| {
                map.into_iter()
                    .map(|(key, inner_map)| {
                        let transformed_vec: Vec<(Color32, HashSet<u64>)> = inner_map
                            .into_iter()
                            .map(|(color, values)| (
                                (&color).into(), 
                                values.into_iter()
                                    .fold(HashSet::new(), |mut set, id| {
                                        id.add_to_set(&mut set);

                                        set
                                    })
                            ))
                            .collect();
                        (key, transformed_vec)
                    })
                    .collect()
            })
            .collect();

        let objective_colors: HashMap<String, HashMap<u64, Vec<(Color32, HashSet<u64>)>>> = self
            .objective_colors
            .into_iter()
            .map(|(name, map)| {
                (
                    name,
                    map.into_iter()
                        .map(|(key, map)| {
                            let transformed_vec: Vec<(Color32, HashSet<u64>)> = map
                                .into_iter()
                                .map(|(color, values)| {
                                    ((&color).into(), values.into_iter()
                                    .fold(HashSet::new(), |mut set, id| {
                                        id.add_to_set(&mut set);

                                        set
                                    }))
                                })
                                .collect();
                            (key, transformed_vec)
                        })
                        .collect(),
                )
            })
            .collect();

        OptimizedLevelView {
            key_colors,
            objective_colors,
            ignore_zones: self.ignore_zones.into_iter().collect(),
            default_color: (&self.default_color).into(),
        }
    }
}

impl LookUpColor for OptimizedLevelView {
    fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32> {
        match location {
            Location::ColoredKey(_, zone, id) | Location::BulkheadKey(_, zone, id) => Some(
                self.key_colors
                    .get(location_vec_id)?
                    .get(zone)?
                    .iter()
                    .filter(|(_, vec)| vec.contains(id))
                    .next()?
                    .0,
            ),
            Location::Gatherable(item_identifier, zone, id) => Some(
                self.objective_colors
                    .get(&item_identifier.to_string())?
                    .get(zone)?
                    .iter()
                    .filter(|(_, vec)| vec.contains(id))
                    .next()?
                    .0,
            ),
            Location::BigObjective(name, zone, area) => Some(
                self.objective_colors
                    .get(name)?
                    .get(zone)?
                    .iter()
                    .filter(|(_, vec)| vec.contains(area))
                    .next()?
                    .0,
            ),
            _ => None,
        }
    }

    fn is_valid_zone(&self, zone: &u64) -> bool {
        !self.ignore_zones.contains(zone)
    }
}

impl LookUpColor for Option<&OptimizedLevelView> {
    fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32> {
        self.map(|s| {
            s.lookup(location_vec_id, location)
                .unwrap_or(s.default_color)
        })
    }

    fn is_valid_zone(&self, zone: &u64) -> bool {
        self.map(|s| s.is_valid_zone(zone)).unwrap_or(true)
    }
}