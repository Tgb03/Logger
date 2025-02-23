use egui::{Color32, Ui};

use crate::{graphics::create_text, run::objectives::run_objective::RunObjective};


pub struct RunObjectiveReader {

  objective: RunObjective,
  player_input_string: String,

}

impl Default for RunObjectiveReader {
  fn default() -> Self {
    let objective = RunObjective::default()
      .with_player_count(1);

    Self { 
      player_input_string: objective.player_count.to_string(),
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

  pub fn show(&mut self, ui: &mut Ui) -> Option<&RunObjective> {

    let mut changed = false;
    
    ui.horizontal(|ui| {

      if ui.checkbox(&mut self.objective.secondary, create_text("Sec")).changed() { changed = true; }
      if ui.checkbox(&mut self.objective.overload, create_text("Ovrl")).changed() { changed = true; }

      if ui.add(egui::TextEdit::singleline(&mut self.player_input_string)
        .desired_width(20.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE)).changed() {

        if let Ok(player_count) = self.player_input_string.parse::<u8>() {
          self.objective.player_count = player_count;
        }

      }

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