use egui::Ui;

use crate::{graphics::create_text, run::objectives::run_objective::RunObjective};


pub struct RunObjectiveReader {

  objective: RunObjective,

}

impl Default for RunObjectiveReader {
  fn default() -> Self {
    let objective = RunObjective::default()
      .with_player_count(1);

    Self {
      objective,  
    }
  }
}

impl RunObjectiveReader {

  pub fn get_objective(&self) -> &RunObjective {
    &self.objective
  }

  pub fn set_name(&mut self, name: String) {
    self.objective.set_name(name);
  }

  pub fn set_player_count(&mut self, count: u8) {
    self.objective.player_count = count;
  }

  pub fn show(&mut self, ui: &mut Ui) -> Option<&RunObjective> {

    let mut changed = false;
    
    ui.horizontal(|ui| {

      if ui.checkbox(&mut self.objective.secondary, create_text("Sec   ")).changed() { changed = true; }
      if ui.checkbox(&mut self.objective.overload, create_text("Ovrl")).changed() { changed = true; }

    });
    
    ui.horizontal(|ui| {

      if ui.checkbox(&mut self.objective.glitched, create_text("Glitch")).changed() { changed = true; }
      if ui.checkbox(&mut self.objective.early_drop, create_text("E-Drop")).changed() { changed = true; }

    });

    match changed {
      true => Some(&self.objective),
      false => None,
    }

  }

}