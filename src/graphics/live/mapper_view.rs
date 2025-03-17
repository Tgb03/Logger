use std::collections::HashMap;

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::logs::location::{Location, LocationType};

use super::mapper::LookUpColor;



#[derive(Default, Serialize, Deserialize, Debug)]
pub struct LevelView {
  
  key_colors: Vec<HashMap<u64, HashMap<[u8; 3], Vec<u64>>>>,
  objective_colors: HashMap<String, HashMap<u64, HashMap<[u8; 3], Vec<u64>>>>,
  ignore_zones: Vec<u64>,

}

impl LookUpColor for LevelView {
  fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32> {

    let zone = location.get_zone()?;
    let id = location.get_id()?;

    match location.get_type() {
      LocationType::Unknown => None,
      LocationType::ColoredKey | LocationType::BulkheadKey => {
        if location_vec_id >= self.key_colors.len() { return None }

        let map = &self.key_colors[location_vec_id];
        let zone_colors = map.get(&zone)?;
        
        for (color, vec) in zone_colors {
          if vec.contains(&id) {
            return Some(Color32::from_rgb(
              color[0], 
              color[1],
              color[2] 
            ))
          }
        }

        None
      },
      LocationType::Objective => {
        let name = location.get_name()?;

        let map = self.objective_colors.get(name)?;
        let zone_colors = map.get(&zone)?;

        for (color, vec) in zone_colors {
          if vec.contains(&id) {
            return Some(Color32::from_rgb(
              color[0], 
              color[1],
              color[2] 
            ))
          }
        }

        None
      },
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