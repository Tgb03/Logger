
use crate::{logs::tokenizer::Token, objective_data::ObjectiveData, time::Time};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TimedRun {

  pub level_name: String,
  pub objective_data: ObjectiveData,
  pub win: bool,
  pub last_drop: Time,
  times: Vec<Time>,

}

impl TimedRun {

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
    match self.objective_data.early_drop {
      false => self.times[self.times.len() - 1],
      true => self.times[self.times.len() - 1].sub(&self.last_drop)
    }
  }

  pub fn get_best_splits(&self, other: &TimedRun) -> Vec<Time> {
    let splits_self = self.get_splits();
    let mut splits_other = other.get_splits();

    for i in 0..splits_self.len() {
      if splits_self[i].is_greater_than(&splits_other[i]) {
        splits_other[i] = splits_other[i];
      }
    }

    splits_other
  }

}

/*
impl PartialEq for TimedRun {
  fn eq(&self, other: &Self) -> bool {
    self.level_name == other.level_name && self.objective_data == other.objective_data 
  }
}
*/

pub struct TimedRunParser {

  in_game: bool,
  players: Vec<u32>,
  level_name: String,
  game_start_time: Option<Time>,
  last_drop: Time,
  secondary_done: bool,
  overload_done: bool,
  early_drop: bool,
  
  times: Vec<Time>,
  results: Vec<TimedRun>,
  
  is_done: bool,

}

impl TimedRunParser {

  pub fn new() -> TimedRunParser {
    TimedRunParser {
      in_game: false,
      players: Vec::new(),
      level_name: String::new(),
      game_start_time: None,
      secondary_done: false,
      overload_done: false,
      early_drop: false,
      times: Vec::new(),
      results: Vec::new(),
      is_done: false,
      last_drop: Time::new(),
    }
  }

  pub fn parse_times(&mut self, tokens: Vec<(Time, Token)>) {
    
    for (time, token) in tokens {
      match token {
        Token::SelectExpedition(name) => { self.level_name = name; },
        Token::GameStarted => { 
          self.in_game = true;
          self.game_start_time = Some(time); 
        },
        Token::PlayerDroppedInLevel(id) => { 
          if self.in_game {
            if !self.players.contains(&id) {
              self.players.push(id);

              //println!("Got id: {}", id);
            }
            self.last_drop = time;
          }
        },
        Token::DoorOpen => { 
          if self.game_start_time != None {
            //self.early_drop = true;
            self.times.push(time.sub(&self.game_start_time.unwrap()));
          } 
          //println!("{} Added stamp: {:?} - {:?} = {:?}", self.times.len(), time, self.game_start_time, self.times[self.times.len() - 1]);
        },
        Token::BulkheadScanDone => { 
          self.times.push(time.sub(&self.game_start_time.unwrap()));
        },
        Token::SecondaryDone => {
          self.secondary_done = true;
        },
        Token::OverloadDone => {
          self.overload_done = true;
        }
        Token::GameEndWin => {
          self.times.push(time.sub(&self.game_start_time.unwrap()));

          let player_count = match u8::try_from(self.players.len()) {
            Ok(count) => count,
            Err(_) => 255,
          };

          self.results.push(TimedRun {
            level_name: self.level_name.clone(),
            objective_data: ObjectiveData::from(self.secondary_done, self.overload_done, self.early_drop, self.early_drop, player_count),
            times: self.times.clone(),
            win: true,
            last_drop: self.last_drop.sub(&self.game_start_time.unwrap()),
          });

          self.reset_state();
          
        },
        Token::GameEndLost | Token::GameEndAbort => {
          if self.times.len() == 0 { 
            self.reset_state();

            continue 
          }

          let player_count = match u8::try_from(self.players.len()) {
            Ok(count) => count,
            Err(_) => 255,
          };

          self.results.push(TimedRun {
            level_name: self.level_name.clone(),
            objective_data: ObjectiveData::from(self.secondary_done, self.overload_done, false, false, player_count),
            times: self.times.clone(),
            win: false,
            last_drop: self.last_drop.sub(&self.game_start_time.unwrap()),
          });

          self.reset_state();
        },
        Token::LogFileEnd => {
          self.is_done = true;
        }
      }
    }
  }

  pub fn get_results(self) -> Vec<TimedRun> {
    self.results
  }

  fn reset_state(&mut self) {
    self.in_game = false;
    self.players.clear();
    self.game_start_time = None;
    self.times.clear();
    self.last_drop = Time::new();
    self.secondary_done = false;
    self.overload_done = false;
    self.early_drop = false;
  }

}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_base_game() {
    let tokens = vec![
      (Time::from("00:00:00.000"), Token::SelectExpedition("R1C1".to_string())),
      (Time::from("00:00:10.000"), Token::GameStarted),
      (Time::from("00:00:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("00:00:10.100"), Token::PlayerDroppedInLevel(2)),
      (Time::from("00:00:10.110"), Token::PlayerDroppedInLevel(3)),
      (Time::from("00:00:10.250"), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:01:12.135"), Token::DoorOpen),
      (Time::from("00:03:12.198"), Token::DoorOpen),
      (Time::from("00:04:06.000"), Token::DoorOpen),
      (Time::from("00:14:12.135"), Token::DoorOpen),
      (Time::from("00:16:11.890"), Token::BulkheadScanDone),
      (Time::from("00:17:59.343"), Token::GameEndWin),
    ];

    let mut timed_run_parser = TimedRunParser::new();
    timed_run_parser.parse_times(tokens);
    let result = timed_run_parser.get_results();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].level_name, "R1C1");
    assert_eq!(result[0].objective_data, ObjectiveData::from(false, false, false, false, 4));
    assert_eq!(result[0].last_drop, Time::from("00:00:00.250"));
    assert_eq!(result[0].times, vec![
      Time::from("00:01:02.135"),
      Time::from("00:03:02.198"),
      Time::from("00:03:56.000"),
      Time::from("00:14:02.135"),
      Time::from("00:16:01.890"),
      Time::from("00:17:49.343"),
    ]);
    assert_eq!(result[0].win, true);
  }

  #[test]
  pub fn test_splits() {
    let tokens = vec![
      (Time::from("00:00:00.000"), Token::SelectExpedition("R1C1".to_string())),
      (Time::from("00:00:10.000"), Token::GameStarted),
      (Time::from("00:00:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("00:00:10.100"), Token::PlayerDroppedInLevel(2)),
      (Time::from("00:00:10.110"), Token::PlayerDroppedInLevel(3)),
      (Time::from("00:00:10.250"), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:01:12.135"), Token::DoorOpen),
      (Time::from("00:03:12.198"), Token::DoorOpen),
      (Time::from("00:04:06.000"), Token::DoorOpen),
      (Time::from("00:14:12.135"), Token::DoorOpen),
      (Time::from("00:16:11.890"), Token::BulkheadScanDone),
      (Time::from("00:17:59.343"), Token::GameEndWin),
    ];

    let mut timed_run_parser = TimedRunParser::new();
    timed_run_parser.parse_times(tokens);
    let result = timed_run_parser.get_results();

    let splits = result[0].get_splits();
    assert_eq!(splits, vec![
      Time::from("00:01:02.135"),
      Time::from("00:02:00.063"),
      Time::from("00:00:53.802"),
      Time::from("00:10:06.135"),
      Time::from("00:01:59.755"),
      Time::from("00:01:47.453"),
    ])
  }

  #[test]
  pub fn test_multiple_games() {
    let tokens = vec![
      (Time::from("00:00:00.000"), Token::SelectExpedition("R1C1".to_string())),
      (Time::from("00:00:10.000"), Token::GameStarted),
      (Time::from("00:00:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("00:00:10.100"), Token::PlayerDroppedInLevel(2)),
      (Time::from("00:00:10.110"), Token::PlayerDroppedInLevel(3)),
      (Time::from("00:00:10.250"), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:01:12.135"), Token::DoorOpen),
      (Time::from("00:03:12.198"), Token::DoorOpen),
      (Time::from("00:04:06.000"), Token::DoorOpen),
      (Time::from("00:17:59.343"), Token::GameEndAbort),
      (Time::from("01:00:10.000"), Token::GameStarted),
      (Time::from("01:00:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("01:00:10.100"), Token::PlayerDroppedInLevel(2)),
      (Time::from("01:00:10.110"), Token::PlayerDroppedInLevel(3)),
      (Time::from("01:00:10.250"), Token::PlayerDroppedInLevel(4)),
      (Time::from("01:01:12.135"), Token::DoorOpen),
      (Time::from("01:03:12.198"), Token::DoorOpen),
      (Time::from("01:04:06.000"), Token::DoorOpen),
      (Time::from("01:14:12.135"), Token::DoorOpen),
      (Time::from("01:16:11.890"), Token::BulkheadScanDone),
      (Time::from("01:17:59.343"), Token::GameEndWin),
      (Time::from("02:00:10.000"), Token::GameStarted),
      (Time::from("02:00:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("02:00:10.100"), Token::PlayerDroppedInLevel(2)),
      (Time::from("02:01:12.135"), Token::DoorOpen),
      (Time::from("02:03:12.198"), Token::DoorOpen),
      (Time::from("02:04:06.000"), Token::DoorOpen),
      (Time::from("02:14:12.135"), Token::DoorOpen),
      (Time::from("02:16:11.890"), Token::BulkheadScanDone),
      (Time::from("02:17:59.343"), Token::GameEndWin),
    ];

    let mut timed_run_parser = TimedRunParser::new();
    timed_run_parser.parse_times(tokens);
    let result = timed_run_parser.get_results();

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].level_name, "R1C1");
    assert_eq!(result[0].objective_data, ObjectiveData::from(false, false, false, false, 4));
    assert_eq!(result[0].times, vec![
      Time::from("00:01:02.135"),
      Time::from("00:03:02.198"),
      Time::from("00:03:56.000"),
    ]);
    assert_eq!(result[0].win, false);
    assert_eq!(result[1].level_name, "R1C1");
    assert_eq!(result[1].objective_data, ObjectiveData::from(false, false, false, false, 4));
    assert_eq!(result[1].times, vec![
      Time::from("00:01:02.135"),
      Time::from("00:03:02.198"),
      Time::from("00:03:56.000"),
      Time::from("00:14:02.135"),
      Time::from("00:16:01.890"),
      Time::from("00:17:49.343"),
    ]);
    assert_eq!(result[1].win, true);
  }

  #[test]
  pub fn test_overflow() {

    let tokens = vec![
      (Time::from("23:59:00.000"), Token::SelectExpedition("R1C1".to_string())),
      (Time::from("23:59:10.000"), Token::GameStarted),
      (Time::from("23:59:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("23:59:10.100"), Token::PlayerDroppedInLevel(2)),
      (Time::from("23:59:10.110"), Token::PlayerDroppedInLevel(3)),
      (Time::from("23:59:10.250"), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:00:12.135"), Token::DoorOpen),
      (Time::from("00:00:12.197"), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:02:12.198"), Token::DoorOpen),
      (Time::from("00:03:06.000"), Token::DoorOpen),
      (Time::from("00:13:12.135"), Token::DoorOpen),
      (Time::from("00:15:11.890"), Token::BulkheadScanDone),
      (Time::from("00:16:59.343"), Token::GameEndWin),
    ];

    let mut timed_run_parser = TimedRunParser::new();
    timed_run_parser.parse_times(tokens);
    let result = timed_run_parser.get_results();

    let splits = result[0].get_splits();
    assert_eq!(splits, vec![
      Time::from("00:01:02.135"),
      Time::from("00:02:00.063"),
      Time::from("00:00:53.802"),
      Time::from("00:10:06.135"),
      Time::from("00:01:59.755"),
      Time::from("00:01:47.453"),
    ])

  }
}
