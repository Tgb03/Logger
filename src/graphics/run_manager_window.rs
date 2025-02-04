use egui::Color32;

use crate::{run::{objectives::objective_enum::ObjectiveEnum, time::Time}, save_run::SaveManager};

use super::{sorter_window::add_sorter_buttons, traits::RenderRun};


#[derive(Default)]
pub struct RunManagerWindow {

  objective: ObjectiveEnum,

}

impl RunManagerWindow {

  fn sum_run_splits(run: &Vec<Time>) -> Time {
    let mut result = Time::new();

    for time in run {
      result = result.add(time);
    }

    result
  }

  pub fn show(&mut self, ui: &mut egui::Ui, save_manager: &mut SaveManager) {
    let best_splits = match save_manager.get_best_splits(&self.objective) {
      Some(v) => v.clone(),
      None => Vec::new(),
    };

    /*
    ui.horizontal(|ui| {
      ui.label(super::create_text("Level name: "));
      ui.add(egui::TextEdit::singleline(&mut self.objective.level_name)
          .desired_width(60.0)
          .background_color(Color32::from_rgb(16, 16, 16))
          .text_color(Color32::WHITE));
      ui.label(super::create_text("Player count: "));
      ui.add(egui::TextEdit::singleline(&mut self.player_input_string)
          .desired_width(15.0)
          .background_color(Color32::from_rgb(16, 16, 16))
          .text_color(Color32::WHITE));
      ui.checkbox(&mut self.objective.secondary, super::create_text("Secondary"));
      ui.checkbox(&mut self.objective.overload, super::create_text("Overload"));
      ui.checkbox(&mut self.objective.glitched, super::create_text("Glitched"));
      ui.checkbox(&mut self.objective.early_drop, super::create_text("Early Drop Bug"));

      self.objective.level_name = self.objective.level_name.to_uppercase();
      if let Some(count) = self.player_input_string.parse::<u8>().ok() {
        self.objective.player_count = count;
      }
      
    });
    */

    ui.horizontal(|ui| {
       egui::ComboBox::from_label(super::create_text("Select loaded objective"))
        .selected_text(super::create_text(self.objective.to_string()))
        .height(500.0)
        .show_ui(ui, |ui| {
          
          for key in save_manager.get_all_objectives() {
            if ui.selectable_value(
              &mut self.objective, 
              ObjectiveEnum::from(key.clone()), 
              super::create_text(key.to_string())).clicked() {
              
            };
          }
        }
      );

      if ui.button(super::create_text("Remove useless runs")).clicked() {
        save_manager.optimize_obj(&self.objective);
      }
        
      ui.colored_label(Color32::GOLD, super::create_text("Theoretical:"));
      ui.colored_label(Color32::GOLD, super::create_text(Self::sum_run_splits(&best_splits).to_string()));

    });

    ui.separator();

    ui.horizontal(|ui| {
      
      if ui.button(super::create_text("Save run to PC")).clicked() {
        save_manager.save_to_file(&self.objective);
      }
      
      if ui.button(super::create_text("Save ALL runs to PC")).clicked() {
        save_manager.save_to_files();
      }

      if ui.button(super::create_text("Load runs for this objective")).clicked() {
        save_manager.load(&self.objective);
      }

      if ui.button(super::create_text("Load ALL runs")).clicked() {
        save_manager.load_all_runs();
      }

    });
      
    // handles all sorters
    if let Some(runs) = save_manager.get_runs(&self.objective) {
      add_sorter_buttons(ui, runs);
    }

    ui.horizontal(|ui| {
      ui.label(super::create_text("Best split for each part:         "));
      for stamp in &best_splits {
        ui.label(super::create_text(stamp.to_string_no_hours()));
      }
    });

    let timed_runs = match save_manager.get_runs(&self.objective) {
      Some(run) => run,
      None => return,
    };
    
    let mut has_deleted = false;
    let mut for_deletion = Vec::new();

    egui::ScrollArea::vertical().show_rows(ui, ui.text_style_height(&egui::TextStyle::Body), timed_runs.len(), |ui, row_range| {
      
      for row in row_range {
        let timed_run = &mut timed_runs[row];
        
        let result = timed_run.show(ui);
        
        if result.delete { for_deletion.push(row); has_deleted = true; }
      }

      for it in for_deletion.iter().rev() {
        timed_runs.remove(*it);
      }
    });
    
    if has_deleted {
      save_manager.calculate_best_splits(self.objective.clone());
    }
  }
}
