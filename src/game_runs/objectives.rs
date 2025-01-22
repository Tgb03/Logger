

use std::fmt::Display;
use strum_macros::EnumIter;

use crate::objective_data::ObjectiveData;

use super::levels::GameRunRundown;

const MAIN_ONLY: (bool, bool) = (false, false);
const SECD_ONLY: (bool, bool) = (true, false);
const OVRL_ONLY: (bool, bool) = (false, true);
const PE_OBJECT: (bool, bool) = (true, true);

const OPTIONALS_ALL: [(bool, bool); 83] = [
  MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY,
  MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY,
  MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY,
  SECD_ONLY, PE_OBJECT, PE_OBJECT, SECD_ONLY, PE_OBJECT, PE_OBJECT, SECD_ONLY, PE_OBJECT, PE_OBJECT, SECD_ONLY, PE_OBJECT, SECD_ONLY,
  SECD_ONLY, PE_OBJECT, PE_OBJECT, PE_OBJECT, PE_OBJECT, SECD_ONLY, MAIN_ONLY, SECD_ONLY, PE_OBJECT, SECD_ONLY, SECD_ONLY, MAIN_ONLY, MAIN_ONLY,
  MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, SECD_ONLY, SECD_ONLY, MAIN_ONLY, SECD_ONLY, PE_OBJECT, MAIN_ONLY, MAIN_ONLY, SECD_ONLY, PE_OBJECT, MAIN_ONLY,
  MAIN_ONLY, MAIN_ONLY, SECD_ONLY, PE_OBJECT, MAIN_ONLY, PE_OBJECT, PE_OBJECT, OVRL_ONLY, MAIN_ONLY, MAIN_ONLY,
  MAIN_ONLY, SECD_ONLY, SECD_ONLY, SECD_ONLY, SECD_ONLY, SECD_ONLY, PE_OBJECT, MAIN_ONLY, MAIN_ONLY, SECD_ONLY, SECD_ONLY, SECD_ONLY
];

#[derive(Clone, PartialEq, Eq, EnumIter)]
pub enum GameRunObjective {

  AnyPercent,
  FullPercent,

}

impl Into<&[(bool, bool)]> for GameRunRundown {
  fn into(self) -> &'static [(bool, bool)] {
    match self {
      GameRunRundown::Rundown1 => &OPTIONALS_ALL[0..6],
      GameRunRundown::Rundown2 => &OPTIONALS_ALL[6..16],
      GameRunRundown::Rundown3 => &OPTIONALS_ALL[16..23],
      GameRunRundown::Rundown4 => &OPTIONALS_ALL[23..35],
      GameRunRundown::Rundown5 => &OPTIONALS_ALL[35..48],
      GameRunRundown::Rundown6 => &OPTIONALS_ALL[48..61],
      GameRunRundown::Rundown7 => &OPTIONALS_ALL[61..71],
      GameRunRundown::Rundown8 => &OPTIONALS_ALL[71..83],
      GameRunRundown::FullGame => &OPTIONALS_ALL,
    }
  }
}

impl Display for GameRunObjective {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GameRunObjective::AnyPercent => write!(f, "Any%"),
      GameRunObjective::FullPercent => write!(f, "100%"),
    }
  }
}

impl GameRunObjective {

  fn into_single_obj(&self, level_name: &str, has_secondary: bool, has_overload: bool, player_count: u8) -> ObjectiveData {
    match self {
      GameRunObjective::AnyPercent => ObjectiveData::from(
        level_name.to_owned(), 
        false, 
        false, 
        false, 
        false, 
        player_count
      ),
      GameRunObjective::FullPercent => ObjectiveData::from(
        level_name.to_owned(), 
        has_secondary, 
        has_overload, 
        false, 
        false, 
        player_count
      ),
    }
  }

  pub fn into_objective(self, levels: GameRunRundown, player_count: u8) -> Vec<ObjectiveData> {
    
    let optionals: &[(bool, bool)] = levels.clone().into();

    let mut result: Vec<ObjectiveData> = Vec::with_capacity(optionals.len());
    let levels: &[&str] = levels.into();

    for (level, (secondary, overload)) in levels.iter().zip(optionals) {

      result.push(self.into_single_obj(level, *secondary, *overload, player_count));
    
    }

    result
  }

}

#[cfg(test)]
mod tests {
    use crate::{game_runs::{levels::GameRunRundown, objectives::{MAIN_ONLY, OVRL_ONLY, PE_OBJECT, SECD_ONLY}}, objective_data::ObjectiveData};

    use super::GameRunObjective;

  #[test]
  pub fn test_objectives_grr() {
    let objectives_1: &[(bool, bool)] = GameRunRundown::Rundown1.into();
    let objectives_4: &[(bool, bool)] = GameRunRundown::Rundown4.into();
    let objectives_7: &[(bool, bool)] = GameRunRundown::Rundown7.into();

    assert_eq!(objectives_1, vec![MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY, MAIN_ONLY]);
    assert_eq!(objectives_4, vec![SECD_ONLY, PE_OBJECT, PE_OBJECT, SECD_ONLY, PE_OBJECT, PE_OBJECT, SECD_ONLY, PE_OBJECT, PE_OBJECT, SECD_ONLY, PE_OBJECT, SECD_ONLY]);
    assert_eq!(objectives_7, vec![MAIN_ONLY, MAIN_ONLY, SECD_ONLY, PE_OBJECT, MAIN_ONLY, PE_OBJECT, PE_OBJECT, OVRL_ONLY, MAIN_ONLY, MAIN_ONLY]);
  
  }

  #[test]
  pub fn test_objectives_r1() {
    let objectives = GameRunObjective::AnyPercent.into_objective(GameRunRundown::Rundown1, 4);

    assert_eq!(objectives, vec![
      ObjectiveData::from("R1A1".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R1B1".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R1B2".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R1C1".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R1C2".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R1D1".to_owned(), false, false, false, false, 4),
    ]);
  }

  #[test]
  pub fn test_objectives_all_r4() {
    let objectives = GameRunObjective::FullPercent.into_objective(GameRunRundown::Rundown4, 4);

    assert_eq!(objectives, vec![
      ObjectiveData::from("R4A1".to_owned(), true, false, false, false, 4),
      ObjectiveData::from("R4A2".to_owned(), true, true, false, false, 4),
      ObjectiveData::from("R4A3".to_owned(), true, true, false, false, 4),
      ObjectiveData::from("R4B1".to_owned(), true, false, false, false, 4),
      ObjectiveData::from("R4B2".to_owned(), true, true, false, false, 4),
      ObjectiveData::from("R4B3".to_owned(), true, true, false, false, 4),
      ObjectiveData::from("R4C1".to_owned(), true, false, false, false, 4),
      ObjectiveData::from("R4C2".to_owned(), true, true, false, false, 4),
      ObjectiveData::from("R4C3".to_owned(), true, true, false, false, 4),
      ObjectiveData::from("R4D1".to_owned(), true, false, false, false, 4),
      ObjectiveData::from("R4D2".to_owned(), true, true, false, false, 4),
      ObjectiveData::from("R4E1".to_owned(), true, false, false, false, 4),
    ]);
  }

  #[test]
  pub fn test_objectives_none_r4() {
    let objectives = GameRunObjective::AnyPercent.into_objective(GameRunRundown::Rundown4, 4);

    assert_eq!(objectives, vec![
      ObjectiveData::from("R4A1".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4A2".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4A3".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4B1".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4B2".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4B3".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4C1".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4C2".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4C3".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4D1".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4D2".to_owned(), false, false, false, false, 4),
      ObjectiveData::from("R4E1".to_owned(), false, false, false, false, 4),
    ]);
  }

}