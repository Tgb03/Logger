
pub mod file_parse {
  use std::thread;
  use std::time::Instant;
//use std::time::Instant;
  use std::{fs::File, io::Read};
  use crate::logs::parser::{Parser, ParserResult};
  use crate::logs::token_parser::TokenParserT;
  use crate::logs::tokenizer::Tokenizer;

  pub fn parse_all_files_async<'a>(mut paths: Vec<File>) -> ParserResult {
    let start = Instant::now();

    let thread_count = 8.min(paths.len() / 12 + 1);
    let mut split_paths: Vec<Vec<File>> = Vec::new();
    split_paths.resize_with(thread_count, || Vec::new());

    for i in 0..paths.len() {
      split_paths[i % thread_count].push(paths.swap_remove(0));
    }

    let mut threads = Vec::new();

    for ps in split_paths {
      threads.push(thread::spawn(move || {
        let file_refs = &ps;

        parse_all_files(file_refs)
      }));
    }

    let mut result = ParserResult::default();
    for thread in threads {
      match thread.join() {
        Ok(res) => result.merge_result(res),
        Err(_) => {},
      }
    }

    let duration = start.elapsed();
    println!("Time elapsed with threads is: {:?}", duration);

    result
  }

  pub fn parse_all_files<'a, I>(paths: I) -> ParserResult
  where 
    I: IntoIterator<Item = &'a File> {
    let mut result: ParserResult = Default::default();

    let start = Instant::now();
    let mut counter = 0;
    for path in paths {
      counter += 1;
      result.merge_result(parse_file(path));
    }
    let duration = start.elapsed();
    println!("Parsed without threads {} files in: {:?}", counter, duration); 

    result
  }

  fn parse_file(mut path: &File) -> ParserResult {
    let mut data = String::new();
    let res = path.read_to_string(&mut data);
    if res.is_err() { return Default::default(); }
    
    let tokens = Tokenizer::tokenize(&data);

    Parser::parse_all_tokens_default(tokens.into_iter())
  }
}

#[cfg(test)]
mod tests {
  use std::{fs, io::Write, path::PathBuf};
  use fs::File;
  use tempfile::{tempdir, TempDir};
  use crate::{objective_data::ObjectiveData, time::Time};

  use super::*;

  fn init_file(dir: &TempDir, name: &str, data: &str) -> (File, PathBuf) {
    let file_path = dir.path().join(name);
    let file = File::create(&file_path);
    if file.is_err() { assert!(false) }
    let mut file = file.unwrap();

    let _ = write!(file, "{}", data);

    (file, file_path)
  }

  #[test]
  pub fn test_base() {
    let dir = tempdir();
    if dir.is_err() { assert!(false) }

    let dir = dir.unwrap();
    
    let text = "00:00:00.000 - <color=#C84800>SelectActiveExpedition : Selected! Local Local_32 TierC 0 433572712 1571494152  sessionGUID:SNetwork.SNetStructs+pSessionGUID FriendsData expID set to: Local_32,3,0 ActiveExpeditionUniqueKey: Local_32_TierC_0</color>
      00:00:10.000 - GAMESTATEMANAGER CHANGE STATE FROM : ReadyToStartLevel TO: InLevel
      00:00:10.000 - Player1 exits PLOC_InElevator 1</color>
      00:00:10:055 - Useless line
      00:00:10.100 - Player2 exits PLOC_InElevator 2</color>
      00:00:10.110 - Player3 exits PLOC_InElevator 3</color>
      00:00:10.250 - Player4 exits PLOC_InElevator 4</color>
      00:01:12.135 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:03:12.198 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:04:06.000 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:14:12.135 - OnDoorIsOpened, LinkedToZoneData.EventsOnEnter
      00:16:11.890 - BulkheadDoorController_Core.OnScanDone
      00:17:59.343 - GAMESTATEMANAGER CHANGE STATE FROM : InLevel TO: ExpeditionSuccess";
    let (file1, path1) = init_file(&dir, "file1.txt", text);
    let (file2, path2) = init_file(&dir, "file2.txt", text);

      let file1_reader = File::open(path1).unwrap();
      let file2_reader = File::open(path2).unwrap();
      let result = file_parse::parse_all_files(&vec![file1_reader, file2_reader]);
      let result = result.get_runs();

      assert_eq!(result.len(), 2);
      assert_eq!(result[0].objective_data, ObjectiveData::from("R1C1".to_string(), false, false, false, false, 4));
      assert_eq!(result[0].is_win(), true);
      assert_eq!(*result[0].get_times(), vec![
        Time::from("00:01:02.135"),
        Time::from("00:03:02.198"),
        Time::from("00:03:56.000"),
        Time::from("00:14:02.135"),
        Time::from("00:16:01.890"),
        Time::from("00:17:49.343"),
      ]);

      drop(file1);
      drop(file2);
      let _ = dir.close();
  }
}