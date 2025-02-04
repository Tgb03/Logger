use egui::Ui;

use crate::run::{time::Time, traits::{Run, Timed}};


fn get_total_times<T: Timed>(timed_runs: &Vec<T>) -> Time {
  let mut total: Time = Time::new();
  
  for timed_run in timed_runs {
    total = total.add(&timed_run.get_time());
  }

  total
}

pub fn add_sorter_buttons<T>(ui: &mut Ui, timed_runs: &mut Vec<T>)
where T: Run {
  ui.horizontal(|ui| {
    ui.label(super::create_text(format!("Total times added: {}", get_total_times(timed_runs).to_string())));
    
    if ui.button(super::create_text("Sort by Win")).clicked() {
      timed_runs.sort_by(|d, e| d.is_win().cmp(&e.is_win()).reverse());
    }

    if ui.button(super::create_text("Sort by objective")).clicked() {
      timed_runs.sort_by(|d, e| d.get_objective_str().cmp(&e.get_objective_str()));
    }
    
    if ui.button(super::create_text("Sort by time")).clicked() {
      timed_runs.sort_by(|d, e| d.get_time().get_stamp().cmp(&e.get_time().get_stamp()));
    }

    // if ui.button(super::create_text("Sort by Players")).clicked() {
    //   timed_runs.sort_by(|d, e| d.get_objective().get_player_count().cmp(&e.get_objective().get_player_count()));
    // }

    if ui.button(super::create_text("Sort by Stamps")).clicked() {
      timed_runs.sort_by(|d, e| d.get_times().len().cmp(&e.get_times().len()).reverse());
    }
  });
}