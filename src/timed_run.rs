
use crate::{objective_data::ObjectiveData, time::Time};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedRun {

  pub objective_data: ObjectiveData,
  pub win: bool,
  pub last_drop: Time,
  pub times: Vec<Time>,

}

impl TimedRun {

  pub fn new(name: String) -> TimedRun {
    TimedRun { 
      objective_data: ObjectiveData::from(name, false, false, false, false, 0),
      win: false,
      last_drop: Time::new(),
      times: Vec::new(),  
    }
  }

  pub fn get_split(&self, id: usize) -> Time {
    if id == 0 {
      return self.times[0];
    }

    return self.times[id].sub(&self.times[id - 1])
  }

  pub fn get_splits(&self) -> Vec<Time> {
    let mut result = Vec::new();

    if self.times.len() > 1 { result.push(self.times[0]); }
    let times_end_id = if self.win { self.times.len().saturating_sub(1) } else { self.times.len().saturating_sub(2) };
    for i in 0..times_end_id {
      result.push(self.times[i + 1].sub(&self.times[i]));
    }

    result
  }

  pub fn get_times(&self) -> Vec<Time> {
    let mut result = Vec::new();

    let times_end_id = if self.win { self.times.len() } else { self.times.len().saturating_sub(1) };
    for i in 0..times_end_id {
      result.push(match self.objective_data.early_drop {
        true => {self.times[i].sub(&self.last_drop)},
        false => self.times[i],
      })
    }

    result
  }

  pub fn get_time(&self) -> Time {
    if self.times.len() == 0 {
      return Time::new()
    }

    let time = self.times[self.times.len() - 1];
    if self.objective_data.early_drop { return time.sub(&self.last_drop) }
    
    time
  }

}

/*
impl PartialEq for TimedRun {
  fn eq(&self, other: &Self) -> bool {
    self.level_name == other.level_name && self.objective_data == other.objective_data 
  }
}
*/

