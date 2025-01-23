use egui::{Color32, Ui};
use strum::IntoEnumIterator;

use crate::{game_runs::{levels::GameRunRundown, objectives::GameRunObjective}, save_run::SaveManager};

pub struct FullRunManagerWindow {

  objective: GameRunObjective,
  rundown: GameRunRundown,

  player_input_string: String,
  player_count: u8,

}

impl Default for FullRunManagerWindow {
  fn default() -> Self {
    Self { 
      objective: GameRunObjective::AnyPercent, 
      rundown: GameRunRundown::Rundown1, 
      player_input_string: "1".to_string(), 
      player_count: 1,
    }
  }
}

impl FullRunManagerWindow {

  pub fn show(&mut self, ui: &mut Ui, save_manager: &mut SaveManager) {
    
    ui.horizontal(|ui| {
      egui::ComboBox::from_label("")
      .selected_text(super::create_text(format!("{}", self.rundown)))
      .height(256.0)
      .show_ui(ui, |ui| {
        
        for key in GameRunRundown::iter() {
          ui.selectable_value(&mut self.rundown, key.clone(), super::create_text(key.to_string()));
        }

      });
      
      egui::ComboBox::from_label(" ")
      .selected_text(super::create_text(format!("{}", self.objective)))
      .height(256.0)
      .show_ui(ui, |ui| {
        
        for key in GameRunObjective::iter() {
          ui.selectable_value(&mut self.objective, key.clone(), super::create_text(key.to_string()));
        }

      });

      ui.label(super::create_text("Player count: "));

      if ui.add(egui::TextEdit::singleline(&mut self.player_input_string)
          .desired_width(100.0)
        ).changed() {
        
        let player_count = self.player_input_string.parse::<u8>().ok();
        
        if let Some(player_count) = player_count {
          self.player_count = player_count;
        }
      }
    });

    ui.separator();

    ui.horizontal(|ui| {
      if ui.button(super::create_text("Save run to files")).clicked() {
        save_manager.save_to_file_full_game(self.rundown.clone(), self.objective.clone(), self.player_count);
      }
    });

    let runs = save_manager.get_game_runs(&format!("{}_{}_{}p.rsave", self.objective, self.rundown, self.player_count));

    if let Some(runs) = runs {

      ui.separator();

      egui::ScrollArea::vertical().show_rows(ui, ui.text_style_height(&egui::TextStyle::Body), runs.len(), |ui, row_range| {
        
        for row in row_range {
          let run = &runs[row];

          ui.horizontal(|ui| {
            let color = match run.2 {
              true => Color32::GREEN,
              false => Color32::RED,
            };

            ui.colored_label(color, super::create_text(run.1.to_string()));
          });
        }

      });
    }
  }

}


