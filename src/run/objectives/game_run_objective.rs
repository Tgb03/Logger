use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::error::ObjectiveParseError;



#[derive(Clone, Debug, PartialEq, Eq, EnumIter, Hash, Serialize, Deserialize, Default)]
pub enum GameRunObjective {

  #[default] AnyPercent,
  FullPercent,

}

impl Display for GameRunObjective {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      GameRunObjective::AnyPercent => write!(f, "Any%"),
      GameRunObjective::FullPercent => write!(f, "100%"),
    }
  }
}

impl<'a> TryInto<GameRunObjective> for &'a str {
  type Error = ObjectiveParseError;

  fn try_into(self) -> Result<GameRunObjective, ObjectiveParseError> {
    match self {
      "Any%" => Ok(GameRunObjective::AnyPercent),
      "100%" => Ok(GameRunObjective::FullPercent),
      _ => Err(ObjectiveParseError::FailedParseIntoGameObjective)
    }
  }
}
