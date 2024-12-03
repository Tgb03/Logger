use egui::Color32;

use crate::{objective_data::ObjectiveData, save_run::SaveManager, time::Time};

use super::sorter_window::add_sorter_buttons;



pub struct RunManagerWindow {

  objective: ObjectiveData,

  player_input_string: String,

}

impl RunManagerWindow {
  pub fn new() -> RunManagerWindow {
    RunManagerWindow {
      objective: ObjectiveData::new(),
      player_input_string: String::new(),
    }
  }

  fn sum_run_splits(run: &Vec<Time>) -> Time {
    let mut result = Time::new();

    for time in run {
      result = result.add(time);
    }

    result
  }

  pub fn show(&mut self, ui: &mut egui::Ui, save_manager: &mut SaveManager) {
    let best_splits = save_manager.get_best_splits(&self.objective);

    ui.horizontal(|ui| {
      ui.label("Level name: ");
      ui.add(egui::TextEdit::singleline(&mut self.objective.level_name)
          .desired_width(60.0));
        ui.label("Player count: ");
      ui.add(egui::TextEdit::singleline(&mut self.player_input_string)
          .desired_width(30.0));
      ui.checkbox(&mut self.objective.secondary, "Secondary");
      ui.checkbox(&mut self.objective.overload, "Overload");
      ui.checkbox(&mut self.objective.glitched, "Glitched");
      ui.checkbox(&mut self.objective.early_drop, "Early Drop Bug");

      self.objective.level_name = self.objective.level_name.to_uppercase();
      if let Some(count) = self.player_input_string.parse::<u8>().ok() {
        self.objective.player_count = count;
      }
      
      ui.colored_label(Color32::GOLD, "  Theoretical:");
      ui.colored_label(Color32::GOLD, Self::sum_run_splits(&best_splits).to_string());

    });
      
    // handles all sorters
    if let Some(mut runs) = save_manager.get_runs(&self.objective) {
      add_sorter_buttons(ui, &mut runs);
    }

    ui.horizontal(|ui| {
      ui.label("Best split for each part:                              ");
      for stamp in &best_splits {
        ui.label(stamp.to_string_no_hours());
      }
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
      if let Some(runs) = save_manager.get_runs(&self.objective) {
        
        let mut for_deletion = Vec::new();

        for (id, timed_run) in runs.iter().enumerate() {
          ui.horizontal(|ui| {

            ui.colored_label(Color32::WHITE, &timed_run.objective_data.level_name);
  
            let time_color = match timed_run.win {
              true => Color32::GREEN,
              false => Color32::RED,
            };
            let times = timed_run.get_times();
  
            ui.colored_label(time_color, timed_run.get_time().to_string());
            ui.label(format!("{:03}", times.len()));

            if ui.button("Delete Run").clicked() {
              for_deletion.push(id);
            }

            for (id, stamp) in timed_run.get_splits().iter().enumerate() {
              let time_color = match best_splits.len() > id && stamp.is_equal(&best_splits[id]) {
                true => Color32::GREEN,
                false => Color32::RED,
              };
              ui.colored_label(time_color, stamp.to_string_no_hours());
            }
          });
        }

        for it in for_deletion.iter().rev() {
          runs.remove(*it);
        }
      }
    });
  }
}
