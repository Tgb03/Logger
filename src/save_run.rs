
use std::{collections::HashMap, env, path::{Path, PathBuf}};

use crate::{objective_data::ObjectiveData, time::Time, timed_run::TimedRun};

pub struct SaveManager {

  loaded_runs: HashMap<String, Vec<TimedRun>>,

}

impl SaveManager {

  pub fn new() -> SaveManager {
    SaveManager {
      loaded_runs: HashMap::new()
    }
  }

  fn get_directory() -> PathBuf {
    Path::new(env!("HOME")).join("Appdata\\Locallow\\Tgb03\\GTFO Logger")
  }

  fn get_name(objective_data: &ObjectiveData) -> String {
    let secondary = match objective_data.secondary {
      true => "_sec",
      false => ""
    };
    let overload = match objective_data.overload {
      true => "_ovrl",
      false => ""
    };
    let glitched = match objective_data.glitched {
      true => "_glitch",
      false => ""
    };
    let early_drop = match objective_data.early_drop {
      true => "_edrop",
      false => ""
    };

    //println!("Saved: {}{}{}{}{}_{}.save", objective_data.level_name, secondary, overload, glitched, early_drop, objective_data.get_player_count());
    format!("{}{}{}{}{}_{}.save", objective_data.level_name, secondary, overload, glitched, early_drop, objective_data.get_player_count())
  }

  pub fn save(&mut self, timed_run: TimedRun) {

    if timed_run.times.len() == 1 { return }

    let name = Self::get_name(&timed_run.objective_data);

    match self.loaded_runs.get_mut(&name) {
      Some(v) => v.push(timed_run),
      None => { self.loaded_runs.insert(name, vec![timed_run]); },
    };

  }

  pub fn save_multiple(&mut self, timed_runs: Vec<TimedRun>) {
    
    for run in timed_runs {
      self.save(run);
    }

  }

  fn get_largest_stamp_count(runs: &Vec<TimedRun>) -> usize {
    let mut max = 0;
    for run in runs {
      max = max.max(run.get_splits().len()); 
    }
    
    max
  }

  pub fn get_runs(&mut self, objective_data: &ObjectiveData) -> Option<&mut Vec<TimedRun>> {
    self.loaded_runs.get_mut(&Self::get_name(objective_data))
  }

  pub fn get_best_splits(&mut self, objective_data: &ObjectiveData) -> Vec<Time> {

    let id = Self::get_name(&objective_data);

    if !self.loaded_runs.contains_key(&id){
      //self.load(objective_data);
    }

    let empty = Vec::new();
    let runs = self.loaded_runs.get(&id).unwrap_or(&empty);

    let mut result = vec![Time::max(); Self::get_largest_stamp_count(runs)];
    for run in runs {
      for (id, time) in run.get_splits().iter().enumerate() {
        result[id] = result[id].min(time);
      }
    }

    result
  }

  pub fn load(&mut self, objective_data: &ObjectiveData) {

    let id = Self::get_name(objective_data);
    todo!()
    
  }

}
