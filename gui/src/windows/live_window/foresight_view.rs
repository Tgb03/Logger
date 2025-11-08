use std::collections::{HashMap, HashSet};

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::windows::live_window::mapper_view::{KeyID, MapperColor};

type InnerForesightView = HashMap<String, HashMap<i32, HashMap<MapperColor, Vec<KeyID>>>>;
type InnerOptimizedForesightView = HashMap<String, HashMap<i32, HashMap<Color32, HashSet<i32>>>>;

mod option_as_value {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match value {
            Some(inner) => inner.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        Ok(Some(T::deserialize(deserializer)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForesightView {
    #[serde(default, alias = "data", alias = "color_data")]
    data: InnerForesightView,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "option_as_value")] 
    default_color: Option<MapperColor>,
    #[serde(default)] ignore_zones: Vec<i32>,
    #[serde(default)] ignore_pairs: HashMap<String, HashSet<i32>>,
    #[serde(default, alias = "conditional_ignores", alias = "conditional")] conditional_ignores: Vec<ConditionalIgnore<InnerForesightView>>,
    #[serde(default)] rename: HashMap<String, String>,
    #[serde(default)] group_zones: HashSet<i32>,
    #[serde(default)] order: Vec<(i32, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewCondition {

    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "option_as_value")]
    zone: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "option_as_value")]
    id: Option<KeyID>,

}

impl ViewCondition {

    pub fn matches(&self, name: &String, zone: &i32, id: &i32) -> bool {
        self.name == *name && 
        self.zone.as_ref().is_none_or(|v| v == zone) &&
        self.id.as_ref().is_none_or(|v| v.contains(id))
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionalIgnore<CD> {
    condition: ViewCondition,
    #[serde(default)] ignore_zones: HashSet<i32>,
    #[serde(default)] ignore_pairs: HashMap<String, HashSet<i32>>,
    #[serde(default, alias = "data", alias = "color_data")] color_data: CD, 
}

impl Into<ConditionalIgnore<InnerOptimizedForesightView>> for ConditionalIgnore<InnerForesightView> {
    fn into(self) -> ConditionalIgnore<InnerOptimizedForesightView> {
        ConditionalIgnore { 
            condition: self.condition, 
            ignore_zones: self.ignore_zones, 
            ignore_pairs: self.ignore_pairs, 
            color_data: optimize_foresight_view(self.color_data) 
        }
    }
}

pub struct OptimizedForesightView {
    data: InnerOptimizedForesightView,
    default_color: Option<Color32>,
    ignore_zones: HashSet<i32>,
    ignore_pairs: HashMap<String, HashSet<i32>>,
    conditional_ignores: Vec<ConditionalIgnore<InnerOptimizedForesightView>>,
    conditions_triggered: Vec<bool>,
    rename: HashMap<String, String>,
    group_zones: HashSet<i32>,
    order: HashMap<i32, HashMap<String, usize>>,
}

pub trait AddToConditions {

    fn add_found(&mut self, name: &String, zone: &i32, id: &i32);
    fn reset(&mut self);

}

impl AddToConditions for OptimizedForesightView {
    fn add_found(&mut self, name: &String, zone: &i32, id: &i32) {
        for (pos_vec, condition) in self.conditional_ignores.iter().enumerate() {
            if condition.condition.matches(name, zone, id) {
                self.conditions_triggered[pos_vec] = true;
            }
        }
    }
    
    fn reset(&mut self) {
        self.conditions_triggered.iter_mut()
            .for_each(|v| *v = false);
    }
}

impl<T> AddToConditions for Option<&mut T> 
where T: AddToConditions {
    fn add_found(&mut self, name: &String, zone: &i32, id: &i32) {
        self.as_mut().map(|v| v.add_found(name, zone, id));
    }

    fn reset(&mut self) {
        self.as_mut().map(|v| v.reset());
    }
}

impl Into<OptimizedForesightView> for ForesightView {
    fn into(self) -> OptimizedForesightView {
        let conditional_size = self.conditional_ignores.len();
        OptimizedForesightView {
            data: optimize_foresight_view(self.data),
            default_color: self.default_color.map(|v| (&v).into()),
            ignore_zones: self.ignore_zones.into_iter().collect(),
            ignore_pairs: self.ignore_pairs,
            conditional_ignores: self.conditional_ignores.into_iter()
                .map(|v| v.into())
                .collect(),
            conditions_triggered: vec![false; conditional_size],
            rename: self.rename,
            group_zones: self.group_zones,
            order: self.order.into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut hmap, (key, (zone, name))| {
                    if !hmap.contains_key(&zone) {
                        hmap.insert(zone, HashMap::new());
                    }

                    hmap.get_mut(&zone).map(|h| h.insert(name, key));

                    hmap
                })
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
    fn is_ignored(&self, name: &String, zone: &i32, id: &i32) -> bool;

    fn rename(&self, name: &String) -> Option<String>;
    fn is_grouped(&self, zone: &i32) -> bool;
    fn get_order(&self, name: &String, zone: &i32) -> Option<usize>;
}

impl LookUpForesight for InnerOptimizedForesightView {
    fn lookup(&self, name: &String, zone: &i32, id: &i32) -> Option<Color32> {
        self.get(name)?
            .get(&zone)?
            .iter()
            .find(|(_, set)| set.contains(&id))
            .map(|(c, _)| c.clone())
    }
    
    fn is_ignored(&self, _: &String, _: &i32, _: &i32) -> bool {
        false
    }
    
    fn rename(&self, _: &String) -> Option<String> {
        None
    }
    
    fn is_grouped(&self, _: &i32) -> bool {
        false
    }
    
    fn get_order(&self, _: &String, _: &i32) -> Option<usize> {
        None
    }
}

impl LookUpForesight for OptimizedForesightView {
    fn lookup(&self, name: &String, zone: &i32, id: &i32) -> Option<Color32> {
        self.data
            .lookup(name, zone, id)
            .or_else(|| 
                self.conditional_ignores.iter()
                    .enumerate()
                    .filter_map(|(vec_pos, v)| {
                        if self.conditions_triggered[vec_pos] {
                            v.color_data.lookup(name, zone, id)
                        } else {
                            None
                        }
                    })
                    .next()
            )
            .or(self.default_color)
    }
    
    fn is_ignored(&self, name: &String, zone: &i32, _: &i32) -> bool {
        self.ignore_zones.contains(&zone) ||
        self.ignore_pairs.get(name)
            .map(|v| v.contains(zone))
            .unwrap_or_default() ||
        self.conditional_ignores.iter()
            .enumerate()
            .any(|(vec_pos, v)| {
                self.conditions_triggered[vec_pos] == true && 
                (
                    v.ignore_zones.contains(zone) ||
                    v.ignore_pairs.get(name)
                        .map(|v| v.contains(zone))
                        .unwrap_or_default()
                )
            })
    }
    
    fn rename(&self, name: &String) -> Option<String> {
        self.rename
            .get(name)
            .cloned()
    }
    
    fn is_grouped(&self, zone: &i32) -> bool {
        self.group_zones.contains(zone)
    }
    
    fn get_order(&self, name: &String, zone: &i32) -> Option<usize> {
        self.order.get(zone)?
            .get(name)
            .cloned()
    }
}

impl<T> LookUpForesight for Option<&T>
where
    T: LookUpForesight,
{
    fn lookup(&self, name: &String, zone: &i32, id: &i32) -> Option<Color32> {
        self.as_ref()?.lookup(name, zone, id)
    }
    
    fn is_ignored(&self, name: &String, zone: &i32, id: &i32) -> bool {
        self.as_ref()
            .map(|v| v.is_ignored(name, zone, id))
            .unwrap_or(false)
    }
    
    fn rename(&self, name: &String) -> Option<String> {
        self.as_ref()
            .map(|v| v.rename(name))
            .flatten()
    }
    
    fn is_grouped(&self, zone: &i32) -> bool {
        self.as_ref()
            .map(|v| v.is_grouped(zone))
            .unwrap_or_default()
    }
    
    fn get_order(&self, name: &String, zone: &i32) -> Option<usize> {
        self.as_ref()
            .map(|v| v.get_order(name, zone))
            .unwrap_or_default()
    }
}
