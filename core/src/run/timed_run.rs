
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::{run::{objectives::objective_enum::ObjectiveEnum, split::{NamedSplit, Split}, traits::Run}, time::Time};

pub type LevelRun = TimedRun<NamedSplit>;
pub type GameRun = TimedRun<LevelRun>;

#[enum_dispatch(Run, Split)]
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum RunEnum {
    Level(LevelRun),
    Game(GameRun),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TimedRun<S>
where 
    S: Split, {

    total_time: Time,

    used_checkpoint: bool,
    is_win: bool,

    objective: ObjectiveEnum,
    objective_cache: String, 
    splits: Vec<S>,

}

impl<S> Split for TimedRun<S>
where
    S: Split, {
    
    fn get_name(&self) -> &str {
        &self.objective_cache
    }
    
    fn get_time(&self) -> Time {
        self.total_time
    }
}

impl<S> Run for TimedRun<S>
where
    S: Split, {
    
    fn get_splits<'a>(&'a self) -> Box<dyn Iterator<Item =  &'a dyn Split> +'a> {
        Box::new(self.splits.iter().map(|v| v as &dyn Split))
    }
    
    fn get_time_for_split(&self, split_name: &str) -> Option<Time> {
        self.splits.iter()
            .find(|s| s.get_name() == split_name)
            .map(|v| v.get_time())
    }
    
    fn is_win(&self) -> bool {
        self.is_win
    }
    
    fn len(&self) -> usize {
        self.splits.len()
    }
    
    fn set_win(&mut self, is_win: bool) {
        self.is_win = is_win
    }
    
    fn get_objective(&self) -> &ObjectiveEnum {
        &self.objective
    }
    
    fn set_objective(&mut self, objective: ObjectiveEnum) {
        self.objective = objective;
        self.objective_cache = self.objective.to_string();
    }

    fn set_objective_str(&mut self, objective: &str) {
        if let Ok(obj) = objective.try_into() {
            self.set_objective(obj);
        }
    }
}

impl<S: Split> Default for TimedRun<S> {
    fn default() -> Self {
        Self { 
            total_time: Default::default(), 
            used_checkpoint: Default::default(), 
            is_win: Default::default(), 
            splits: Default::default(),
            objective: Default::default(),
            objective_cache: Default::default(), 
        }
    }
} 

impl<S> TimedRun<S>
where 
    S: Split {

    pub fn new(objective: ObjectiveEnum) -> Self {
        Self {
            objective_cache: objective.to_string(),
            objective: objective,
            ..Default::default()
        }
    }

    pub fn add_split(&mut self, split: S) {
        self.total_time += split.get_time();
        self.splits.push(split);
    }

}
