use std::{fs::File, io::{BufRead, BufReader, Seek}, path::PathBuf, sync::mpsc::Receiver};

use crate::logs::{generation_parser::GenerationParser, parser::{Parser, ParserResult}, run_parser::RunParser, token_parser::TokenParserT};

use super::folder_watcher::FolderWatcher;

#[derive(Default)]
pub struct LiveParser {
  
  last_position: u64,
  parser: Parser,
  
  file: Option<BufReader<File>>,
  folder_watcher: Option<Receiver<PathBuf>>,

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
  
  fn into_result_mut(&mut self) -> &mut ParserResult {
    self.parser.into_result_mut()
  }

  fn parse_one_token(&mut self, token_pair: (crate::run::time::Time, crate::logs::token::Token)) -> bool {
    self.parser.parse_one_token(token_pair)
  }
}

impl LiveParser {
  pub fn get_run_parser(&self) -> Option<&RunParser> {
    self.parser.get_run_parser()
  }

  pub fn get_run_parser_mut(&mut self) -> Option<&mut RunParser> {
    self.parser.get_run_parser_mut()
  }

  pub fn get_generation_parser(&self) -> Option<&GenerationParser> {
    self.parser.get_generation_parser()
  }

  pub fn reset(&mut self) {
    self.parser = Parser::default();
    self.last_position = 0;
  }

  pub fn start_watcher(&mut self, folder_path: PathBuf) {
    self.folder_watcher = Some(FolderWatcher::new_watcher(folder_path));
  }

  pub fn stop_watcher(&mut self) {
    self.folder_watcher = None;
  }
  
  pub fn load_file(&mut self) -> bool {
    let path = self.folder_watcher.as_mut()
      .map(|v| v.try_recv().ok())
      .flatten();

    if let Some(path) = path {

      self.file = match File::open(path) {
        Ok(file) => Some(BufReader::new(file)),
        Err(_) => None,
      };

      self.reset();

      return true
    }

    false
  }

  pub fn load_text(&mut self) -> String {
    let Some(reader) = &mut self.file else {
      return String::new();
    };

    if let Err(_) = reader.seek(std::io::SeekFrom::Start(self.last_position)) {
      return String::new();
    }

    let mut buffer = String::new();
    let mut line = String::new();

    while reader.read_line(&mut line).unwrap_or(0) > 0 {
      buffer.push_str(&line);
      line.clear();
    }

    self.last_position = reader.stream_position().unwrap_or(self.last_position);

    buffer
  }
}

