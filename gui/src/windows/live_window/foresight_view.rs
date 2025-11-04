use std::collections::{HashMap, HashSet};

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::windows::live_window::mapper_view::{KeyID, MapperColor};

type InnerForesightView = HashMap<String, HashMap<i32, HashMap<MapperColor, Vec<KeyID>>>>;
type InnerOptimizedForesightView = HashMap<String, HashMap<i32, HashMap<Color32, HashSet<i32>>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForesightView {
    data: InnerForesightView,
    default_color: MapperColor,
    ignore_zones: Vec<i32>,
    #[serde(default)] ignore_pairs: Vec<(String, i32)>,
}

pub struct OptimizedForesightView {
    data: InnerOptimizedForesightView,
    default_color: Color32,
    ignore_zones: HashSet<i32>,
    ignore_pairs: HashMap<String, HashSet<i32>>,
}

impl Into<OptimizedForesightView> for ForesightView {
    fn into(self) -> OptimizedForesightView {
        OptimizedForesightView {
            data: optimize_foresight_view(self.data),
            default_color: (&self.default_color).into(),
            ignore_zones: self.ignore_zones.into_iter().collect(),
            ignore_pairs: self.ignore_pairs.into_iter()
                .fold(HashMap::new(), |mut acc, (name, id)| {
                    acc.entry(name).or_insert_with(HashSet::new).insert(id);
                    acc
                }),
        }
    }
}

fn optimize_foresight_view(view: InnerForesightView) -> InnerOptimizedForesightView {
    let mut optimized: InnerOptimizedForesightView = HashMap::new();

    for (string_key, i32_map) in view {
        let mut new_i32_map: HashMap<i32, HashMap<Color32, HashSet<i32>>> = HashMap::new();

        for (i32_key, color_map) in i32_map {
            let mut new_color_map: HashMap<Color32, HashSet<i32>> = HashMap::new();

            for (mapper_color, key_ids) in color_map {
                let color32: Color32 = (&mapper_color).into();

                let mut id_set: HashSet<i32> = HashSet::new();
                for key_id in key_ids {
                    key_id.add_to_set_i32(&mut id_set);
                }

                new_color_map.insert(color32, id_set);
            }

            new_i32_map.insert(i32_key, new_color_map);
        }

        optimized.insert(string_key, new_i32_map);
    }

    optimized
}

pub trait LookUpForesight {
    fn lookup(&self, name: &String, zone: &i32, id: &i32) -> Option<Color32>;
    fn is_ignored(&self, zone: &i32) -> bool;
    fn is_name_ignored(&self, name: &String, zone: &i32) -> bool;
}

impl LookUpForesight for InnerOptimizedForesightView {
    fn lookup(&self, name: &String, zone: &i32, id: &i32) -> Option<Color32> {
        self.get(name)?
            .get(&zone)?
            .iter()
            .find(|(_, set)| set.contains(&id))
            .map(|(c, _)| c.clone())
    }
    
    fn is_ignored(&self, _: &i32) -> bool {
        false
    }
    
    fn is_name_ignored(&self, _: &String, _: &i32) -> bool {
        false
    }
}

impl LookUpForesight for OptimizedForesightView {
    fn lookup(&self, name: &String, zone: &i32, id: &i32) -> Option<Color32> {
        self.data
            .lookup(name, zone, id)
            .or(Some(self.default_color))
    }
    
    fn is_ignored(&self, zone: &i32) -> bool {
        self.ignore_zones.contains(&zone)
    }

    fn is_name_ignored(&self, name: &String, zone: &i32) -> bool {
        self.ignore_zones.contains(&zone) ||
        self.ignore_pairs.get(name)
            .map(|v| v.contains(zone))
            .unwrap_or_default()
    }
}

impl<T> LookUpForesight for Option<&T>
where
    T: LookUpForesight,
{
    fn lookup(&self, name: &String, zone: &i32, id: &i32) -> Option<Color32> {
        self.as_ref()?.lookup(name, zone, id)
    }
    
    fn is_ignored(&self, zone: &i32) -> bool {
        self.as_ref()
            .map(|v| v.is_ignored(zone))
            .unwrap_or(false)
    }
    
    fn is_name_ignored(&self, name: &String, zone: &i32) -> bool {
        self.as_ref()
            .map(|v| v.is_name_ignored(name, zone))
            .unwrap_or(false)
    }
}
