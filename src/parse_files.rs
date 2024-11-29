
pub mod file_parse {
  use std::{fs::File, io::Read, path::PathBuf};
  use crate::{logs::tokenizer::Tokenizer, timed_run::{TimedRun, TimedRunParser}};

  pub async fn parse_all_files_async(_paths: Vec<PathBuf>) -> Vec<TimedRun> {
    todo!()
  }

  pub fn parse_all_files(paths: Vec<File>) -> Vec<TimedRun> {
    let mut result: Vec<TimedRun> = Vec::new();

    for path in paths {
      result.extend(parse_file(path));
    }

    result
  }

  fn parse_file(mut path: File) -> Vec<TimedRun> {
    let mut data = String::new();
    let res = path.read_to_string(&mut data);
    if res.is_err() { return Vec::new(); }
    
    let tokens = Tokenizer::tokenize(&data);

    let mut run_parser = TimedRunParser::new();
    run_parser.parse_times(tokens);

    run_parser.get_results()
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
      let result = file_parse::parse_all_files(vec![file1_reader, file2_reader]);

      assert_eq!(result.len(), 2);
      assert_eq!(result[0].objective_data, ObjectiveData::from("R1C1".to_string(), false, false, false, false, 4));
      assert_eq!(result[0].get_times(), vec![
        Time::from("00:01:02.135"),
        Time::from("00:03:02.198"),
        Time::from("00:03:56.000"),
        Time::from("00:14:02.135"),
        Time::from("00:16:01.890"),
        Time::from("00:17:49.343"),
      ]);
      assert_eq!(result[0].win, true);

      drop(file1);
      drop(file2);
      let _ = dir.close();
  }
}