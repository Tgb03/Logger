
use std::{fmt::Display, hash::Hash};

pub mod error;
mod constants;

pub mod run_objective;
pub mod game_objective;

pub mod game_run_rundown;
pub mod game_run_objective;

pub mod objective_enum;

pub trait Objective: 
  for<'a> TryFrom<&'a str> +
  Display +
  Hash {

  fn with_player_count(self, player_count: u8) -> Self;

}

