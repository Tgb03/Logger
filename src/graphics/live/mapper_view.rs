use std::collections::HashMap;

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::logs::location::{Location, LocationType};

use super::mapper::LookUpColor;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MapperColor {
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

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LevelView {
    key_colors: Vec<HashMap<u64, HashMap<MapperColor, Vec<u64>>>>,
    objective_colors: HashMap<String, HashMap<u64, HashMap<MapperColor, Vec<u64>>>>,
    ignore_zones: Vec<u64>,
}

impl LookUpColor for LevelView {
    fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32> {
        let zone = location.get_zone()?;
        let id = location.get_id()?;

        match location.get_type() {
            LocationType::Unknown => None,
            LocationType::ColoredKey | LocationType::BulkheadKey => {
                if location_vec_id >= self.key_colors.len() {
                    return None;
                }

                let map = &self.key_colors[location_vec_id];
                let zone_colors = map.get(&zone)?;

                for (color, vec) in zone_colors {
                    if vec.contains(&id) {
                        return Some(color.into());
                    }
                }

                None
            }
            LocationType::Objective => {
                let name = location.get_name()?;

                let map = self.objective_colors.get(name)?;
                let zone_colors = map.get(&zone)?;

                for (color, vec) in zone_colors {
                    if vec.contains(&id) {
                        return Some(color.into());
                    }
                }

                None
            }
        }
    }

    fn is_valid_zone(&self, zone: &u64) -> bool {
        !self.ignore_zones.contains(zone)
    }
}

impl LookUpColor for Option<&LevelView> {
    fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32> {
        self.map(|s| s.lookup(location_vec_id, location)).flatten()
    }

    fn is_valid_zone(&self, zone: &u64) -> bool {
        self.map(|s| s.is_valid_zone(zone)).unwrap_or(true)
    }
}
