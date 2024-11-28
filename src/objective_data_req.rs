use crate::timed_run::TimedRun;

/// trait which is implemented by objects that 
/// can set the objective data of a run
pub trait RequestObjectiveData {

  fn set_objective_data(run: TimedRun) -> TimedRun;

}

