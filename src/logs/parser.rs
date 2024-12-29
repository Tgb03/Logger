use crate::{time::Time, timed_run::TimedRun};

use super::{run_parser::RunParser, token_parser::TokenParserT, tokenizer::Token};

#[derive(Default)]
pub struct ParserResult {

  runs: Vec<TimedRun>,

}

impl Into<Vec<TimedRun>> for ParserResult {
  fn into(self) -> Vec<TimedRun> {
    self.runs
  }
}

impl ParserResult {

  pub fn merge_result(&mut self, other: ParserResult) {
    self.runs.extend(other.runs);
  }

  pub fn get_runs(&self) -> &Vec<TimedRun> {
    &self.runs
  }

  pub fn get_runs_mut(&mut self) -> &mut Vec<TimedRun> {
    &mut self.runs
  }

}

#[derive(Default, PartialEq)]
enum ParserState {
  
  #[default] 
  OutOfGame,
  InGame,
  Finished,

}


#[derive(Default)]
pub struct Parser {

  result: ParserResult,
  state: ParserState,

  name_of_level: String,

  //parsers:
  run_parser: Option<RunParser>,

}


impl Into<ParserResult> for Parser {
  fn into(self) -> ParserResult {
    self.result
  }
}

impl TokenParserT<ParserResult> for Parser {

  fn into_result(&self) -> &ParserResult {
    &self.result 
  }
  
  fn parse_one_token(&mut self, (time, token): (Time, Token)) -> bool {

    match &self.state {
      ParserState::OutOfGame => {
        match token {
          Token::SelectExpedition(name) => self.name_of_level = name,
          Token::GameStarted => {
            self.state = ParserState::InGame;
            self.run_parser = Some(RunParser::new(self.name_of_level.clone(), time))
          },
          Token::PlayerDroppedInLevel(_) | Token::GameEndAbort => return self.state == ParserState::Finished,
          Token::LogFileEnd => {
            self.state = ParserState::Finished;
            
            return true;
          },
          _ => {},//eprintln!("{:?} failed to parse in parser.rs", token)
        }
      },
      ParserState::InGame => {
        if self.run_parser.as_mut().unwrap().parse_one_token((time, token)) {
          
          let run: TimedRun = self.run_parser.take().unwrap().into();
          self.result.runs.push(run);
          self.state = ParserState::OutOfGame;
        }
      },
      ParserState::Finished => return true,
    }

    false
  }

}
