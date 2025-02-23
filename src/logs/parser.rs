

use crate::run::{time::Time, timed_run::LevelRun};

use super::{generation_parser::GenerationParser, location::Location, run_parser::RunParser, token_parser::TokenParserT, tokenizer::Token};

#[derive(Default)]
pub struct ParserResult {

  runs: Vec<LevelRun>,
  locations: Vec<Location>,

}

impl Into<Vec<LevelRun>> for ParserResult {
  fn into(self) -> Vec<LevelRun> {
    self.runs
  }
}

impl ParserResult {

  pub fn merge_result(&mut self, other: ParserResult) {
    self.runs.extend(other.runs);
  }

  pub fn get_runs(&self) -> &Vec<LevelRun> {
    &self.runs
  }

  pub fn get_runs_mut(&mut self) -> &mut Vec<LevelRun> {
    &mut self.runs
  }

  pub fn get_locations(&self) -> &Vec<Location> {
    &self.locations
  }

}

#[derive(Default, PartialEq)]
enum ParserState {
  
  #[default] 
  OutOfGame,
  GeneratingLevel,
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
  generation_parser: Option<GenerationParser>,

}

impl Parser {

  pub fn get_run_parser(&self) -> Option<&RunParser> {
    self.run_parser.as_ref()
  }

  pub fn get_run_parser_mut(&mut self) -> Option<&mut RunParser> {
    self.run_parser.as_mut()
  }

  pub fn get_generation_parser(&self) -> Option<&GenerationParser> {
    self.generation_parser.as_ref()
  }

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
          Token::GeneratingLevel => {
            //eprintln!("Started generating.");
            self.state = ParserState::GeneratingLevel;
            self.result.locations.clear();
            self.generation_parser = Some(GenerationParser::default());
          }
          Token::SelectExpedition(name) => self.name_of_level = name,
          Token::GameStarting => {
            //eprintln!("Started game.");
            self.state = ParserState::InGame;
            self.run_parser = Some(RunParser::new(self.name_of_level.clone()))
          },
          // logs have so many edge cases like this bs one
          // at some point some shit like this might be useful
          // however it is so cursed I will ignore it until
          // someone finds a bug with the current implementation
          // that doesn't use or screw around with the PLOC
          /*
          Token::PlayerDroppedInLevel(id) => {
            self.state = ParserState::InGame;
            let mut parser = RunParser::new(self.name_of_level.clone(), time);
            parser.parse_one_token((time, Token::PlayerDroppedInLevel(id)));
            self.run_parser = Some(parser);
          },
          */
          Token::GameEndAbort => return self.state == ParserState::Finished,
          Token::LogFileEnd => {
            self.state = ParserState::Finished;
            
            return true;
          },
          _ => { /* eprintln!("{:?} failed to parse in parser.rs", token) */ },
        }
      },
      ParserState::GeneratingLevel => {
        if self.generation_parser.as_mut().unwrap().parse_one_token((time, token)) {

          let locations: Vec<Location> = self.generation_parser.take().unwrap().into();
          self.result.locations.extend(locations);
          self.state = ParserState::OutOfGame;

          //eprintln!("Finished generating");
        }
      }
      ParserState::InGame => {
        if self.run_parser.as_mut().unwrap().parse_one_token((time, token)) {
          
          let run: LevelRun = self.run_parser.take().unwrap().into();
          self.result.runs.push(run);
          self.state = ParserState::OutOfGame;
        
          //eprintln!("Finished game");
        }
      },
      ParserState::Finished => return true,
    }

    false
  }

fn into_result_mut(&mut self) -> &mut ParserResult {
  &mut self.result
}

}
