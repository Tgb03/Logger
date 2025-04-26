use core::logs::location::Location;
use std::collections::{HashMap, HashSet};

use egui::Color32;
use serde::{Deserialize, Serialize};

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
    Red,
    RGB(u8, u8, u8),
}

impl From<&MapperColor> for Color32 {
    fn from(value: &MapperColor) -> Self {
        match value {
            MapperColor::White => Color32::from_rgb(255, 255, 255),
            MapperColor::Blue => Color32::from_rgb(30, 155, 255),
            MapperColor::Green => Color32::from_rgb(45, 255, 30),
            MapperColor::Yellow => Color32::from_rgb(255, 255, 30),
            MapperColor::Red => Color32::from_rgb(255, 15, 15),
            MapperColor::RGB(r, g, b) => Color32::from_rgb(*r, *g, *b),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelView {
    key_colors: Vec<HashMap<u64, HashMap<MapperColor, Vec<u64>>>>,
    objective_colors: HashMap<String, HashMap<u64, HashMap<MapperColor, Vec<u64>>>>,
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
                            .map(|(color, values)| ((&color).into(), values.into_iter().collect()))
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
                                    ((&color).into(), values.into_iter().collect())
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
            Location::BigCollectable(_, _) => None,
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
