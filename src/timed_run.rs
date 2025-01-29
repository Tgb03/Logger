
use std::{fmt::Display, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{game_runs::objectives::GameRunObjective, objective_data::ObjectiveData, time::Time};

pub type LevelRun = TimedRun<Time, ObjectiveData>;
pub type RundownRun = TimedRun<LevelRun, GameRunObjective>;

pub trait Timed {
  
  fn get_time(&self) -> Time;
  fn get_name(&self) -> Option<String>;

}

impl Timed for Time {
  fn get_time(&self) -> Time {
    *self
  }
  
  fn get_name(&self) -> Option<String> {
    None
  }
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Default)]
pub struct TimedRun<T, O>
where 
  T: Timed {

  splits: Vec<T>,
  times: Vec<Time>,
  total_time: Time,
  is_win: bool,

  objective: O,

}

impl<T, O> Timed for TimedRun<T, O>
where 
  T: Timed,
  O: Display, {
  
  fn get_time(&self) -> Time {
    self.total_time
  }

  fn get_name(&self) -> Option<String> {
    Some(format!("{}", self.get_objective()))    
  }

}

impl<T, O> TimedRun<T, O>
where 
  T: Timed {

  pub fn new(objective_data: O) -> Self {
    Self {
      splits: Vec::new(),
      times: Vec::new(),
      total_time: Time::default(),
      objective: objective_data,
      is_win: false,
    }
  }

  pub fn get_objective(&self) -> &O {
    &self.objective
  }

  pub fn get_objective_mut(&mut self) -> &mut O {
    &mut self.objective
  }

  pub fn add_split(&mut self, split: T) {
    self.total_time = self.total_time.add(&split.get_time());
    self.times.push(self.total_time);

    self.splits.push(split);
  }

  pub fn get_time(&self) -> Time {
    self.total_time
  }

  pub fn is_win(&self) -> bool {
    self.is_win
  }

  pub fn set_win(&mut self, win: bool) {
    self.is_win = win;
  }

  pub fn len(&self) -> usize {
    self.splits.len()
  }

  pub fn get_splits(&self) -> &Vec<T> {
    &self.splits
  }

  pub fn get_times(&self) -> &Vec<Time> {
    &self.times
  }

}
