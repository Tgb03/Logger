
pub mod save_run {
  use std::{env, path::PathBuf};

use crate::timed_run::TimedRun;
  use bincode;

  fn get_directory() -> PathBuf {
    match std::env::var("HOME") {
      Ok(path) => PathBuf::from(format!("{}\\AppData\\LocalLow\\Tgb03\\", path)),
      Err(_) => match env::current_dir() {
        Ok(path) => path,
        Err(_) => panic!("No current directory or home folder found."),
      },
    }
  }

  fn get_name_base(name: String, secondary: bool, overload: bool, glitched: bool, early_drop: bool) -> String {
    let secondary = match secondary {
      true => "_sec",
      false => ""
    };
    let overload = match overload {
      true => "_ovrl",
      false => ""
    };
    let glitched = match glitched {
      true => "_glitch",
      false => ""
    };
    let early_drop = match early_drop {
      true => "_edrop",
      false => ""
    };

    format!("{}{}{}{}{}.save", name, secondary, overload, glitched, early_drop)
  }

  fn get_name_file(timed_run: TimedRun) -> String {
    get_name_base(
      timed_run.level_name, 
      timed_run.objective_data.secondary, 
      timed_run.objective_data.overload, 
      timed_run.objective_data.glitched, 
      timed_run.objective_data.early_drop
    )
  }

  pub fn save(timed_run: TimedRun) {

    let file = get_directory().join(get_name_file(timed_run));


  }

  pub fn save_multiple(timed_run: Vec<TimedRun>) {
    
  }

  pub fn load(name: String, secondary: bool, overload: bool, glitched: bool, early_drop: bool) {

    let id = get_name_base(name, secondary, overload, glitched, early_drop);

  }

}
