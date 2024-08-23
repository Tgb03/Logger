use crate::{time::Time, log_handler::LogHandler};

/// Time Manager is a struct that allows u to manage
/// times. It allows adding to it and calculates several
/// important values.
pub struct TimeManager {

  started: bool,
  start_time: Time,
  times: Vec<Time>

}

impl TimeManager {

  /// creates a new Time Manager struct.
  pub fn new() -> TimeManager {
    TimeManager { started: false, start_time: Time::new(), times: Vec::new() }
  }

  pub fn new_with_time(time: Time) -> TimeManager {
    let mut tm: TimeManager = TimeManager::new();

    tm.add_time(time);

    return tm;
  }

  /// add time to Time Manager.
  pub fn add_time(&mut self, time: Time) {
    match self.started {
        true => self.times.push(time.sub(&self.start_time)),
        false => self.start_time = time,
    }

    self.started = true;
  }

  /// return times vector.
  pub fn get_times(&self) -> &Vec<Time> {
    return &self.times;
  }

  /// return splits vector.
  /// these are all the differences between the introduced.
  pub fn get_splits(&self) -> Vec<Time> {
    let mut splits: Vec<Time> = Vec::new();

    let mut current: &Time = &self.start_time;
    for time in &self.times {
      splits.push(time.sub(current));

      current = time;
    }

    return splits;
  }

}

impl LogHandler for TimeManager {
    
  fn parse_line(&mut self, line: &str) {
    todo!();  
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {

    let time_manager: TimeManager = TimeManager::new();

    assert_eq!(time_manager.start_time.get_stamp(), 0);
    assert_eq!(time_manager.started, false);
    assert_eq!(time_manager.start_time.get_stamp(), 0);

  }

  #[test]
  fn test_add() {

    let mut time_manager: TimeManager = TimeManager::new();
    time_manager.add_time(Time::from("00:00:01.000"));

    assert_eq!(time_manager.started, true);
    assert_eq!(time_manager.start_time.get_stamp(), 1000);

    time_manager.add_time(Time::from("00:00:03.000"));

    assert_eq!(time_manager.times.len(), 1);
    assert_eq!(time_manager.times[0].get_stamp(), 2000);

  }

  #[test]
  fn test_get_times() {

    let mut time_manager: TimeManager = TimeManager::new();
    time_manager.add_time(Time::from("00:00:00.000"));
    time_manager.add_time(Time::from("00:00:01.000"));
    time_manager.add_time(Time::from("00:00:03.000"));
    time_manager.add_time(Time::from("00:00:04.000"));
    time_manager.add_time(Time::from("00:00:10.000"));

    let vec = time_manager.get_times();
    
    assert_eq!(vec[0].get_stamp(), 1000);
    assert_eq!(vec[1].get_stamp(), 3000);
    assert_eq!(vec[2].get_stamp(), 4000);
    assert_eq!(vec[3].get_stamp(), 10000);

  }

  #[test]
  fn test_get_splits() {

    let mut time_manager: TimeManager = TimeManager::new();
    time_manager.add_time(Time::from("00:00:00.000"));
    time_manager.add_time(Time::from("00:00:01.000"));
    time_manager.add_time(Time::from("00:00:03.000"));
    time_manager.add_time(Time::from("00:00:04.000"));
    time_manager.add_time(Time::from("00:00:10.000"));

    let vec = time_manager.get_splits();
    
    assert_eq!(vec[0].get_stamp(), 1000);
    assert_eq!(vec[1].get_stamp(), 2000);
    assert_eq!(vec[2].get_stamp(), 1000);
    assert_eq!(vec[3].get_stamp(), 6000);

  }
}