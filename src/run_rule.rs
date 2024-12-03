use crate::{objective_data::ObjectiveData, parse_files::file_parse::TokenParserResult, time::Time, timed_run::{self, TimedRun}};


pub struct RunRule {

  order: RuleOrder,
  objective: ObjectiveData,

}

pub enum RuleOrder {

  MergeSplitIntoNext(usize),
  MustBeBiggerThan(usize, Time),

}

pub trait ApplyRulesOrder {
  fn apply_rules<I>(iter: I, timed_run: &mut TimedRun) 
  where I: Iterator<Item=Self> {

  }
}

impl RuleOrder {

  pub fn apply_rule(&self, timed_run: &mut TimedRun) {
    match self {
      RuleOrder::MergeSplitIntoNext(id) => {
        if *id >= timed_run.times.len() { return }

        timed_run.times.remove(*id);
      },
      RuleOrder::MustBeBiggerThan(id, time) => {
        if *id >= timed_run.times.len() { return }

        if timed_run.get_split(*id).is_smaller_or_equal_than(time) {
          timed_run.times.remove(*id);
        }
      },
    }
  }

}
