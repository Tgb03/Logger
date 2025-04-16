use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectiveParseError {
    NoName,
    NoPlayerCount,
    FailedParseIntoGameObjective,
    FailedParseIntoGameRundown,
    IncompatibleType(String),
}

impl<'a> Display for ObjectiveParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectiveParseError::IncompatibleType(obj_type) => {
                write!(f, "Incompatible type passed: {}", obj_type)
            }
            ObjectiveParseError::FailedParseIntoGameObjective => {
                write!(f, "Failed to parse &str into GameRunObjective")
            }
            ObjectiveParseError::FailedParseIntoGameRundown => {
                write!(f, "Failed to parse &str into GameRunRundown")
            }
            ObjectiveParseError::NoName => write!(f, "No name given to objective"),
            ObjectiveParseError::NoPlayerCount => write!(f, "No player count given to objective"),
        }
    }
}
