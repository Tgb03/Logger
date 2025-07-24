use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::time::Time;
use crate::run::timed_run::{LevelRun, GameRun, RunEnum};

#[enum_dispatch]
pub trait Split {

    fn get_name(&self) -> &str;
    fn get_time(&self) -> Time;

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NamedSplit {

    time: Time,
    name: String,

}

impl NamedSplit {

    pub fn new(time: Time, name: String) -> Self {
        Self {
            time,
            name
        }
    }

}

impl Split for NamedSplit {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_time(&self) -> Time {
        self.time
    }
}

impl<S: Split> From<&S> for NamedSplit {
    fn from(value: &S) -> Self {
        Self {
            time: value.get_time(),
            name: value.get_name().to_owned(),
        }
    }
}

impl From<&dyn Split> for NamedSplit {
    fn from(value: &dyn Split) -> Self {
        Self {
            time: value.get_time(),
            name: value.get_name().to_owned(),
        }
    }
}
