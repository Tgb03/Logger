use std::num::ParseIntError;

use serde::{Serialize, Deserialize};



#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct Time {
  // specific time stamp. 
  // millisecond time precision
  stamp: u64
}

impl Default for Time {
  fn default() -> Self {
    Self { 
      stamp: Default::default() 
    }
  }
}

impl Time {

  ///
  /// Creates a Time with stamp 0
  ///
  pub fn new() -> Time {
    return Time { stamp: 0 };
  }

  ///
  /// Creates a Time from a String
  /// String format: {Hours}:{Minutes}:{Seconds}.{Milliseconds}
  ///
  pub fn from(time: &str) -> Result<Time, ParseIntError> {
    let hours: u64 = time[0..2].parse::<u64>()?;
    let minutes: u64 = time[3..5].parse::<u64>()?;
    let seconds: u64 = time[6..8].parse::<u64>()?;
    let milliseconds: u64 = time[9..12].parse::<u64>()?;

    Ok(Time {
      stamp: (((hours * 60 + minutes) * 60) + seconds) * 1000 + milliseconds
    })
  }

  pub fn max() -> Time {
    Time {
      stamp: 24 * 60 * 60 * 1000
    }
  }

  pub fn get_stamp(&self) -> u64 {
    return self.stamp
  }

  /// 
  /// Returns a string showing the time of the object.
  /// 
  pub fn to_string(&self) -> String {
    let milliseconds: u64 = self.stamp % 1000;
    let seconds: u64 = self.stamp / 1000 % 60; 
    let minutes: u64 = self.stamp / 60000 % 60;
    let hours: u64 = self.stamp / 3600000;
    
    return format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds);
  }

  /// 
  /// Returns a string showing the time of the object
  /// without the hours mark if the hours is 0
  /// 
  pub fn to_string_no_hours(&self) -> String {
    let milliseconds: u64 = self.stamp % 1000;
    let seconds: u64 = self.stamp / 1000 % 60; 
    let minutes: u64 = self.stamp / 60000 % 60;
    let hours: u64 = self.stamp / 3600000;
    
    if hours > 0 {
      return format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds);
    }

    format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
  }

  //// Create a new time stamp with the sum of these 2 times.
  pub fn add(&self, other: &Time) -> Time {
    Time {stamp: self.stamp + other.stamp}
  }

  /// Create a new time stamp with the subtraction of these 2 times.
  pub fn sub(&self, other: &Time) -> Time {
    if self.is_smaller_than(other) {
      return Time {stamp : self.stamp + 24 * 60 * 60 * 1000 - other.stamp}
    }

    return Time {stamp: self.stamp - other.stamp }
  }

  /// Check if this time is greater than another time.
  pub fn is_greater_than(&self, other: &Time) -> bool {
    return self.stamp > other.stamp
  }

  /// Check if this time is greater or equal than another time.
  pub fn is_greater_or_equal_than(&self, other: &Time) -> bool {
    return self.stamp >= other.stamp
  }

  /// Check if these times are equal.
  pub fn is_equal(&self, other: &Time) -> bool {
    return self.stamp == other.stamp;
  }

  /// Check if this time is smaller than another time.
  pub fn is_smaller_than(&self, other: &Time) -> bool {
    return self.stamp < other.stamp
  }

  /// Check if this time is smaller or equal than another time.
  pub fn is_smaller_or_equal_than(&self, other: &Time) -> bool {
    return self.stamp <= other.stamp;
  }

  pub fn min(&self, other: &Time) -> Time {
    if self.is_smaller_than(other) {
      return *self
    }

    *other
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {

    let time: Time = Time::new();
    let stamp : u64 = time.stamp;

    assert_eq!(stamp, 0);

  }

    #[test]
  fn test_from() {

    let time1 = Time::from("11:22:33.444");
    let time2 = Time::from("23:59:59.999");
    let time3 = Time::from("00:00:00.000");
      
    assert_eq!(time1.unwrap().stamp, 40953444);
    assert_eq!(time2.unwrap().stamp, 86399999);
    assert_eq!(time3.unwrap().stamp, 0);

  }

  #[test] 
  fn test_from_overflow() {

    let time1 = Time::from("11:22:33.44400");
    let time2 = Time::from("23:59:59.999adda");
    let time3 = Time::from("00:00:00.000111");
      
    assert_eq!(time1.unwrap().stamp, 40953444);
    assert_eq!(time2.unwrap().stamp, 86399999);
    assert_eq!(time3.unwrap().stamp, 0);

  }

  #[test]
  fn test_to_string() {

    let time1 = Time::from("11:22:33.444");
    let time2 = Time::from("23:59:59.999");
    let time3 = Time::from("00:00:00.000");

    assert_eq!(time1.unwrap().to_string(), "11:22:33.444");
    assert_eq!(time2.unwrap().to_string(), "23:59:59.999");
    assert_eq!(time3.unwrap().to_string(), "00:00:00.000");

  }

  #[test]
  fn test_add() {

    let time1 = Time::from("00:00:00.010");
    let time2 = Time::from("00:00:00.133");

    assert_eq!(time1.unwrap().add(&time2.unwrap()).to_string(), "00:00:00.143".to_string());

  }

  #[test]
  fn test_sub_normal() {

    let time1: Time = Time::from("00:00:00.500").unwrap();
    let time2: Time = Time::from("00:00:00.133").unwrap();

    assert_eq!(time1.sub(&time2).to_string(), "00:00:00.367".to_string());

  }

  #[test]
  fn test_sub_underflow() {

    let time1: Time = Time::from("00:00:00.500").unwrap();
    let time2: Time = Time::from("00:00:01.000").unwrap();

    assert_eq!(time1.sub(&time2).to_string(), "23:59:59.500".to_string());

  }

  #[test]
  fn test_operators() {

    let time1: Time = Time::from("00:00:01.000").unwrap();
    let time2: Time = Time::from("00:00:02.000").unwrap();

    assert!(time2.is_greater_than(&time1));
    assert!(time2.is_greater_or_equal_than(&time1));
    assert!(time1.is_smaller_than(&time2));
    assert!(time1.is_smaller_or_equal_than(&time2));
    assert!(time1.is_greater_or_equal_than(&time1));
    assert!(time2.is_greater_or_equal_than(&time2));

  }
  
}