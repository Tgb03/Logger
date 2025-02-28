use std::{collections::HashMap, fs::File, io::Read};

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
    let link = "https://raw.githubusercontent.com/Tgb03/Logger/collectables/collectable_maps.ron";

    let resp = reqwest::blocking::get(link)
      .ok()?
      .text()
      .ok()?;

    println!("{}", resp);

    let result = ron::from_str(&resp);

    match result {
      Ok(res) => Some(res),
      Err(e) => { println!("Error: {:?}", e); None },
    }
  }

}

#[cfg(test)]
mod tests {
    use super::CollectableMapper;


  #[test]
  fn load_file() {

    let map = CollectableMapper::load_from_web();

    assert!(map.is_some())

  }

}
