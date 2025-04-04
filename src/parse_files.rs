
pub mod file_parse {

  const MAX_THREAD: usize = 8;
  
  use std::thread;
  use std::sync::{Arc, Mutex};
  #[cfg(debug_assertions)]
  use std::time::Instant;
  use std::{fs::File, io::Read};

  use crate::logs::parser::{Parser, ParserResult};
  use crate::logs::token_parser::TokenParserT;
  use crate::logs::tokenizer::{GenericTokenizer, RunTokenizer, Tokenizer};

  pub fn parse_all_files_async<'a>(paths: Vec<File>) -> ParserResult {
    #[cfg(debug_assertions)]
    let start = Instant::now();

    let thread_count = MAX_THREAD.min(paths.len() / 12 + 1);
    
    let paths = Arc::new(Mutex::new(paths.into_iter()));

    let mut threads = Vec::new();
    for _ in 0..thread_count {
      let paths_clone = paths.clone();

      threads.push(thread::spawn(move || {
        
        let mut result = ParserResult::default();
        let tokenizer = GenericTokenizer::default()
          .add_tokenizer(RunTokenizer);

        loop {
          
          let file = match paths_clone.lock() {
            Ok(mut p_id) => {
              p_id.next()
            },
            Err(_) => return result,
          };

          match file {
            Some(file) => result.merge_result(parse_file(&file, &tokenizer)),
            None => return result,
          }

        }

      }));
    }

    let mut result = ParserResult::default();
    for thread in threads {
      match thread.join() {
        Ok(res) => result.merge_result(res),
        Err(_) => {},
      }
    }

    #[cfg(debug_assertions)] {
      let duration = start.elapsed();
      println!("Time elapsed with threads is: {:?}", duration);
    }
    
    result
  }

  pub fn parse_all_files<'a, I>(paths: I) -> ParserResult
  where 
    I: IntoIterator<Item = &'a File> {
    let mut result: ParserResult = Default::default();

    let tokenizer = GenericTokenizer::default()
      .add_tokenizer(RunTokenizer);

    // let start = Instant::now();
    // let mut counter = 0;
    for path in paths {
      // counter += 1;
      result.merge_result(parse_file(path, &tokenizer));
    }
    // let duration = start.elapsed();
    //println!("Parsed without threads {} files in: {:?}", counter, duration); 

    result
  }

  fn parse_file(mut path: &File, tokenizer: &impl Tokenizer) -> ParserResult {
    let mut data = String::new();
    let res = path.read_to_string(&mut data);
    if res.is_err() { return Default::default(); }
    
    let tokens = tokenizer.tokenize(&data);

    Parser::parse_all_tokens_default(tokens.into_iter())
  }
}

#[cfg(test)]
mod tests {
  use std::{fs, io::Write, path::PathBuf};
  use fs::File;
  use tempfile::{tempdir, TempDir};

  use crate::run::{objectives::run_objective::RunObjective, time::Time, traits::{Run, Timed}};

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
      00:00:09.000 - GAMESTATEMANAGER CHANGE STATE FROM : StopElevatorRide TO: ReadyToStartLevel
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
      assert_eq!(result[0].is_win(), true);
      assert_eq!(result[0].get_objective::<RunObjective>().unwrap().level_name, "R1C1");
      assert_eq!(*result[0].get_splits().map(|v| v.get_time()).collect::<Vec<Time>>(), vec![
        Time::from("00:01:02.135").unwrap(),
        Time::from("00:02:00.063").unwrap(),
        Time::from("00:00:53.802").unwrap(),
        Time::from("00:10:06.135").unwrap(),
        Time::from("00:01:59.755").unwrap(),
        Time::from("00:01:47.453").unwrap(),
      ]);

      drop(file1);
      drop(file2);
      let _ = dir.close();
  }
}