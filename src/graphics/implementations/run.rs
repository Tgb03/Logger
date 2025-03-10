
use std::ops::Range;

use egui::Color32;

use crate::{graphics::{create_text, traits::{RenderResult, RenderRun}}, run::{objectives::{objective_enum::ObjectiveEnum, Objective}, time::Time, traits::Run}, save_run::SaveManager};


impl<T> RenderRun for T
where T: Run {
  fn show(&self, min_sizes: &Vec<usize>, range: Range<usize>, save_manager: &SaveManager, ui: &mut egui::Ui, show_split_times: bool) -> RenderResult {
    let mut result = RenderResult::default();
    let empty_vec = Vec::new();
    let objective_str = self.get_objective_str();
    let objective = self.get_objective::<ObjectiveEnum>().unwrap();
    let split_names = save_manager.get_split_names(&objective_str).unwrap_or(&empty_vec);

    ui.horizontal(|ui| {

      let time = self.get_time();
      
      let color = match self.is_win() {
        true => Color32::GREEN,
        false => Color32::RED,
      };
      
      ui.label(create_text("RUN:"));
      ui.colored_label(color, create_text(time.to_string()));

      ui.label(create_text(objective.get_player_count().to_string()));
      
      if ui.button(create_text(format!("DELETE RUN"))).clicked() {
        result.delete = true;
      }
      ui.label(create_text(format!("{:03}", self.len())));
      

      let mut running_total = Time::default();
      for id in 0..range.start {
        if let Some(time) = self.get_time_for_split(&split_names[id]) {
          running_total = running_total.add(&time);
        } 
      }

      let first = range.start;

      for id in range {
        if let Some(time) = self.get_time_for_split(&split_names[id]) {
          
          if show_split_times {
            let color = match save_manager.get_best_split(&objective_str, &split_names[id]).is_some_and(|v| v.is_equal(&time)) {
              true => Color32::GREEN,
              false => Color32::GRAY,
            };

            ui.colored_label(color, create_text(format!("{: ^fill$}", time.to_string(), fill = min_sizes[id - first])));
          } else {
            running_total = running_total.add(&time);
            ui.colored_label(Color32::GRAY, create_text(format!("{: ^fill$}", running_total.to_string(), fill = min_sizes[id - first])));
          }
        } else {
          ui.label(create_text("            "));
        }
      }

    });

    result
  }
}
