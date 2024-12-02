
use crate::{logs::tokenizer::Token, time::Time, timed_run::TimedRun};

pub struct TimedRunParser {

  level_name: String,
  start_time: Time,
  players: Vec<u32>,

}

impl TimedRunParser {

  pub fn new(level_name: String, start_time: Time) -> TimedRunParser {
    TimedRunParser { 
      players: Vec::new(),
      start_time,
      level_name
    }
  }

  pub fn set_name(&mut self, level_name: String) {
    self.level_name = level_name;
  }

  pub fn set_start_time(&mut self, time: Time) {
    self.start_time = time;
  }

  fn end_parse(&mut self, result: TimedRun) -> TimedRun {
    self.players.clear();
    self.start_time = Time::new();

    result
  }

  pub fn get_run<I: Iterator<Item=(Time, Token)>>(&mut self, tokens: &mut I) -> TimedRun {

    let mut result = TimedRun::new(self.level_name.clone());
    
    while let Some((time, token)) = tokens.next() {
      match token {
        Token::PlayerDroppedInLevel(id) => {
          if !self.players.contains(&id) {
            self.players.push(id);
            result.last_drop = time.sub(&self.start_time);
          }
        },
        Token::DoorOpen => result.times.push(time.sub(&self.start_time)),
        Token::BulkheadScanDone => result.times.push(time.sub(&self.start_time)),
        Token::SecondaryDone => result.objective_data.secondary = true,
        Token::OverloadDone => result.objective_data.overload = true,
        Token::GameEndWin => {
          result.times.push(time.sub(&self.start_time));
          result.win = true;

          result.objective_data.player_count = self.players.len() as u8;

          return self.end_parse(result);
        },
        Token::GameEndLost | Token::GameEndAbort | Token::LogFileEnd => {
          result.objective_data.player_count = self.players.len() as u8;

          return self.end_parse(result);
        },
        _ => panic!("{:?} token cannot be matched by TimedRunParser.", token),
      }
    }

    result.objective_data.player_count = self.players.len() as u8;

    self.end_parse(result)
  }

}


#[cfg(test)]
mod tests {
  use crate::{logs::tokenizer::Token, objective_data::ObjectiveData, time::Time};

  use super::*;

  #[test]
  pub fn test_base_game() {
    let tokens = vec![
      (Time::from("00:00:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("00:00:10.100"), Token::PlayerDroppedInLevel(2)),
      (Time::from("00:01:12.135"), Token::DoorOpen),
      (Time::from("00:03:12.198"), Token::DoorOpen),
      (Time::from("00:04:06.000"), Token::DoorOpen),
      (Time::from("00:14:12.135"), Token::DoorOpen),
      (Time::from("00:16:11.890"), Token::BulkheadScanDone),
      (Time::from("00:17:59.343"), Token::GameEndWin),
    ];

    let mut timed_run_parser = TimedRunParser::new("R1C1".to_string(), Time::from("00:00:10.000"));
    let result = timed_run_parser.get_run(&mut tokens.into_iter());

    assert_eq!(result.objective_data, ObjectiveData::from("R1C1".to_string(), false, false, false, false, 2));
    assert_eq!(result.last_drop, Time::from("00:00:00.100"));
    assert_eq!(result.times, vec![
      Time::from("00:01:02.135"),
      Time::from("00:03:02.198"),
      Time::from("00:03:56.000"),
      Time::from("00:14:02.135"),
      Time::from("00:16:01.890"),
      Time::from("00:17:49.343"),
    ]);
    assert_eq!(result.win, true);
  }

  #[test]
  pub fn test_splits() {
    let tokens = vec![
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

    let mut timed_run_parser = TimedRunParser::new("R1C1".to_string(), Time::from("00:00:10.000"));
    let result = timed_run_parser.get_run(&mut tokens.into_iter());

    let splits = result.get_splits();
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
  pub fn test_overflow() {

    let tokens = vec![
      (Time::from("23:59:10.000"), Token::PlayerDroppedInLevel(1)),
      (Time::from("23:59:10.100"), Token::PlayerDroppedInLevel(2)),
      (Time::from("23:59:10.110"), Token::PlayerDroppedInLevel(3)),
      (Time::from("23:59:10.250"), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:00:12.135"), Token::DoorOpen),
      (Time::from("00:00:12.197"), Token::PlayerDroppedInLevel(4)),
      (Time::from("00:02:12.198"), Token::DoorOpen),
      (Time::from("00:03:06.000"), Token::DoorOpen),
      (Time::from("00:13:12.135"), Token::DoorOpen),
      (Time::from("00:13:15.135"), Token::SecondaryDone),
      (Time::from("00:15:11.890"), Token::BulkheadScanDone),
      (Time::from("00:16:59.343"), Token::GameEndWin),
    ];

    let mut timed_run_parser = TimedRunParser::new("R1C1".to_string(), Time::from("23:59:10.000"));
    let result = timed_run_parser.get_run(&mut tokens.into_iter());

    let splits = result.get_splits();
    assert_eq!(splits, vec![
      Time::from("00:01:02.135"),
      Time::from("00:02:00.063"),
      Time::from("00:00:53.802"),
      Time::from("00:10:06.135"),
      Time::from("00:01:59.755"),
      Time::from("00:01:47.453"),
    ]);
    assert_eq!(result.objective_data.secondary, true);

  }
}
