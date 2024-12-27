
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
}
