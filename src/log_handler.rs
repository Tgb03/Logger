use std::{borrow::Borrow, collections::HashMap, rc::Rc};

use crate::time_manager::TimeManager;

pub(crate) trait LogHandler {

  fn parse_lines(&mut self, lines: &[&str]) {
    for line in lines {
      self.parse_line(line);
    }
  }

  fn parse_line(&mut self, line: &str);

}

#[cfg(test)]
mod tests {
  use super::*;
  
  struct Counter {
    counter: u32
  }

  impl LogHandler for Counter {
    fn parse_line(&mut self, line: &str) {
      if line.contains("TEST") {
        self.counter += 1;
      }
    }
  }

  impl Counter {
    fn new() -> Counter {
      Counter { counter: 0 }
    }

    fn get_counter(&self) -> u32 {
      return self.counter;
    }
  }

  #[test]
  fn test_base() {
    let mut counter: Counter = Counter::new();
    counter.parse_lines(&vec!["This is uwu.", "This is a TEST"]);
    
    assert_eq!(counter.get_counter(), 1);
  }
}

