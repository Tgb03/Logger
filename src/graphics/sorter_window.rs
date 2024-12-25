use egui::Ui;

use crate::{time::Time, timed_run::TimedRun};


fn get_total_times(timed_runs: &Vec<TimedRun>) -> Time {
  let mut total: Time = Time::new();
  
  for timed_run in timed_runs {
    total = total.add(&timed_run.get_time());
  }

  total
}

pub fn add_sorter_buttons(ui: &mut Ui, timed_runs: &mut Vec<TimedRun>) {
  ui.horizontal(|ui| {
    ui.label(format!("Total times added: {}", get_total_times(timed_runs).to_string()));
    
    if ui.button("Sort by Win").clicked() {
      timed_runs.sort_by(|d, e| d.is_win().cmp(&e.is_win()).reverse());
    }

    if ui.button("Sort by name").clicked() {
      timed_runs.sort_by(|d, e| d.objective_data.level_name.cmp(&e.objective_data.level_name));
    }
    
    if ui.button("Sort by time").clicked() {
      timed_runs.sort_by(|d, e| d.get_time().get_stamp().cmp(&e.get_time().get_stamp()));
    }

    if ui.button("Sort by Players").clicked() {
      timed_runs.sort_by(|d, e| d.objective_data.get_player_count().cmp(&e.objective_data.get_player_count()));
    }

    if ui.button("Sort by Stamps").clicked() {
      timed_runs.sort_by(|d, e| d.get_times().len().cmp(&e.get_times().len()).reverse());
    }
  });
}