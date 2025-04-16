use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CollectableMapper {
    map: HashMap<String, HashMap<u64, HashMap<u64, u64>>>,
}

impl CollectableMapper {
    /// load the file from the web.
    ///
    /// it assumes the linked file is a .ron file which contains the correct
    /// hash map for this mapper.
    pub fn load_from_web() -> Option<Self> {
        let link = "https://raw.githubusercontent.com/Tgb03/Logger/master/collectable_maps.ron";

        let resp = reqwest::blocking::get(link).ok()?.text().ok()?;

        let result = ron::from_str(&resp);

        match result {
            Ok(res) => Some(res),
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    }

    pub fn load_from_file() -> Option<Self> {
        let baked = include_str!("../../../collectable_maps.ron");

        ron::from_str(&baked).ok()
    }

    pub fn get_id(&self, level_name: &str, zone: u64, seed: u64) -> Option<u64> {
        //println!("Called: {} in {} at {}", level_name, zone, seed);

        self.map.get(level_name)?.get(&zone)?.get(&seed).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::CollectableMapper;

    #[test]
    fn load_file() {
        let map = CollectableMapper::load_from_web();

        assert!(map.is_some());
        let map = map.unwrap();

        assert_eq!(map.get_id("R8B3", 336, 1913762560), Some(19));
        assert_eq!(map.get_id("R8B3", 334, 1604288640), Some(0));
    }
}
