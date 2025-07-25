use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{
    Objective, error::ObjectiveParseError, game_objective::GameObjective,
    run_objective::RunObjective,
};

#[derive(Hash, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectiveEnum {
    Run(RunObjective),
    Game(GameObjective),
}

impl Default for ObjectiveEnum {
    fn default() -> Self {
        ObjectiveEnum::Run(RunObjective::default())
    }
}

impl Display for ObjectiveEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectiveEnum::Run(run_objective) => run_objective.fmt(f),
            ObjectiveEnum::Game(game_run_objective) => game_run_objective.fmt(f),
        }
    }
}

impl<'a> TryFrom<&'a str> for ObjectiveEnum {
    type Error = ObjectiveParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.ends_with(".save") {
            return match value.try_into() {
                Ok(v) => Ok(ObjectiveEnum::Run(v)),
                Err(e) => Err(e),
            };
        }

        if value.ends_with(".rsave") {
            return match value.try_into() {
                Ok(v) => Ok(ObjectiveEnum::Game(v)),
                Err(e) => Err(e),
            };
        }

        return Err(ObjectiveParseError::IncompatibleType(
            value[value.find('.').unwrap_or_default()..value.len()].to_owned(),
        ));
    }
}

impl Objective for ObjectiveEnum {
    fn with_player_count(self, player_count: u8) -> Self {
        match self {
            ObjectiveEnum::Run(run_objective) => {
                ObjectiveEnum::Run(run_objective.with_player_count(player_count))
            }
            ObjectiveEnum::Game(game_objective) => {
                ObjectiveEnum::Game(game_objective.with_player_count(player_count))
            }
        }
    }

    fn get_player_count(&self) -> u8 {
        match self {
            ObjectiveEnum::Run(run_objective) => run_objective.get_player_count(),
            ObjectiveEnum::Game(game_objective) => game_objective.get_player_count(),
        }
    }

    fn get_name(&self) -> Option<&String> {
        match self {
            ObjectiveEnum::Run(run_objective) => run_objective.get_name(),
            ObjectiveEnum::Game(game_objective) => game_objective.get_name(),
        }
    }
}

impl ObjectiveEnum {
    pub fn as_level_run(&self) -> Option<&RunObjective> {
        match self {
            ObjectiveEnum::Run(run_objective) => Some(run_objective),
            _ => None,
        }
    }
}
