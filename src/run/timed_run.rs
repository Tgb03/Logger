
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use super::{objectives::Objective, time::Time, traits::{Run, Timed}};

pub type LevelRun = TimedRun<Time>;
pub type GameRun = TimedRun<TimedRun<Time>>;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Default)]
pub struct TimedRun<T>
where 
  T: Timed {

  splits: Vec<T>,
  times: Vec<Time>,
  total_time: Time,
  is_win: bool,

  #[serde(skip)] objective: String,

}

impl<T> Timed for TimedRun<T>
where 
  T: Timed {
  
  fn get_time(&self) -> Time {
    self.total_time
  }

  fn get_name(&self) -> Option<&String> {
    Some(&self.objective)
  }

}

impl<T> TimedRun<T>
where 
  T: Timed {

  pub fn new<O: Objective>(objective_data: O) -> Self {
    Self {
      splits: Vec::new(),
      times: Vec::new(),
      total_time: Time::default(),
      objective: objective_data.to_string(),
      is_win: false,
    }
  }

}

impl<T> Run for TimedRun<T>
where 
  T: Timed {

  fn get_times(&self) -> &Vec<Time> {
    &self.times
  }

  fn is_win(&self) -> bool {
    self.is_win
  }

  fn len(&self) -> usize {
    self.splits.len() - if self.is_win { 0 } else { 1 }
  }

  fn get_objective<O: Objective>(&self) -> Option<O> {
    let v: Result<O, _> = self.objective.as_str().try_into();

    match v {
      Ok(o) => Some(o),
      Err(_) => None,
    }
  }

  fn set_objective<O: Objective>(&mut self, objective: &O) {
    self.objective = objective.to_string();
  }

  fn get_objective_str(&self) -> &str {
    &self.objective
  }
  
  fn set_win(&mut self, is_win: bool) {
    self.is_win = is_win;
  }
  
  fn get_splits<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Timed> + 'a> {
    Box::new(
      self.splits.iter().map(|v| v as &dyn Timed)
    )
  }
}

impl<T> TimedRun<T>
where T: Timed {

  pub fn get_splits<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
    self.splits.iter()
  }

  pub fn add_split(&mut self, split: T) {
    self.total_time = self.total_time.add(&split.get_time());
    self.times.push(self.total_time.clone());
    self.splits.push(split);
  }
}


#[cfg(test)]
mod tests {
  use crate::run::{objectives::run_objective::RunObjective, time::Time, traits::{Run, Timed}};
  use super::TimedRun;

  #[test]
  pub fn test_basic() {
    let mut run = TimedRun::<Time>::new(RunObjective::default());

    run.add_split(Time::from("00:01:10.000"));
    run.add_split(Time::from("00:01:10.000"));
    run.add_split(Time::from("00:01:10.000"));

    assert_eq!(run.get_time(), Time::from("00:03:30.000"));
    assert_eq!(run.len(), 3);
    let it = run.get_times();
    assert_eq!(it.get(0).map(|t| t.get_time()), Some(Time::from("00:01:10.000")));
    assert_eq!(it.get(1).map(|t| t.get_time()), Some(Time::from("00:02:20.000")));
    assert_eq!(it.get(2).map(|t| t.get_time()), Some(Time::from("00:03:30.000")));
    for split in run.get_splits() {
      assert_eq!(*split, Time::from("00:01:10.000"));
    }
  }

}