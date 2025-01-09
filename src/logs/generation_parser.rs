use crate::time::Time;

use super::{location::Location, token_parser::TokenParserT, tokenizer::Token};

#[derive(Default)]
pub struct GenerationParser {

  buffer: Vec<String>,
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

  fn parse_one_token(&mut self, (_time, token): (Time, Token)) -> bool {
    
    if self.done { return true }

    match token {
      Token::ItemAllocated(name) => self.buffer.push(name),
      Token::ItemSpawn(zone, id) => {
        let name = self.buffer.pop();

        let location = Location::default()
          .with_id(id)
          .with_zone(zone);

        let location = match name {
          Some(name) => location.with_name(name),
          None => location,
        };

        println!("Added: {}", Into::<String>::into(&location));
        self.result.push(location);
        self.result.sort();
      },
      Token::GeneratingFinished | Token::GameEndAbort => {
        self.done = true;
        return true;
      },
      _ => { eprintln!("Failed to parse token in gen parser: {:?}", token) }
    }

    false

  }
}
