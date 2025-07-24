use core::{logs::data::LevelDescriptor, run::{objectives::{objective_enum::ObjectiveEnum, run_objective::RunObjective}, split::NamedSplit, timed_run::LevelRun, traits::Run}, time::Time};

use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct TimedRun {
    name: LevelDescriptor,
    total_time: Time,
    player_count: u8,

    used_checkpoint: bool,
    is_win: bool,

    did_secondary: bool,
    did_overload: bool,

    splits: Vec<NamedSplit>,
}

impl Into<LevelRun> for TimedRun {
    fn into(self) -> LevelRun {
        let mut lr = LevelRun::default();
        let objective = RunObjective::from_name(format!("{}", self.name))
            .with_secondary(self.did_secondary)
            .with_overload(self.did_overload)
            .with_player_count(self.player_count);

        lr.set_objective(ObjectiveEnum::Run(objective));
        lr.set_win(self.is_win);

        for split in self.splits {
            lr.add_split(split);
        }

        lr
    }
}
