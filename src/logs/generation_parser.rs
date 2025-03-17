

use crate::run::time::Time;

use super::{
  collectable_mapper, location::{
    Location, LocationType
  }, token_parser::TokenParserT, tokenizer::Token
};

#[derive(Default)]
pub struct GenerationParser {

  level_name: String,
  buffer_keys: Vec<(String, LocationType)>,
  buffer_collectable: (Vec<String>, Vec<u64>, usize),

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
      Token::ItemAllocated(name, key_type) => { 
        self.buffer_keys.push((name, match key_type {
            true => LocationType::BulkheadKey,
            false => LocationType::ColoredKey,
        }))
      },
      Token::ItemSpawn(zone, id) => {
        let name_loc = self.buffer_keys.pop();

        let location = Location::default()
          .with_id(id)
          .with_zone(zone);

        let location = match name_loc {
          Some((name, key_type)) => {
            location
              .with_name(name)
              .with_type(key_type)
          },
          None => location,
        };

        self.result.push(location);
      },
      Token::CollectableAllocated(zone) => {
        self.buffer_collectable.1.push(zone);
      },
      Token::ObjectiveSpawnedOverride(id, name) => {
        let zone = self.buffer_collectable.1.pop().unwrap_or(9999);
        let location = Location::default()
          .with_zone(zone)
          .with_id(id)
          .with_type(LocationType::Objective);
        let location = match name {
          Some(name) => location.with_name(name),
          None => location,
        };

        self.result.push(location);
      }
      Token::CollectableItemID(id) => {
        let id = Self::get_collectable_name(id);
        if id != "Cryo" && id != "Cargo" {
          self.buffer_collectable.0.push(id);
        } else {

          let zone = self.buffer_collectable.1.remove(0);

          self.result.push(Location::default()
            .with_name(id)
            .with_zone(zone)
            .with_type(LocationType::Objective));

        }
      }
      Token::CollectableItemSeed(mut seed) => {
        if let Ok(mutex) = collectable_mapper::COLLECTABLE_MAPPER.lock() {
          mutex.as_ref()
            .map(|m| 
              m.get_id(
                &self.level_name, 
                *self.buffer_collectable.1.get(0).unwrap_or(&0), 
                seed
              )
            ).flatten()
            .map(|v| { seed = v });
        }

        let location = Location::default()
          .with_name(self.buffer_collectable.0.remove(0))
          .with_zone(self.buffer_collectable.1.remove(0))
          .with_type(LocationType::Objective)
          .with_id(seed);

        self.result.push(location);
      }
      Token::GeneratingFinished | Token::GameEndAbort | Token::LogFileEnd => {
        self.done = true;
        return true;
      },
      _ => { eprintln!("Failed to parse token in gen parser: {:?}", token) }
    }

    false

  }
}

impl GenerationParser {

  pub fn new(level_name: String) -> Self {
    Self {
      level_name,
      ..Default::default()
    }
  }

  fn get_collectable_name(id: u64) -> String {
    match id {
      128 => "ID".to_owned(),
      129 => "PD".to_owned(),
      148 => "Cryo".to_owned(),
      149 => "ID".to_owned(),
      150 => "OSIP".to_owned(),
      165 => "DataCube".to_owned(),
      176 => "Cargo".to_owned(),
      _ => "UNK".to_owned(),
    }
  }

}
