use crate::time::Time;

use super::{
    location::{KeyGenerator, Location, LocationGenerator, ObjectiveItemGenerator},
    token::Token,
    token_parser::TokenParserT,
};

#[derive(Default)]
pub struct GenerationParser {
    result: Vec<Location>,

    key_generator: KeyGenerator,
    objective_generator: ObjectiveItemGenerator,

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
        if self.done {
            return true;
        }

        if let Some(key) = self.key_generator.accept_token(&token) {
            self.result.push(key);
            return false;
        }
        if let Some(obj) = self.objective_generator.accept_token(&token) {
            self.result.push(obj);
            return false;
        }

        match token {
            Token::GeneratingFinished | Token::GameEndAbort | Token::LogFileEnd => {
                self.done = true;
                return true;
            }
            _ => {
                // eprintln!("Failed to parse token in gen parser: {:?}", token)
            }
        }

        false
    }
}
