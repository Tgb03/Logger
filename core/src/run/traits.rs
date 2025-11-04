use crate::run::timed_run::{GameRun, LevelRun};
use enum_dispatch::enum_dispatch;
use glr_core::{split::Split, time::Time};

use crate::run::{objectives::objective_enum::ObjectiveEnum, timed_run::RunEnum};

#[enum_dispatch]
pub trait Run: Split {
    fn get_splits<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Split> + 'a>;
    fn get_time_for_split(&self, split_name: &str) -> Option<Time>;

    fn is_win(&self) -> bool;
    fn len(&self) -> usize;

    fn set_win(&mut self, is_win: bool);

    fn get_objective(&self) -> &ObjectiveEnum;
    fn set_objective(&mut self, objective: ObjectiveEnum);
    fn set_objective_str(&mut self, objective: &str);

    fn get_split_by_name<'a>(&'a self, split_name: &str) -> Option<&'a dyn Split> {
        self.get_splits().find(|s| s.get_name() == split_name)
    }
}
