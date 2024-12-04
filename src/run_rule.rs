use crate::{objective_data::ObjectiveData, time::Time, timed_run::TimedRun};

pub struct ObjectiveFilter {

  level_name: Option<String>,
  secondary: Option<bool>,
  overload: Option<bool>,
  glitched: Option<bool>,
  early_drop: Option<bool>,
  player_count: Option<u8>,

}

impl ObjectiveFilter {
  pub fn check_match(&self, timed_run: &TimedRun) -> bool {
    if self.level_name.as_ref().is_some_and(|v| v == &timed_run.objective_data.level_name) { return true }
    if self.secondary.as_ref().is_some_and(|v| v == &timed_run.objective_data.secondary) { return true }
    if self.overload.as_ref().is_some_and(|v| v == &timed_run.objective_data.overload) { return true }
    if self.glitched.as_ref().is_some_and(|v| v == &timed_run.objective_data.glitched) { return true }
    if self.early_drop.as_ref().is_some_and(|v| v == &timed_run.objective_data.early_drop) { return true }
    if self.player_count.as_ref().is_some_and(|v| v == &timed_run.objective_data.player_count) { return true }

    false
  }
}

pub struct RunRule {

  order: RuleOrder,
  objective: ObjectiveData,

}

pub enum RuleOrder {

  MergeSplitIntoNext(usize),
  AllBeBiggerThan(Time),

}

impl RuleOrder {

  pub fn apply_rule(&self, timed_run: &mut TimedRun) {
    match self {
      RuleOrder::MergeSplitIntoNext(id) => {
        if *id >= timed_run.times.len() { return }

        timed_run.times.remove(*id);
      },
      RuleOrder::AllBeBiggerThan(time) => {
        
      },
    }
  }

}
