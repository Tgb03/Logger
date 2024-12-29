
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

  pub fn get_id(&self) -> String {
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

    //println!("Saved: {}{}{}{}{}_{}.save", objective_data.level_name, secondary, overload, glitched, early_drop, objective_data.get_player_count());
    format!("{}{}{}{}{}_{}.save", self.level_name, secondary, overload, glitched, early_drop, self.get_player_count())
  }
}
