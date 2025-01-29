
use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectiveData {

  pub level_name: String,
  pub secondary: bool,
  pub overload: bool,
  pub glitched: bool,
  pub early_drop: bool,
  pub player_count: u8,

}

impl Display for ObjectiveData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let secondary = match self.secondary {
      true => "_sec",
      false => ""
    };
    let overload = match self.overload {
      true => "_ovrl",
      false => ""
    };
    let glitched = match self.glitched {
      true => "_glitch",
      false => ""
    };
    let early_drop = match self.early_drop {
      true => "_edrop",
      false => ""
    };

    write!(f, "{}_{}{}{}{}{}.save", self.level_name.to_uppercase(), self.get_player_count(), secondary, overload, glitched, early_drop)  
  }
}

impl ObjectiveData {

  pub fn new() -> ObjectiveData {
    ObjectiveData {
      level_name: String::new(),
      secondary: false,
      overload: false,
      glitched: false,
      early_drop: false,
      player_count: 0,
    }
  }

  pub fn get_player_count(&self) -> u8 {
    self.player_count
  }

  pub fn from(level_name: String, secondary: bool, overload: bool, glitched: bool, early_drop: bool, player_count: u8) -> ObjectiveData {
    ObjectiveData { level_name, secondary, overload, glitched, early_drop, player_count }
  }

  pub fn add_player(&mut self) {
    self.player_count += 1;
  }

  pub fn reset_players(&mut self) {
    self.player_count = 0;
  }

  pub fn from_id(id: &String) -> Self {
    let id_arr: Vec<&str> = id.trim_end_matches(".save").split('_').collect();

    Self {
      level_name: id_arr[0].to_owned().to_uppercase(),
      secondary: id.contains("sec"),
      overload: id.contains("ovrl"),
      glitched: id.contains("glitch"),
      early_drop: id.contains("edrop"),
      player_count: id_arr[1].to_owned().parse::<u8>().unwrap(),
    }
  }
}
