use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{error::ObjectiveParseError, Objective};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RunObjective {

  pub level_name: String,
  pub secondary: bool,
  pub overload: bool,
  pub glitched: bool,
  pub early_drop: bool,
  pub player_count: u8,

}

impl Display for RunObjective {
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

    write!(f, "{}_{}{}{}{}{}.save", self.level_name.to_uppercase(), self.player_count, secondary, overload, glitched, early_drop)  
  }
}

impl<'a> Into<String> for &RunObjective {
  fn into(self) -> String {
    format!("{}", self)
  }
}

impl<'a> TryFrom<&'a str> for RunObjective {
  type Error = ObjectiveParseError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let mut obj = RunObjective::default();

    if !value.ends_with(".save") { 
      return Err(ObjectiveParseError::IncompatibleType(value[value.find('.').unwrap_or_default()..value.len()].to_owned())) 
    } 

    let mut split = value.trim_end_matches(".save").split('_').peekable();

    obj.level_name = split.next().unwrap_or_default().to_owned();
    obj.player_count = split.next().unwrap_or_default().to_owned().parse::<u8>().unwrap_or_default();

    if split.next_if_eq(&"sec").is_some() { obj.secondary = true; }; 
    if split.next_if_eq(&"ovrl").is_some() { obj.overload = true; }; 
    if split.next_if_eq(&"glitch").is_some() { obj.glitched = true; }; 
    if split.next_if_eq(&"edrop").is_some() { obj.early_drop = true; }; 

    Ok(obj)
  }
}

impl Objective for RunObjective {
  fn with_player_count(mut self, player_count: u8) -> Self {
    self.player_count = player_count;

    self  
  }
  
  fn get_player_count(&self) -> u8 {
    self.player_count
  }
}

impl RunObjective {

  pub fn from_name(name: String) -> RunObjective {
    RunObjective {
      level_name: name,
      ..Default::default()
    }
  }

  pub fn with_name(mut self, name: String) -> RunObjective {
    self.level_name = name;

    self
  }

  pub fn with_secondary(mut self, secondary: bool) -> RunObjective {
    self.secondary = secondary;

    self
  }

  pub fn with_overload(mut self, overload: bool) -> RunObjective {
    self.overload = overload;

    self
  }

  pub fn with_glitched(mut self, glitched: bool) -> RunObjective {
    self.glitched = glitched;

    self
  }

  pub fn with_early_drop(mut self, early_drop: bool) -> RunObjective {
    self.early_drop = early_drop;

    self
  }

  pub fn with_player_count(mut self, player_count: u8) -> RunObjective {
    self.player_count = player_count;

    self
  }

}


#[cfg(test)]
mod tests {
    use crate::run::objectives::run_objective::RunObjective;


  #[test]
  pub fn test_objectives_grr() {
    let mut run_obj = RunObjective::default();
    run_obj.player_count = 2;
    run_obj.secondary = true;
    run_obj.level_name = "R1B1".to_owned();

    assert_eq!(TryInto::<RunObjective>::try_into("R1B1_2_sec.save"), Ok(run_obj.clone()));
    assert_eq!(Into::<String>::into(&run_obj), "R1B1_2_sec.save");

    run_obj.overload = true;

    assert_eq!(TryInto::<RunObjective>::try_into("R1B1_2_sec_ovrl.save"), Ok(run_obj.clone()));
    assert_eq!(Into::<String>::into(&run_obj), "R1B1_2_sec_ovrl.save");
  
  }

}

