use crate::{objective_data::ObjectiveData, time::Time, timed_run::TimedRun};

use super::{levels::GameRunRundown, objectives::GameRunObjective};

pub struct RunManager {

  runs_done: Vec<TimedRun>,
  levels_required: Vec<ObjectiveData>,

  total_time: Time,

}

impl RunManager {

  pub fn finished_level(&mut self, run: TimedRun) {
    
    self.total_time = self.total_time.add(&run.get_time());
    let obj = &run.objective_data;

    match self.levels_required.iter().position(|o| o == obj) {
      Some(id) => { self.levels_required.remove(id); },
      None => {},
    }

    self.runs_done.push(run);
  
  }

  pub fn new(objective: GameRunObjective, rundown: GameRunRundown, player_count: u8) -> RunManager {
    Self {
      levels_required: objective.into_objective(rundown, player_count),
      runs_done: Vec::new(),
      total_time: Time::new(),
    }
  }

}
