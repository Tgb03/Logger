
use egui::Color32;

use crate::{graphics::{create_text, traits::{RenderResult, RenderRun}}, run::{objectives::objective_enum::ObjectiveEnum, time::Time, traits::Run}, save_run::SaveManager};


impl<T> RenderRun for T
where T: Run {
  fn show(&self, save_manager: &SaveManager, ui: &mut egui::Ui, show_split_times: bool) -> RenderResult {
    let mut result = RenderResult::default();
    let empty_vec = Vec::new();
    let objective = self.get_objective::<ObjectiveEnum>().unwrap();
    let split_names = save_manager.get_split_names(&objective).unwrap_or(&empty_vec);

    ui.horizontal(|ui| {

      let time = self.get_time();
      
      let color = match self.is_win() {
        true => Color32::GREEN,
        false => Color32::RED,
      };
      
      ui.label(create_text("RUN:"));
      ui.colored_label(color, create_text(time.to_string()));
      
      if ui.button(create_text(format!("DELETE RUN"))).clicked() {
        result.delete = true;
      }
      ui.label(create_text(format!("{:03}", self.len())));

      let mut running_total = Time::default();

      for obj in split_names {
        if let Some(time) = self.get_time_for_split(obj) {
          
          if show_split_times {
            let color = match save_manager.get_best_split(&objective, obj).is_some_and(|v| v.is_equal(&time)) {
              true => Color32::GREEN,
              false => Color32::GRAY,
            };

            ui.colored_label(color, create_text(time.to_string()));
          } else {
            running_total = running_total.add(&time);
            ui.colored_label(Color32::GRAY, create_text(running_total.to_string()));
          }
        } else {
          ui.label(create_text("            "));
        }
      }

    });

    result
  }
}
