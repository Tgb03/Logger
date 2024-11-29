use crate::{base_app::ShowUI, save_run::SaveManager, timed_run::TimedRun};



pub struct RunManagerWindow {

  timed_runs: Vec<TimedRun>,
  save_run: SaveManager,

}

impl RunManagerWindow {
  pub fn new() -> RunManagerWindow {
    RunManagerWindow {
      timed_runs: Vec::new(),
      save_run: SaveManager::new()
    }
  }
}

impl ShowUI for RunManagerWindow {
  
  fn show(&mut self, ui: &mut egui::Ui) {
    
  }

}
