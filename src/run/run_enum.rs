use serde::{Deserialize, Serialize};

use crate::run::timed_run::{GameRun, LevelRun};
use enum_dispatch::enum_dispatch;

use super::traits::Timed;


#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[enum_dispatch]
pub enum RunEnum {

  Level(LevelRun),
  Game(GameRun),

}

impl Default for RunEnum {
  fn default() -> Self {
    RunEnum::Level(LevelRun::default())  
  }
}

impl Timed for RunEnum {
  fn get_time(&self) -> super::time::Time {
    match self {
      RunEnum::Level(timed_run) => timed_run.get_time(),
      RunEnum::Game(timed_run) => timed_run.get_time(),
    }
  }

  fn get_name(&self) -> &String {
    match self {
      RunEnum::Level(timed_run) => timed_run.get_name(),
      RunEnum::Game(timed_run) => timed_run.get_name(),
    }
  }

  fn is_finished(&self) -> bool {
    match self {
      RunEnum::Level(timed_run) => timed_run.is_finished(),
      RunEnum::Game(timed_run) => timed_run.is_finished(),
    }
  }
}
