use std::fmt::Display;

mod constants;
pub mod error;

pub mod game_objective;
pub mod run_objective;

pub mod game_run_objective;
pub mod game_run_rundown;

pub mod objective_enum;

pub trait Objective: for<'a> TryFrom<&'a str> + ToString + Display {
    fn get_player_count(&self) -> u8;
    fn with_player_count(self, player_count: u8) -> Self;

    fn get_name(&self) -> Option<&String>;
}
