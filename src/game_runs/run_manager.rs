use crate::{objective_data::ObjectiveData, time::Time, timed_run::TimedRun};

use super::{levels::GameRunRundown, objectives::GameRunObjective};

pub struct RunManager {

  runs_done: Vec<TimedRun>,
  levels_required: Vec<ObjectiveData>,

  total_time: Time,

  objective_string: String,

  game_rundown: GameRunRundown,
  game_obj: GameRunObjective,
  player_count: u8,

}

impl Into<(Vec<TimedRun>, Time, bool)> for RunManager {
  fn into(self) -> (Vec<TimedRun>, Time, bool) {
    (self.runs_done, self.total_time, self.levels_required.is_empty())  
  }
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
      objective_string: format!("{}_{}_{}p", objective, rundown, player_count),
      levels_required: objective.clone().into_objective(rundown.clone(), player_count),
      runs_done: Vec::new(),
      total_time: Time::new(),
      game_obj: objective,
      game_rundown: rundown,
      player_count,
    }
  }

  pub fn get_objective(&self) -> &String {
    &self.objective_string
  }

  pub fn get_next_level(&self) -> Option<String> {
    self.levels_required.get(0).map(|obj| obj.get_id())
  }

  pub fn len(&self) -> usize {
    self.levels_required.len()
  }

  pub fn get_last_n_runs(&self, size: usize) -> &[TimedRun] { 
    let size = size.min(self.runs_done.len());
    let left = self.runs_done.len() - size;

    &self.runs_done[left..size]
  }

  pub fn get_game_rundown(&self) -> &GameRunRundown {
    &self.game_rundown
  }

  pub fn get_game_objective(&self) -> &GameRunObjective {
    &self.game_obj
  }

  pub fn get_mut_game_rundown(&mut self) -> &mut GameRunRundown {
    &mut self.game_rundown
  }

  pub fn get_mut_game_objective(&mut self) -> &mut GameRunObjective {
    &mut self.game_obj
  }

  pub fn get_player_count(&self) -> u8 {
    self.player_count
  } 
}
