use std::collections::{HashMap, HashSet};

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::logs::location::{Location, LocationType};

use super::mapper::LookUpColor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum MapperColor {
    Blue,
    Green,
    Yellow,
    Red,
    RGB(u8, u8, u8),
}

impl From<&MapperColor> for Color32 {
    fn from(value: &MapperColor) -> Self {
        match value {
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
        let key_colors: Vec<HashMap<u64, Vec<(Color32, HashSet<u64>)>>> = self.key_colors
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

        let objective_colors: HashMap<String, HashMap<u64, Vec<(Color32, HashSet<u64>)>>> = self.objective_colors
            .into_iter()
            .map(|(name, map)| {(
                    name,
                    map.into_iter()
                        .map(|(key, map)| {
                            let transformed_vec: Vec<(Color32, HashSet<u64>)> = map
                                .into_iter()
                                .map(|(color, values)| ((&color).into(), values.into_iter().collect()))
                                .collect();
                            (key, transformed_vec)
                        })
                        .collect()
                )
            })
            .collect();

        OptimizedLevelView { 
            key_colors, 
            objective_colors, 
            ignore_zones: self.ignore_zones
                .into_iter()
                .collect(),
            default_color: (&self.default_color).into()
        }
    }
}

impl LookUpColor for OptimizedLevelView {
    fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32> {
        match location.get_type() {
            LocationType::Unknown => None,
            LocationType::ColoredKey | LocationType::BulkheadKey => {
                let id = location.get_id()?;
                let zone = location.get_zone()?;

                self.key_colors
                    .get(location_vec_id)?
                    .get(&zone)?
                    .iter()
                    .filter(|(_, vec)| vec.contains(&id))
                    .next()
                    .map(|(color, _)| *color)
            },
            LocationType::Objective => {
                let name = location.get_name()?;
                let zone = location.get_zone()?;
                let id = location.get_id()?;

                self.objective_colors
                    .get(name)?
                    .get(&zone)?
                    .iter()
                    .filter(|(_, map)| map.contains(&id))
                    .next()
                    .map(|(color, _)| *color)
                    
            },
        }
    }

    fn is_valid_zone(&self, zone: &u64) -> bool {
        !self.ignore_zones.contains(zone)
    }
}

impl LookUpColor for Option<&OptimizedLevelView> {
    fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32> {
        self.map(|s| s.lookup(location_vec_id, location).unwrap_or(s.default_color))
    }

    fn is_valid_zone(&self, zone: &u64) -> bool {
        self.map(|s| s.is_valid_zone(zone)).unwrap_or(true)
    }
}