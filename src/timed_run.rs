
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

  pub fn get_splits(&self) -> Vec<Time> {
    let mut result = Vec::new();
    let times = self.get_times();

    if times.len() > 0 { result.push(times[0]); }
    for i in 0..times.len() - 1 {
      result.push(times[i + 1].sub(&times[i]));
    }

    result
  }

  pub fn get_times(&self) -> Vec<Time> {
    let mut result = Vec::new();

    for time in &self.times {
      result.push(match self.objective_data.early_drop {
        true => {time.sub(&self.last_drop)},
        false => *time,
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

