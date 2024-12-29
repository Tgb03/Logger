
use std::{collections::{HashMap, HashSet}, env, path::{Path, PathBuf}};

use crate::{objective_data::ObjectiveData, time::Time, timed_run::TimedRun};

/// Save manager struct
/// 
/// This handles loading runs into memory and then ultimately saving them
#[derive(Default)]
pub struct SaveManager {

  loaded_runs: HashMap<String, Vec<TimedRun>>,

}

impl SaveManager {

  fn get_directory() -> PathBuf {
    Path::new(env!("HOME")).join("Appdata\\Locallow\\Tgb03\\GTFO Logger")
  }

  /// save a single timed run into RAM
  /// 
  /// duplicates are automatically removed.
  pub fn save(&mut self, timed_run: TimedRun) {

    if let Some(name) = self.save_no_remove_duplicates(timed_run) {
      self.remove_duplicates(name);
    }

  }

  fn save_no_remove_duplicates(&mut self, timed_run: TimedRun) -> Option<String> {
    if timed_run.len() == 1 { return None }

    let name = timed_run.objective_data.get_id();

    match self.loaded_runs.get_mut(&name) {
      Some(vec) => { 
        vec.push(timed_run);
      },
      None => { self.loaded_runs.insert(name.clone(), vec![timed_run]); },
    };

    return Some(name);
  }

  /// save multiple runs into the RAM memory.
  /// 
  /// duplicates are automatically removed.
  pub fn save_multiple(&mut self, timed_runs: Vec<TimedRun>) {
    
    let mut set = HashSet::new();

    for run in timed_runs {
      if let Some(name) = self.save_no_remove_duplicates(run) {
        set.insert(name);
      }
    }

    for name in set {
      self.remove_duplicates(name);
    }

  }

  fn remove_duplicates(&mut self, name: String) {
    if let Some(vec) = self.loaded_runs.remove(&name) {
      let set: HashSet<TimedRun> = HashSet::from_iter(vec);
      
      self.loaded_runs.insert(name, 
        set
        .into_iter()
        .collect());
    }
  }

  fn get_largest_stamp_count(runs: &Vec<TimedRun>) -> usize {
    let mut max = 0;
    for run in runs {
      max = max.max(run.get_splits().len()); 
    }
    
    max
  }

  /// returns all runs for the objective.
  pub fn get_runs(&mut self, objective_data: &ObjectiveData) -> Option<&mut Vec<TimedRun>> {
    let id = objective_data.get_id();

    if !self.loaded_runs.contains_key(&id){
      //self.load(objective_data);
    }

    self.loaded_runs.get_mut(&id)
  }

  /// returns all best splits for the objective.
  pub fn get_best_splits(&mut self, objective_data: &ObjectiveData) -> Vec<Time> {

    let id = objective_data.get_id();

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

  /// load from file the objective data.
  pub fn load(&mut self, objective_data: &ObjectiveData) {

    let _id = objective_data.get_id();
    todo!()
    
  }

}
