use crate::time::Time;

use super::{location::{Location, LocationType}, token_parser::TokenParserT, tokenizer::Token};

#[derive(Default)]
pub struct GenerationParser {

  buffer_keys: Vec<String>,
  buffer_objective: Vec<Location>,
  buffer_obj_alloc: Vec<Location>,

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
      Token::ObjectiveAllocated(zone, id) => {
        let location = Location::default()
          .with_zone(zone)
          .with_type(LocationType::Objective);

        let location = match id {
          Some(id) => location.with_id(id),
          None => location
        };

        if self.buffer_obj_alloc.len() > 0 {
          let obj = self.buffer_obj_alloc.remove(0);

          self.result.push(
            
            location.with_name(
              obj.get_name()
                .unwrap_or(&"".to_owned())
                .clone()
            )

          );
          self.result.sort();

          return false;
        }

        self.buffer_objective.push(location)
      },
      Token::ObjectiveSpawned(name) => {
        let known = self.buffer_objective.pop();

        let location = match known {
          Some(known) => known.with_name(name),
          None => Location::default().with_name(name)
        };

        self.result.push(location.with_type(LocationType::Objective));
        self.result.sort();
      },
      Token::ObjectiveGather(id, count) => {
        let name = match id {
          149 => "GLP",
          150 => "OSIP",
          _ => "Unknown",
        };
        
        for _ in 0..count {
          self.buffer_obj_alloc.push(
            Location::default()
              .with_name(name.to_owned())
          );
        }
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
