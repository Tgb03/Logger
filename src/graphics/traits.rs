use egui::Ui;

use crate::{run::traits::Run, save_run::SaveManager};

#[derive(Default)]
pub struct RenderResult {

  pub delete: bool,
  pub save: bool,

}

pub trait RenderRun: Run {

  fn show(&self, save_manager: &SaveManager, ui: &mut Ui, show_split_times: bool) -> RenderResult;

}

/*
pub trait RenderObjective: Objective {

  fn show(&self, ui: &mut Ui);
  fn show_editable(&mut self, ui: &mut Ui);

}
*/
