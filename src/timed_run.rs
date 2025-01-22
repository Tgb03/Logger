
use crate::{objective_data::ObjectiveData, time::Time};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimedRun {

  #[serde(skip)] pub objective_data: ObjectiveData,
  
  win: bool,
  last_drop: Time,
  times: Vec<Time>,

  buffered_times_result: Vec<Time>,
  buffered_split_results: Vec<Time>,

}

impl TimedRun {

  pub fn new(name: String) -> TimedRun {
    TimedRun { 
      objective_data: ObjectiveData::from(name, false, false, false, false, 0),
      win: false,
      last_drop: Time::new(),
      times: Vec::new(),  

      buffered_times_result: Vec::new(),
      buffered_split_results: Vec::new(),
    }
  }

  pub fn get_split(&self, id: usize) -> Time {
    if id == 0 {
      return self.times[0];
    }

    return self.times[id].sub(&self.times[id - 1])
  }

  pub fn get_splits(&self) -> &Vec<Time> {
    &self.buffered_split_results
  }

  pub fn get_times(&self) -> &Vec<Time> {
    &self.buffered_times_result
  }

  fn calculate_vecs(&mut self) {
    // calculate the times
    self.buffered_times_result.clear();
    let times_end_id = if self.win { self.times.len() } else { self.times.len().saturating_sub(1) };
    for i in 0..times_end_id {
      self.buffered_times_result.push(match self.objective_data.early_drop {
        true => {self.times[i].sub(&self.last_drop)},
        false => self.times[i],
      })
    }
    
    // calculate the splits
    self.buffered_split_results.clear();
    if self.times.len() > 1 { self.buffered_split_results.push(self.times[0]); }
    let times_end_id = if self.win { self.times.len().saturating_sub(1) } else { self.times.len().saturating_sub(2) };
    for i in 0..times_end_id {
      self.buffered_split_results.push(self.times[i + 1].sub(&self.times[i]));
    }
  }

  pub fn get_time(&self) -> Time {
    if self.times.len() == 0 {
      return Time::new()
    }

    let time = self.times[self.times.len() - 1];
    if self.objective_data.early_drop { return time.sub(&self.last_drop) }
    
    time
  }

  pub fn push(&mut self, value: Time) {
    self.times.push(value);
    self.calculate_vecs();
  }

  pub fn len(&self) -> usize {
    self.times.len()
  }

  pub fn is_win(&self) -> bool {
    self.win
  }

  pub fn set_win(&mut self, win: bool) {
    self.win = win;
    self.calculate_vecs();
  }

  pub fn set_last_drop(&mut self, time: Time) {
    self.last_drop = time;
    self.calculate_vecs();
  }

}

pub trait GetByObjective {

  fn get_by_objective(&self, objective_data: &ObjectiveData) -> Option<&TimedRun>;

}

impl GetByObjective for [TimedRun] {
  fn get_by_objective(&self, objective_data: &ObjectiveData) -> Option<&TimedRun> {
    for it in self {
      if it.objective_data == *objective_data {
        return Some(it)
      }
    }

    None
  }
}
