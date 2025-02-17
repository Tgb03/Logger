use egui::{Color32, Ui};

use crate::{graphics::create_text, run::{objectives::objective_enum::ObjectiveEnum, time::Time, traits::Run}, save_run::SaveManager};



pub struct RunRenderer {

  objective: ObjectiveEnum,

}

impl Default for RunRenderer {
  fn default() -> Self {
    Self {
      objective: ObjectiveEnum::default(),
    }
  }
}

impl RunRenderer {

  pub fn set_objective(&mut self, objective: ObjectiveEnum) {
    self.objective = objective;
  }

  pub fn render_run<T: Run>(ui: &mut Ui, run: &T, max_length: usize, save_manager: &SaveManager) -> usize {
    
    let size = run.len();
    let objective = run.get_objective::<ObjectiveEnum>().unwrap_or_default();
    let splits = run
      .get_splits()
      .skip(size.saturating_sub(max_length));

    let best_run = save_manager.get_best_run(&objective);
    
    let mut current_time = Time::default();
    let mut compared_run_time = Time::default();

    ui.vertical(|ui| {

      for split in splits {
        
        ui.horizontal(|ui| {
          
          let split_time = split.get_time();
          ui.label(create_text(split.get_name()));

          current_time = current_time.add(&split_time);
          ui.label(create_text(current_time.to_string()));
          
          if best_run.is_some() {
            compared_run_time = compared_run_time.add(
              &best_run.map(|r| r.get_time_for_split(split.get_name()))
                .flatten()
                .unwrap_or_default()
            );

            let (time, color) = match current_time.is_smaller_than(&compared_run_time) {
              true => (compared_run_time.sub(&current_time), Color32::GREEN),
              false => (current_time.sub(&compared_run_time), Color32::RED),
            };

            ui.colored_label(color, create_text(time.to_string_no_hours()));
          }

          if let Some(best_split) = save_manager.get_best_split(&objective, split.get_name()) {
            let (time, color) = match split_time.is_smaller_than(best_split) {
              true => (best_split.sub(&split_time), Color32::GREEN),
              false => (split_time.sub(best_split), Color32::RED),
            };

            ui.colored_label(color, create_text(time.to_string_no_hours()));
          }
        
        });
      }

    });

    ui.label(create_text(format!("Rendering: {}", objective)));

    (size + 1) * 22
  }

}
