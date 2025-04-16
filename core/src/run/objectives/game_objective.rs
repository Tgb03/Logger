use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::run::objectives::constants::OPTIONALS_ALL;

use super::{
    Objective, error::ObjectiveParseError, game_run_objective::GameRunObjective,
    game_run_rundown::GameRunRundown, run_objective::RunObjective,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct GameObjective {
    pub rundown: GameRunRundown,
    pub objective: GameRunObjective,
    pub player_count: u8,
}

impl Display for GameObjective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}_{}_{}.rsave",
            self.rundown, self.objective, self.player_count
        )
    }
}

impl<'a> TryFrom<&'a str> for GameObjective {
    type Error = ObjectiveParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut obj = GameObjective::default();

        if !value.ends_with(".rsave") {
            return Err(ObjectiveParseError::IncompatibleType(
                value[value.find('.').unwrap_or_default()..value.len()].to_owned(),
            ));
        }

        let mut split = value.trim_end_matches(".rsave").split('_');

        obj.rundown = match split.next().unwrap_or_default().try_into() {
            Err(e) => return Err(e),
            Ok(r) => r,
        };
        obj.objective = match split.next().unwrap_or_default().try_into() {
            Err(e) => return Err(e),
            Ok(o) => o,
        };
        obj.player_count = split
            .next()
            .unwrap_or_default()
            .parse::<u8>()
            .unwrap_or_default();

        Ok(obj)
    }
}

impl Objective for GameObjective {
    fn with_player_count(mut self, player_count: u8) -> Self {
        self.player_count = player_count;

        self
    }

    fn get_player_count(&self) -> u8 {
        self.player_count
    }

    fn get_name(&self) -> Option<&String> {
        None
    }
}

impl Into<Vec<RunObjective>> for &GameObjective {
    fn into(self) -> Vec<RunObjective> {
        let levels: &[&str] = self.rundown.clone().into();
        let objectives: &[(bool, bool)] = match self.rundown {
            GameRunRundown::Rundown1 => &OPTIONALS_ALL[0..6],
            GameRunRundown::Rundown2 => &OPTIONALS_ALL[6..16],
            GameRunRundown::Rundown3 => &OPTIONALS_ALL[16..23],
            GameRunRundown::Rundown4 => &OPTIONALS_ALL[23..35],
            GameRunRundown::Rundown5 => &OPTIONALS_ALL[35..48],
            GameRunRundown::Rundown6 => &OPTIONALS_ALL[48..61],
            GameRunRundown::Rundown7 => &OPTIONALS_ALL[61..71],
            GameRunRundown::Rundown8 => &OPTIONALS_ALL[71..83],
            GameRunRundown::FullGame => &OPTIONALS_ALL,
        };

        let mut result = Vec::new();

        for (level, objective) in levels.iter().zip(objectives) {
            let mut obj_data = RunObjective::from_name(level.to_string());

            if objective.0 {
                obj_data = obj_data.with_secondary(true)
            }
            if objective.1 {
                obj_data = obj_data.with_overload(true)
            }

            result.push(obj_data.with_player_count(self.player_count));
        }

        result
    }
}

impl GameObjective {
    pub fn get_rundown(&self) -> &GameRunRundown {
        &self.rundown
    }

    pub fn get_objectives(&self) -> &GameRunObjective {
        &self.objective
    }

    pub fn get_mut_rundown(&mut self) -> &mut GameRunRundown {
        &mut self.rundown
    }

    pub fn get_mut_objectives(&mut self) -> &mut GameRunObjective {
        &mut self.objective
    }
}

#[cfg(test)]
mod tests {
    use crate::run::objectives::{
        game_objective::GameObjective, game_run_objective::GameRunObjective,
        game_run_rundown::GameRunRundown,
    };

    #[test]
    pub fn test_objectives() {
        let mut run_obj = GameObjective::default();
        run_obj.player_count = 2;
        run_obj.objective = GameRunObjective::FullPercent;

        assert_eq!(
            TryInto::<GameObjective>::try_into("Rundown1_100%_2.rsave"),
            Ok(run_obj.clone())
        );
        assert_eq!(run_obj.to_string(), "Rundown1_100%_2.rsave");

        run_obj.rundown = GameRunRundown::Rundown3;

        assert_eq!(
            TryInto::<GameObjective>::try_into("Rundown3_100%_2.rsave"),
            Ok(run_obj.clone())
        );
        assert_eq!(run_obj.to_string(), "Rundown3_100%_2.rsave");
    }
}
