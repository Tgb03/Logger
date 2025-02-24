
use crate::run::time::Time;

use super::{location::{Location, LocationType}, token_parser::TokenParserT, tokenizer::Token};

#[derive(Default)]
pub struct GenerationParser {

  buffer_keys: Vec<String>,

  result: Vec<Location>,

  done: bool,

}

impl From<GenerationParser> for Vec<Location> {
  fn from(value: GenerationParser) -> Self {
    value.result
  }
}

impl TokenParserT<Vec<Location>> for GenerationParser {
  fn into_result(&self) -> &Vec<Location> {
    &self.result
  }
  
  fn into_result_mut(&mut self) -> &mut Vec<Location> {
    &mut self.result
  }

  fn parse_one_token(&mut self, (_time, token): (Time, Token)) -> bool {
    
    if self.done { return true }

    match token {
      Token::ItemAllocated(name) => self.buffer_keys.push(name),
      Token::ItemSpawn(zone, id) => {
        let name = self.buffer_keys.pop();

        let location = Location::default()
          .with_id(id)
          .with_zone(zone)
          .with_type(LocationType::Key);

        let location = match name {
          Some(name) => location.with_name(name),
          None => location,
        };

        self.result.push(location);
        self.result.sort();
      },
      Token::GeneratingFinished | Token::GameEndAbort | Token::LogFileEnd => {
        self.done = true;
        return true;
      },
      _ => { eprintln!("Failed to parse token in gen parser: {:?}", token) }
    }

    false

  }
}
