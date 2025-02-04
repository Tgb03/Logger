use egui::ahash::HashSet;

use crate::run::{objectives::run_objective::RunObjective, time::Time, timed_run::LevelRun, traits::{Run, Timed}};

use super::{token_parser::TokenParserT, tokenizer::Token};

/// struct for parsing a single run
pub struct RunParser {

  start_time: Time,
  players: HashSet<u32>,

  is_done: bool,
  timed_run: LevelRun,
  run_objective: RunObjective,

}

impl RunParser {

  pub fn new(level_name: String, start_time: Time) -> RunParser {
    RunParser {
      start_time,
      players: Default::default(),
      is_done: false,
      timed_run: LevelRun::default(),
      run_objective: RunObjective::from_name(level_name)
    }
  }

  /// check whether or not the run parser finished.
  pub fn is_done(&self) -> bool {
    self.is_done
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
      Token::PlayerDroppedInLevel(id) => {
        self.players.insert(id);
        self.run_objective.player_count = self.players.len() as u8;
      },
      Token::DoorOpen | Token::BulkheadScanDone => {
        self.timed_run.add_split(time.sub(&self.start_time).sub(&self.timed_run.get_time()));
      },
      Token::SecondaryDone => self.run_objective.secondary = true,
      Token::OverloadDone => self.run_objective.overload = true,
      Token::GameEndWin => {
        self.timed_run.set_win(true);
        self.run_objective.player_count = self.players.len() as u8;
        self.is_done = true;
        self.timed_run.add_split(time.sub(&self.start_time).sub(&self.timed_run.get_time()));
        self.timed_run.set_objective(&self.run_objective);

        return true;
      },
      Token::GameEndLost | Token::GameEndAbort | Token::LogFileEnd => { 
        self.is_done = true; 
        self.run_objective.player_count = self.players.len() as u8; 
        self.timed_run.add_split(time.sub(&self.start_time).sub(&self.timed_run.get_time()));
        self.timed_run.set_objective(&self.run_objective);
        
        return true; 
      },
      Token::SelectExpedition(_) => { /* IGNORE TOKEN FOR EARLY DROP */ }
      _ => eprintln!("Failed to parse token {:?} in RunParser", token)
    }
    
    return false;
  }

}

#[cfg(test)]
mod tests {
    use crate::{logs::{token_parser::TokenParserT, tokenizer::Token}, run::{objectives::run_objective::RunObjective, time::Time, traits::Run} };

    use super::RunParser;


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
      
      let result = RunParser::parse_all_tokens(
        tokens.into_iter(), 
        RunParser::new("R1C1".to_string(), Time::from("00:00:10.000"))
      );
      
      assert_eq!(result.get_objective::<RunObjective>(), Some(
        RunObjective::from_name("R1C1".to_string())
          .with_player_count(2)
      ));
      assert_eq!(result.get_times(), &vec![
        Time::from("00:01:02.135"),
        Time::from("00:03:02.198"),
        Time::from("00:03:56.000"),
        Time::from("00:14:02.135"),
        Time::from("00:16:01.890"),
        Time::from("00:17:49.343"),
      ]);
      assert_eq!(result.is_win(), true);
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
      
      let result = RunParser::parse_all_tokens(
        tokens.into_iter(), 
        RunParser::new("R1C1".to_string(), Time::from("00:00:10.000"))
      );
  
      let splits = result.get_splits().collect::<Vec<&Time>>();
      assert_eq!(splits, vec![
        &Time::from("00:01:02.135"),
        &Time::from("00:02:00.063"),
        &Time::from("00:00:53.802"),
        &Time::from("00:10:06.135"),
        &Time::from("00:01:59.755"),
        &Time::from("00:01:47.453"),
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
      
      let result = RunParser::parse_all_tokens(
        tokens.into_iter(), 
        RunParser::new("R1C1".to_string(), Time::from("23:59:10.000"))
      );
  
      let splits = result.get_splits().collect::<Vec<_>>();
      assert_eq!(splits, vec![
        &Time::from("00:01:02.135"),
        &Time::from("00:02:00.063"),
        &Time::from("00:00:53.802"),
        &Time::from("00:10:06.135"),
        &Time::from("00:01:59.755"),
        &Time::from("00:01:47.453"),
      ]);
      assert_eq!(result.get_objective::<RunObjective>().is_some_and(|v| v.secondary == true), true);
  
    }

}
