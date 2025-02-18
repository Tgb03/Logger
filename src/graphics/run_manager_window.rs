use egui::Color32;

use crate::{run::{objectives::objective_enum::ObjectiveEnum, time::Time}, save_run::SaveManager};

use super::{sorter_window::add_sorter_buttons, traits::RenderRun};


#[derive(Default)]
pub struct RunManagerWindow {

  objective: ObjectiveEnum,
  show_split_times: bool,

}

impl RunManagerWindow {

  fn sum_run_splits(run: impl Iterator<Item = Time>) -> Time {
    let mut result = Time::new();

    for time in run {
      result = result.add(&time);
    }

    result
  }

  pub fn show(&mut self, ui: &mut egui::Ui, save_manager: &mut SaveManager) {

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
        
      if let Some(best_splits) = save_manager.get_best_splits(&self.objective) {
      
        ui.colored_label(Color32::GOLD, super::create_text("Theoretical:"));
        ui.colored_label(Color32::GOLD, super::create_text(
          Self::sum_run_splits(
            save_manager.get_split_names(&self.objective)
              .unwrap_or(&Vec::new())
              .iter()
              .map(|name| best_splits.get(name).cloned().unwrap_or_default())
          ).to_string()
        ));
        
      }

      ui.checkbox(&mut self.show_split_times, super::create_text("Show Split Times"));

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
    if let Some(runs) = save_manager.get_runs_mut(&self.objective) {
      add_sorter_buttons(ui, runs);
    }

    ui.horizontal(|ui| {
      ui.vertical(|ui| {
      
        ui.label(super::create_text("Name of splits:                    "));
        ui.label(super::create_text("Best split for each part:          "));

      });

      if let Some(vec) = save_manager.get_split_names(&self.objective) {
        if let Some(hash_map) = save_manager.get_best_splits(&self.objective) {
          for name in vec {
            ui.vertical(|ui| {
              ui.label(super::create_text(format!("{: ^12}", name)));
              ui.label(super::create_text(hash_map.get(name).unwrap_or(&Time::default()).to_string()));
            });
          }
        }
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
        let timed_run = &timed_runs[row];
        
        let result = timed_run.show(&save_manager, ui, self.show_split_times);
        
        if result.delete { for_deletion.push(row); has_deleted = true; }
      }
    });

    let timed_runs = match save_manager.get_runs_mut(&self.objective) {
      Some(run) => run,
      None => return,
    };
    for it in for_deletion.iter().rev() {
      timed_runs.remove(*it);
    }
    
    if has_deleted {
      save_manager.calculate_best_splits(self.objective.clone());
    }
  }
}
