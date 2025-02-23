
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{time::Time, traits::Timed};

#[derive(Default, Clone, Hash, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NamedTime {
  
  time: Time,
  name: String,

}

impl NamedTime {

  pub fn new(time: Time, name: String) -> NamedTime {
    Self {
      time,
      name
    }
  }

}

impl Display for NamedTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.time.to_string())
  }
}

impl Timed for NamedTime {
  fn get_time(&self) -> Time {
    self.time
  }

  fn get_name(&self) -> &String {
    &self.name
  }

  fn is_finished(&self) -> bool {
    true
  }
}
