use enum_dispatch::enum_dispatch;

use super::{
    objectives::Objective,
    run_enum::RunEnum,
    timed_run::{GameRun, LevelRun},
};
use crate::time::Time;

pub trait Timed {
    fn get_time(&self) -> Time;
    fn get_name(&self) -> &String;
    fn is_finished(&self) -> bool;
}

#[enum_dispatch(RunEnum)]
pub trait Run: Timed {
    fn get_splits<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Timed> + 'a>;
    fn get_time_for_split(&self, split_name: &String) -> Option<Time>;

    fn is_win(&self) -> bool;
    fn len(&self) -> usize;

    fn set_win(&mut self, is_win: bool);

    fn get_objective<O: Objective>(&self) -> Option<O>;
    fn set_objective<O: Objective>(&mut self, objective: &O);

    fn get_objective_str(&self) -> &String;
    fn set_objective_str(&mut self, objective: String);
}
