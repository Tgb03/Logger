use egui::{Color32, Ui};
use strum::IntoEnumIterator;

use crate::{graphics::create_text, run::objectives::{game_objective::GameObjective, game_run_objective::GameRunObjective, game_run_rundown::GameRunRundown, Objective}};


pub struct GameObjectiveReader {
  
  objective: GameObjective,
  player_input_string: String,

}

impl Default for GameObjectiveReader {
  fn default() -> Self {
    let objective = GameObjective::default()
      .with_player_count(1);
    
    Self { 
      player_input_string: objective.player_count.to_string(),
      objective: objective, 
    }
  }
}

impl Into<GameObjective> for GameObjectiveReader {
  fn into(self) -> GameObjective {
    self.objective
  }
}

impl GameObjectiveReader {

  pub fn get_objective(&self) -> &GameObjective {
    &self.objective
  }

  pub fn show(&mut self, ui: &mut Ui) {
    
    ui.horizontal(|ui| {

      egui::ComboBox::from_label("")
        .selected_text(create_text(self.objective.rundown.to_string()))
        .height(500.0)
        .show_ui(ui, |ui| {
            
        for key in GameRunRundown::iter() {
          if ui.selectable_value(
            &mut self.objective.rundown, 
            key.clone(), 
            create_text(key.to_string())).clicked() {
            
          };
        }
      });

      egui::ComboBox::from_label(" ")
        .selected_text(create_text(self.objective.objective.to_string()))
        .height(500.0)
        .show_ui(ui, |ui| {
            
        for key in GameRunObjective::iter() {
          if ui.selectable_value(
            &mut self.objective.objective, 
            key.clone(), 
            create_text(key.to_string())).clicked() {
            
          };
        }
      });

      if ui.add(egui::TextEdit::singleline(&mut self.player_input_string)
        .desired_width(20.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE)).changed() {

        if let Ok(player_count) = self.player_input_string.parse::<u8>() {
          self.objective.player_count = player_count;
        }

      }

    });

  }

}