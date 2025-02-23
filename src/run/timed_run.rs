
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use super::{named_time::NamedTime, objectives::{game_objective::GameObjective, run_objective::RunObjective, Objective}, time::Time, traits::{Run, Timed}};

pub type LevelRun = TimedRun<NamedTime>;
pub type GameRun = TimedRun<LevelRun>;

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Default)]
pub struct TimedRun<T>
where 
  T: Timed {

  splits: Vec<T>,
  total_time: Time,
  is_win: bool,

  objective: String,

}

impl<T> Timed for TimedRun<T>
where 
  T: Timed {
  
  fn get_time(&self) -> Time {
    self.total_time
  }

  fn get_name(&self) -> &String {
    &self.objective
  }

  fn is_finished(&self) -> bool {
    self.is_win
  }

}

impl<T> TimedRun<T>
where 
  T: Timed {

  pub fn new<O: Objective>(objective_data: O) -> Self {
    Self {
      splits: Vec::new(),
      total_time: Time::default(),
      objective: objective_data.to_string(),
      is_win: false,
    }
  }

}

impl<T> Run for TimedRun<T>
where 
  T: Timed {

  fn is_win(&self) -> bool {
    self.is_win
  }

  fn len(&self) -> usize {
    self.splits.len()
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

  fn get_objective_str(&self) -> &String {
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
  
  fn get_time_for_split(&self, split_name: &String) -> Option<Time> {
    let mut time = Time::default();
    let mut exists = false;

    for split in &self.splits {
      if split.get_name() == split_name {
        time = time.add(&split.get_time());
        exists = true;
      }
    }

    match exists {
      true => Some(time),
      false => None,
    }
  }
  
  fn set_objective_str(&mut self, objective: String) {
    self.objective = objective;
  }
}

impl<T> TimedRun<T>
where T: Timed {

  pub fn get_splits<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
    self.splits.iter()
  }

  pub fn add_split(&mut self, split: T) {
    self.total_time = self.total_time.add(&split.get_time());
    self.splits.push(split);
  }

}

impl GameRun {

  pub fn get_split_for_objective(&self, objective: &RunObjective) -> (Time, bool) {

    let mut time = Time::default();
    let mut is_win = false;

    for run in self.get_splits() {
      let mut correct = true;
      // println!("A: {:?}", run.get_objective_str());
      let run_obj = run.get_objective::<RunObjective>().unwrap();

      if run_obj.level_name != objective.level_name { correct = false }
      if run_obj.secondary && !objective.secondary { correct = false }
      if run_obj.overload && !objective.overload { correct = false }
      if !run_obj.glitched && objective.glitched { correct = false }
      
      if correct {
        is_win = is_win.max(run.is_win);
        time = time.add(&run.get_time());
      }
    }

    (time, is_win)

  }

  pub fn validate(&mut self) {
    let objectives: Vec<RunObjective> = (&self.get_objective::<GameObjective>()
      .unwrap())
      .into();

    let mut is_good = true;
    for objective in objectives {
      let (_, win) = self.get_split_for_objective(&objective);

      if win == false {
        is_good = false;
      }
    }

    if is_good { self.set_win(true); }
  }  

}


#[cfg(test)]
mod tests {
  use crate::run::{named_time::NamedTime, objectives::run_objective::RunObjective, time::Time, traits::{Run, Timed}};
  use super::TimedRun;

  #[test]
  pub fn test_basic() {
    let mut run = TimedRun::<NamedTime>::new(RunObjective::default());

    run.add_split(NamedTime::new(Time::from("00:01:10.000").unwrap(), "D1".to_owned()));
    run.add_split(NamedTime::new(Time::from("00:01:10.000").unwrap(), "D1".to_owned()));
    run.add_split(NamedTime::new(Time::from("00:01:10.000").unwrap(), "D1".to_owned()));

    assert_eq!(run.get_time(), Time::from("00:03:30.000").unwrap());
    assert_eq!(run.len(), 3);
    for split in run.get_splits() {
      assert_eq!(*split, NamedTime::new(Time::from("00:01:10.000").unwrap(), "D1".to_owned()));
    }
  }

}