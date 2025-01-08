
use std::{collections::{HashMap, HashSet}, fs, path::{Path, PathBuf}};

use crate::{objective_data::ObjectiveData, time::Time, timed_run::TimedRun};

/// Save manager struct
/// 
/// This handles loading runs into memory and then ultimately saving them
#[derive(Default)]
pub struct SaveManager {

  loaded_runs: HashMap<String, Vec<TimedRun>>,
  best_splits: HashMap<String, Vec<Time>>,

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

    let empty = Vec::new();
    let runs = self.loaded_runs.get(&name).unwrap_or(&empty);

    let mut result = vec![Time::max(); Self::get_largest_stamp_count(runs)];
    for run in runs {
      for (id, time) in run.get_splits().iter().enumerate() {
        result[id] = result[id].min(time);
      }
    }
    self.best_splits.insert(name.clone(), result);

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

  // returns the world record run for a level
  pub fn get_best_run(&self, objective_data: &ObjectiveData) -> Option<&TimedRun> {
    let id = objective_data.get_id();

    match self.loaded_runs.get(&id) {
      Some(runs) => {
        let mut best_run = None;
        let mut best_time = Time::max();

        for timed_run in runs {
          if timed_run.get_time().is_smaller_than(&best_time) && timed_run.is_win() {
            best_run = Some(timed_run);
            best_time = timed_run.get_time();
          }
        }

        best_run
      },
      None => None,
    }
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
  pub fn get_best_splits(&self, objective_data: &ObjectiveData) -> Option<&Vec<Time>> {
    
    self.best_splits.get(&objective_data.get_id())

  }

  /// load all runs that were saved to folder
  pub fn load_all_runs(&mut self) {

    let file_path = Self::get_directory();
    let paths = fs::read_dir(file_path).unwrap();
    
    for path in paths {
      if let Ok(entry) = path {

        if entry.file_name().into_string().unwrap().contains(".save") {
          self.load(&ObjectiveData::from_id(&entry.file_name().into_string().unwrap()));
        }

      }
    }

  }

  /// optimize these runs by removing all that do not hold
  /// important information
  /// 
  /// if the run is not world record or has a best split it is removed
  pub fn optimize_obj(&mut self, objective_data: &ObjectiveData) {
    let best_splits = match self.get_best_splits(objective_data).clone() {
      Some(v) => v.clone(),
      None => Vec::new(),
    };
    let best_time = match self.get_best_run(objective_data) {
        Some(run) => Some(run.get_time()),
        None => None,
    };
    let mut for_deletions = Vec::new();

    if let Some(runs) = self.get_runs(objective_data) {

      for (r_id, run) in runs.into_iter().enumerate() {
        let mut is_valid = false;
        if best_time.is_some_and(|t| { run.get_time() == t }) {
          continue;
        }

        for (id, time) in run.get_splits().into_iter().enumerate() {
          if *time == best_splits[id] {
            is_valid = true;
            break;
          }
        }

        if !is_valid {
          for_deletions.push(r_id);
        }
      }
      
      for elem in for_deletions.iter().rev() {
        runs.remove(*elem);
      }
      
    }
  }

  /// load from file the objective data.
  pub fn load(&mut self, objective_data: &ObjectiveData) {

    let id = objective_data.get_id();
    let file_path = Self::get_directory()
      .join(id.clone());

    match std::fs::read(file_path) {
      Ok(binary_data) => {

        let mut vec: Vec<TimedRun> = match bincode::deserialize(&binary_data) {
            Ok(vec) => vec,
            Err(_) => Vec::new(),
        };

        vec.iter_mut().for_each(|r| r.objective_data = objective_data.clone());

        //println!("Added vec with size: {}, {}", vec.len(), binary_data.len());
        self.save_multiple(vec);
      },
      Err(e) => {
        eprintln!("{:?}", e);
      },
    }
  }

  /// save one objective data to pc folder
  pub fn save_to_file(&self, objective_data: &ObjectiveData) {
    let id = objective_data.get_id();
    let file_path = Self::get_directory()
        .join(id.clone());

    let empty = Vec::new();
    if let Ok(bin_data) = bincode::serialize(self.loaded_runs.get(&id).unwrap_or(&empty)) {
      //println!("Saved vec with size: {}: {}", vec.len(), bin_data.len());
      let _ = std::fs::write(file_path, &bin_data);
    }
  }

  /// save all loaded runs to files
  pub fn save_to_files(&self) {
    for (key, vec) in &self.loaded_runs {
      let file_path = Self::get_directory()
        .join(key);

      
      if let Ok(bin_data) = bincode::serialize(&vec) {
        //println!("Saved vec with size: {}: {}", vec.len(), bin_data.len());
        let _ = std::fs::write(file_path, &bin_data);
      }
    }
  }

  pub fn get_all_objectives(&self) -> Vec<String> {
    let mut v = self.loaded_runs.keys().cloned().collect::<Vec<String>>();
    v.sort();
    v
  }

}
