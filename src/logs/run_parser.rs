use egui::ahash::HashSet;

use crate::run::{named_time::NamedTime, objectives::run_objective::RunObjective, time::Time, timed_run::LevelRun, traits::{Run, Timed}};

use super::{token_parser::TokenParserT, tokenizer::Token};

/// struct for parsing a single run
pub struct RunParser {

  start_time: Time,
  players: HashSet<u32>,

  is_done: bool,
  timed_run: LevelRun,
  run_objective: RunObjective,

  door_count: u8,
  bulk_count: u8,

}

impl RunParser {

  pub fn new(level_name: String) -> RunParser {
    RunParser {
      start_time: Time::default(),
      players: Default::default(),
      is_done: false,
      timed_run: LevelRun::default(),
      run_objective: RunObjective::from_name(level_name),
      door_count: 1,
      bulk_count: 1,
    }
  }

  /// check whether or not the run parser finished.
  pub fn is_done(&self) -> bool {
    self.is_done
  }

  pub fn get_result_mut(&mut self) -> &mut LevelRun {
    &mut self.timed_run
  }

}

impl Into<LevelRun> for RunParser {
  
  fn into(self) -> LevelRun {
    self.timed_run
  }

}


impl TokenParserT<LevelRun> for RunParser {

  fn into_result(&self) -> &LevelRun {
    &self.timed_run
  }
  
  fn parse_one_token(&mut self, (time, token): (Time, Token)) -> bool {

    if self.is_done { return true }

    // println!("parsed: {:?}", token);
    
    match token {
      Token::GameStarted => {
        self.start_time = time;
      }
      Token::PlayerDroppedInLevel(id) => {
        self.players.insert(id);
        self.run_objective.player_count = self.players.len() as u8;
        self.timed_run.set_objective(&self.run_objective);
      },
      Token::DoorOpen => {
        self.timed_run.add_split(NamedTime::new(time.sub(&self.start_time).sub(&self.timed_run.get_time()), format!("D_{:02}", self.door_count)));
        self.door_count += 1;
      },
      Token::BulkheadScanDone => {
        self.timed_run.add_split(NamedTime::new(time.sub(&self.start_time).sub(&self.timed_run.get_time()), format!("B_{:02}", self.bulk_count)));
        self.bulk_count += 1;
      },
      Token::SecondaryDone => { 
        self.run_objective.secondary = true;
        self.timed_run.set_objective(&self.run_objective);
      },
      Token::OverloadDone => { 
        self.run_objective.overload = true;
        self.timed_run.set_objective(&self.run_objective);
      },
      Token::GameEndWin => {
        self.timed_run.set_win(true);
        self.run_objective.player_count = self.players.len() as u8;
        self.is_done = true;
        self.timed_run.add_split(NamedTime::new(time.sub(&self.start_time).sub(&self.timed_run.get_time()), "WIN ".to_owned()));
        self.timed_run.set_objective(&self.run_objective);

        return true;
      },
      Token::GameEndLost | Token::GameEndAbort | Token::LogFileEnd => { 
        self.is_done = true; 
        self.run_objective.player_count = self.players.len() as u8; 
        self.timed_run.add_split(NamedTime::new(time.sub(&self.start_time).sub(&self.timed_run.get_time()), "LOSS".to_owned()));
        self.timed_run.set_objective(&self.run_objective);
        
        return true; 
      },
      Token::SelectExpedition(_) => { /* IGNORE TOKEN FOR EARLY DROP */ }
      _ => eprintln!("Failed to parse token {:?} in RunParser", token)
    }
    
    return false;
  }

  fn into_result_mut(&mut self) -> &mut LevelRun {
    &mut self.timed_run    
  }

}

#[cfg(test)]
mod tests {
    use crate::{logs::{token_parser::TokenParserT, tokenizer::Token}, run::{objectives::run_objective::RunObjective, time::Time, traits::{Run, Timed}} };

    use super::RunParser;


    #[test]
    pub fn test_base_game() {
      let tokens = vec![
        (Time::from("00:00:10.000").unwrap(), Token::GameStarted),
        (Time::from("00:00:10.000").unwrap(), Token::PlayerDroppedInLevel(1)),
        (Time::from("00:00:10.100").unwrap(), Token::PlayerDroppedInLevel(2)),
        (Time::from("00:01:12.135").unwrap(), Token::DoorOpen),
        (Time::from("00:03:12.198").unwrap(), Token::DoorOpen),
        (Time::from("00:04:06.000").unwrap(), Token::DoorOpen),
        (Time::from("00:14:12.135").unwrap(), Token::DoorOpen),
        (Time::from("00:16:11.890").unwrap(), Token::BulkheadScanDone),
        (Time::from("00:17:59.343").unwrap(), Token::GameEndWin),
      ];
      
      let result = RunParser::parse_all_tokens(
        tokens.into_iter(), 
        RunParser::new("R1C1".to_string())
      );
      
      assert_eq!(result.get_objective::<RunObjective>(), Some(
        RunObjective::from_name("R1C1".to_string())
          .with_player_count(2)
      ));
      assert_eq!(result.get_splits().map(|v| v.get_time()).collect::<Vec<Time>>(), vec![
        Time::from("00:01:02.135").unwrap(),
        Time::from("00:02:00.063").unwrap(),
        Time::from("00:00:53.802").unwrap(),
        Time::from("00:10:06.135").unwrap(),
        Time::from("00:01:59.755").unwrap(),
        Time::from("00:01:47.453").unwrap(),
      ]);
      assert_eq!(result.is_win(), true);
    }
  
    #[test]
    pub fn test_splits() {
      let tokens = vec![
        (Time::from("00:00:10.000").unwrap(), Token::GameStarted),
        (Time::from("00:00:10.000").unwrap(), Token::PlayerDroppedInLevel(1)),
        (Time::from("00:00:10.100").unwrap(), Token::PlayerDroppedInLevel(2)),
        (Time::from("00:00:10.110").unwrap(), Token::PlayerDroppedInLevel(3)),
        (Time::from("00:00:10.250").unwrap(), Token::PlayerDroppedInLevel(4)),
        (Time::from("00:01:12.135").unwrap(), Token::DoorOpen),
        (Time::from("00:03:12.198").unwrap(), Token::DoorOpen),
        (Time::from("00:04:06.000").unwrap(), Token::DoorOpen),
        (Time::from("00:14:12.135").unwrap(), Token::DoorOpen),
        (Time::from("00:16:11.890").unwrap(), Token::BulkheadScanDone),
        (Time::from("00:17:59.343").unwrap(), Token::GameEndWin),
      ];
      
      let result = RunParser::parse_all_tokens(
        tokens.into_iter(), 
        RunParser::new("R1C1".to_string())
      );
  
      let splits = result.get_splits().map(|v| v.get_time()).collect::<Vec<Time>>();
      assert_eq!(splits, vec![
        Time::from("00:01:02.135").unwrap(),
        Time::from("00:02:00.063").unwrap(),
        Time::from("00:00:53.802").unwrap(),
        Time::from("00:10:06.135").unwrap(),
        Time::from("00:01:59.755").unwrap(),
        Time::from("00:01:47.453").unwrap(),
      ])
    }
  
    #[test]
    pub fn test_overflow() {
  
      let tokens = vec![
        (Time::from("23:59:10.000").unwrap(), Token::GameStarted),
        (Time::from("23:59:10.000").unwrap(), Token::PlayerDroppedInLevel(1)),
        (Time::from("23:59:10.100").unwrap(), Token::PlayerDroppedInLevel(2)),
        (Time::from("23:59:10.110").unwrap(), Token::PlayerDroppedInLevel(3)),
        (Time::from("23:59:10.250").unwrap(), Token::PlayerDroppedInLevel(4)),
        (Time::from("00:00:12.135").unwrap(), Token::DoorOpen),
        (Time::from("00:00:12.197").unwrap(), Token::PlayerDroppedInLevel(4)),
        (Time::from("00:02:12.198").unwrap(), Token::DoorOpen),
        (Time::from("00:03:06.000").unwrap(), Token::DoorOpen),
        (Time::from("00:13:12.135").unwrap(), Token::DoorOpen),
        (Time::from("00:13:15.135").unwrap(), Token::SecondaryDone),
        (Time::from("00:15:11.890").unwrap(), Token::BulkheadScanDone),
        (Time::from("00:16:59.343").unwrap(), Token::GameEndWin),
      ];
      
      let result = RunParser::parse_all_tokens(
        tokens.into_iter(), 
        RunParser::new("R1C1".to_string())
      );
  
      let splits = result.get_splits().map(|v| v.get_time()).collect::<Vec<_>>();
      assert_eq!(splits, vec![
        Time::from("00:01:02.135").unwrap(),
        Time::from("00:02:00.063").unwrap(),
        Time::from("00:00:53.802").unwrap(),
        Time::from("00:10:06.135").unwrap(),
        Time::from("00:01:59.755").unwrap(),
        Time::from("00:01:47.453").unwrap(),
      ]);
      assert_eq!(result.get_objective::<RunObjective>().is_some_and(|v| v.secondary == true), true);
  
    }

}
