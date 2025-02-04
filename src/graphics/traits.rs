use egui::Ui;

use crate::run::traits::Run;

#[derive(Default)]
pub struct RenderResult {

  pub delete: bool,
  pub save: bool,

}

pub trait RenderRun: Run {

  fn show(&self, ui: &mut Ui) -> RenderResult;
  fn show_editable(&mut self, ui: &mut Ui) -> RenderResult;

}

/*
pub trait RenderObjective: Objective {

  fn show(&self, ui: &mut Ui);
  fn show_editable(&mut self, ui: &mut Ui);

}
*/
