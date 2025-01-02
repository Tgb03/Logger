use egui::Color32;

use crate::{objective_data::ObjectiveData, save_run::SaveManager, time::Time};

use super::sorter_window::add_sorter_buttons;


#[derive(Default)]
pub struct RunManagerWindow {

  objective: ObjectiveData,

  player_input_string: String,

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

    ui.horizontal(|ui| {
       egui::ComboBox::from_label("Select loaded objective")
        .selected_text(format!("{}", &self.objective.get_id()))
        .height(500.0)
        .show_ui(ui, |ui| {
          
          for key in save_manager.get_all_objectives() {
            if ui.selectable_value(&mut self.objective, ObjectiveData::from_id(&key), key).clicked() {
              self.player_input_string = self.objective.player_count.to_string();
            };
          }
        }
      );

      if ui.button("Save run to PC").clicked() {
        save_manager.save_to_file(&self.objective);
      }
      
      if ui.button("Save ALL runs to PC").clicked() {
        save_manager.save_to_files();
      }

      if ui.button("Load runs for this objective").clicked() {
        save_manager.load(&self.objective);
      }

      if ui.button("Load ALL runs").clicked() {
        save_manager.load_all_runs();
      }

      if ui.button("Optimize runs").clicked() {
        save_manager.optimize_obj(&self.objective);
      }
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

    let timed_runs = match save_manager.get_runs(&self.objective) {
      Some(run) => run,
      None => return,
    };

    egui::ScrollArea::vertical().show_rows(ui, ui.text_style_height(&egui::TextStyle::Body), timed_runs.len(), |ui, row_range| {
      let mut for_deletion = Vec::new();

      for row in row_range {
        let timed_run = &mut timed_runs[row];
        ui.horizontal(|ui| {

          ui.colored_label(Color32::WHITE, &timed_run.objective_data.level_name);
  
          let time_color = match timed_run.is_win() {
            true => Color32::GREEN,
            false => Color32::RED,
          };
          let times = timed_run.get_times();
  
          ui.colored_label(time_color, timed_run.get_time().to_string());
          ui.label(format!("{:03}", times.len()));

          if ui.button("Delete Run").clicked() {
            for_deletion.push(row);
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
        timed_runs.remove(*it);
      }
    });
  }
}
