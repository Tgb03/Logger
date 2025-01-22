use std::{fs::{self, File}, io::{BufRead, BufReader, Seek}};

use crate::logs::{generation_parser::GenerationParser, parser::{Parser, ParserResult}, run_parser::RunParser, token_parser::TokenParserT};

use super::settings_window::SettingsWindow;


#[derive(Default)]
pub struct LiveParser {
  
  last_position: u64,
  parser: Parser,
  
  file: Option<File>,
  file_name: Option<String>,

}

impl Into<ParserResult> for LiveParser {
  fn into(self) -> ParserResult {
    self.parser.into()  
  }
}

impl TokenParserT<ParserResult> for LiveParser {
  fn into_result(&self) -> &ParserResult {
    self.parser.into_result()
  }

  fn parse_one_token(&mut self, token_pair: (crate::time::Time, crate::logs::tokenizer::Token)) -> bool {
    self.parser.parse_one_token(token_pair)
  }
}

impl LiveParser {
  pub fn get_run_parser(&self) -> Option<&RunParser> {
    self.parser.get_run_parser()
  }

  pub fn get_generation_parser(&self) -> Option<&GenerationParser> {
    self.parser.get_generation_parser()
  }

  pub fn reset(&mut self) {
    self.parser = Parser::default();
    self.last_position = 0;
  }

  
  pub fn load_file(&mut self, settings: &SettingsWindow) {
    let path = settings.get_logs_folder();

    let path = fs::read_dir(path)
      .expect("Couldn't access local directory")
      .flatten()
      .filter(|f| {
        let metadata = match f.metadata() {
          Ok(metadata) => metadata,
          Err(_) => { return false; },
        };

        metadata.is_file() && f.file_name().to_str().unwrap_or_default().contains("NICKNAME_NETSTATUS")
      })
      .max_by_key(|x| {
        match x.metadata() {
          Ok(metadata) => metadata.modified().ok(),
          Err(_) => Default::default(),
        }
      });

    if let Some(path) = path {
      let path = path.path();
      let name = path.file_name().unwrap_or_default();
      let str_name = name.to_str().unwrap_or_default();
    
      self.file_name = Some(str_name.to_string());
      self.file = match File::open(path) {
        Ok(file) => Some(file),
        Err(_) => None,
      };

      self.reset();
    }

  }

  pub fn load_text(&mut self) -> String {
    if let Some(file) = &mut self.file {
      let _ = file.seek(std::io::SeekFrom::Start(self.last_position));
      let mut reader = BufReader::new(file);
      let mut buffer = String::new();
      let mut new_lines = Vec::new();

      while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
        new_lines.push(buffer.to_string());
        buffer.clear();
      }

      self.last_position = reader.seek(std::io::SeekFrom::Current(0)).expect("Seek 0 failed in live window.");
    
      return new_lines.iter()
        .fold(String::new(), |s1, s2| s1 + s2)
        .to_string();
    }

    "".to_owned()
  }
}

