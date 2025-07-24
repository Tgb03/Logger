use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::{constants::LEVELS_ALL, error::ObjectiveParseError};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Hash, Serialize, Deserialize, Default, PartialOrd, Ord)]
pub enum GameRunRundown {
    #[default]
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

impl Display for GameRunRundown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameRunRundown::Rundown1 => write!(f, "Rundown1"),
            GameRunRundown::Rundown2 => write!(f, "Rundown2"),
            GameRunRundown::Rundown3 => write!(f, "Rundown3"),
            GameRunRundown::Rundown4 => write!(f, "Rundown4"),
            GameRunRundown::Rundown5 => write!(f, "Rundown5"),
            GameRunRundown::Rundown6 => write!(f, "Rundown6"),
            GameRunRundown::Rundown7 => write!(f, "Rundown7"),
            GameRunRundown::Rundown8 => write!(f, "Rundown8"),
            GameRunRundown::FullGame => write!(f, "FullGame"),
        }
    }
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

impl<'a> TryInto<GameRunRundown> for &'a str {
    type Error = ObjectiveParseError;

    fn try_into(self) -> Result<GameRunRundown, ObjectiveParseError> {
        match self {
            "Rundown1" => Ok(GameRunRundown::Rundown1),
            "Rundown2" => Ok(GameRunRundown::Rundown2),
            "Rundown3" => Ok(GameRunRundown::Rundown3),
            "Rundown4" => Ok(GameRunRundown::Rundown4),
            "Rundown5" => Ok(GameRunRundown::Rundown5),
            "Rundown6" => Ok(GameRunRundown::Rundown6),
            "Rundown7" => Ok(GameRunRundown::Rundown7),
            "Rundown8" => Ok(GameRunRundown::Rundown8),
            "FullGame" => Ok(GameRunRundown::FullGame),
            _ => Err(ObjectiveParseError::FailedParseIntoGameRundown),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::GameRunRundown;

    #[test]
    pub fn test_objectives_grr() {
        let objectives_1: &[&str] = GameRunRundown::Rundown1.into();
        let objectives_4: &[&str] = GameRunRundown::Rundown4.into();
        let objectives_7: &[&str] = GameRunRundown::Rundown7.into();

        assert_eq!(
            objectives_1,
            vec!["R1A1", "R1B1", "R1B2", "R1C1", "R1C2", "R1D1"]
        );
        assert_eq!(
            objectives_4,
            vec![
                "R4A1", "R4A2", "R4A3", "R4B1", "R4B2", "R4B3", "R4C1", "R4C2", "R4C3", "R4D1",
                "R4D2", "R4E1"
            ]
        );
        assert_eq!(
            objectives_7,
            vec![
                "R7A1", "R7B1", "R7B2", "R7B3", "R7C1", "R7C2", "R7C3", "R7D1", "R7D2", "R7E1"
            ]
        );
    }
}
