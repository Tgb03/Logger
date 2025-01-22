use std::fmt::Display;
use strum_macros::EnumIter;

const LEVELS_ALL: [&str; 83] = [
  "R1A1", "R1B1", "R1B2", "R1C1", "R1C2", "R1D1", 
  "R2A1", "R2B1", "R2B2", "R2B3", "R2B4", "R2C1", "R2C2", "R2D1", "R2D2", "R2E1", 
  "R3A1", "R3A2", "R3A3", "R3B1", "R3B2", "R3C1", "R3D1", 
  "R4A1", "R4A2", "R4A3", "R4B1", "R4B2", "R4B3", "R4C1", "R4C2", "R4C3", "R4D1", "R4D2", "R4E1", 
  "R5A1", "R5A2", "R5A3", "R5B1", "R5B2", "R5B3", "R5B4", "R5C1", "R5C2", "R5C3", "R5D1", "R5D2", "R5E1", 
  "R6A1", "R6AX", "R6B1", "R6B2", "R6BX", "R6C1", "R6C2", "R6C3", "R6CX", "R6D1", "R6D2", "R6D3", "R6D4", 
  "R7A1", "R7B1", "R7B2", "R7B3", "R7C1", "R7C2", "R7C3", "R7D1", "R7D2", "R7E1", 
  "R8A1", "R8A3", "R8B1", "R8B2", "R8B3", "R8B4", "R8C1", "R8C3", "R8D1", "R8D3", "R8E1", "R8E3"
];


#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum GameRunRundown {

  Rundown1,
  Rundown2,
  Rundown3,
  Rundown4,
  Rundown5,
  Rundown6,
  Rundown7,
  Rundown8,

  FullGame,

}

impl Into<&[&str]> for GameRunRundown {
  
  fn into(self) -> &'static [&'static str] {

    match self {
      GameRunRundown::Rundown1 => &LEVELS_ALL[0..6],
      GameRunRundown::Rundown2 => &LEVELS_ALL[6..16],
      GameRunRundown::Rundown3 => &LEVELS_ALL[16..23],
      GameRunRundown::Rundown4 => &LEVELS_ALL[23..35],
      GameRunRundown::Rundown5 => &LEVELS_ALL[35..48],
      GameRunRundown::Rundown6 => &LEVELS_ALL[48..61],
      GameRunRundown::Rundown7 => &LEVELS_ALL[61..71],
      GameRunRundown::Rundown8 => &LEVELS_ALL[71..83],
      GameRunRundown::FullGame => &LEVELS_ALL,
    }
    
  }

}

impl Display for GameRunRundown {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GameRunRundown::Rundown1 => write!(f, "Rundown 1"),
      GameRunRundown::Rundown2 => write!(f, "Rundown 2"),
      GameRunRundown::Rundown3 => write!(f, "Rundown 3"),
      GameRunRundown::Rundown4 => write!(f, "Rundown 4"),
      GameRunRundown::Rundown5 => write!(f, "Rundown 5"),
      GameRunRundown::Rundown6 => write!(f, "Rundown 6"),
      GameRunRundown::Rundown7 => write!(f, "Rundown 7"),
      GameRunRundown::Rundown8 => write!(f, "Rundown 8"),
      GameRunRundown::FullGame => write!(f, "Full Game"),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::game_runs::levels::GameRunRundown;

  #[test]
  pub fn test_objectives_grr() {
    let objectives_1: &[&str] = GameRunRundown::Rundown1.into();
    let objectives_4: &[&str] = GameRunRundown::Rundown4.into();
    let objectives_7: &[&str] = GameRunRundown::Rundown7.into();

    assert_eq!(objectives_1, vec!["R1A1", "R1B1", "R1B2", "R1C1", "R1C2", "R1D1"]);
    assert_eq!(objectives_4, vec!["R4A1", "R4A2", "R4A3", "R4B1", "R4B2", "R4B3", "R4C1", "R4C2", "R4C3", "R4D1", "R4D2", "R4E1"]);
    assert_eq!(objectives_7, vec!["R7A1", "R7B1", "R7B2", "R7B3", "R7C1", "R7C2", "R7C3", "R7D1", "R7D2", "R7E1"]);
  
  }

}
