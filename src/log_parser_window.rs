use egui::{Color32, Ui};

use crate::{base_app::ShowUI, time::Time, timed_run::TimedRun};


pub struct LogParserWindow {

  timed_runs: Vec<TimedRun>,

  set_all_secondary: bool,
  set_all_overload: bool,
  set_all_glitched: bool,
  set_all_early_drop: bool,  

}

impl LogParserWindow {

  pub fn new() -> LogParserWindow {
    LogParserWindow { 
      timed_runs: Vec::new(), 
      set_all_secondary: false, 
      set_all_early_drop: false, 
      set_all_glitched: false, 
      set_all_overload: false 
    }
  }

  fn get_total_times(&self) -> Time {
    let mut total: Time = Time::new();
    
    for timed_run in &self.timed_runs {
      total = total.add(&timed_run.get_time());
    }

    total
  }

  pub fn set_times(&mut self, times: Vec<TimedRun>) {
    self.timed_runs = times;
  }

}

impl ShowUI for LogParserWindow {
  fn show(&mut self, ui: &mut Ui) {
    // handles all sorters
    ui.horizontal(|ui| {
      ui.label(format!("Total times added: {}", self.get_total_times().to_string()));
      
      if ui.button("Sort by Win").clicked() {
        self.timed_runs.sort_by(|d, e| d.win.cmp(&e.win).reverse());
      }

      if ui.button("Sort by name").clicked() {
          self.timed_runs.sort_by(|d, e| d.objective_data.level_name.cmp(&e.objective_data.level_name));
      }
      
      if ui.button("Sort by time").clicked() {
        self.timed_runs.sort_by(|d, e| d.get_time().get_stamp().cmp(&e.get_time().get_stamp()));
      }

      if ui.button("Sort by Players").clicked() {
        self.timed_runs.sort_by(|d, e| d.objective_data.get_player_count().cmp(&e.objective_data.get_player_count()));
      }

      if ui.button("Sort by Stamps").clicked() {
        self.timed_runs.sort_by(|d, e| d.get_times().len().cmp(&e.get_times().len()).reverse());
      }
    });

    // handles all the set all buttons.
    ui.horizontal(|ui| {
      let secondary_checkbox = ui.checkbox(&mut self.set_all_secondary, "Set ALL secondary");
      let overload_checkbox = ui.checkbox(&mut self.set_all_overload, "Set ALL overload");
      let glitched_checkbox = ui.checkbox(&mut self.set_all_glitched, "Set ALL glitched");
      let early_drop_checkbox = ui.checkbox(&mut self.set_all_early_drop, "Set ALL early drop");
    
      if secondary_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.objective_data.secondary = self.set_all_secondary;
        }
      }
      
      if overload_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.objective_data.overload = self.set_all_overload;
        }
      }

      if glitched_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.objective_data.glitched = self.set_all_glitched;
        }
      }

      if early_drop_checkbox.clicked() {
        for timed_run in &mut self.timed_runs {
          timed_run.objective_data.early_drop = self.set_all_early_drop;
        }
      }
    });
    
    ui.vertical(|ui| {
      let mut for_removal = Vec::new();

      for (id, timed_run) in self.timed_runs.iter_mut().enumerate() {
        ui.horizontal(|ui|{
          ui.colored_label(Color32::WHITE, &timed_run.objective_data.level_name);

          let time_color = match timed_run.win {
            true => Color32::GREEN,
            false => Color32::RED,
          };
          let times = timed_run.get_times();

          ui.colored_label(time_color, times.last().unwrap_or(&Time::new()).to_string());
          ui.label(format!("{:03} stamps", times.len()));
          ui.label(format!("{} players", timed_run.objective_data.get_player_count()));

          ui.checkbox(&mut timed_run.objective_data.secondary, "Secondary");
          ui.checkbox(&mut timed_run.objective_data.overload, "Overload");
          ui.checkbox(&mut timed_run.objective_data.glitched, "Glitched");
          ui.checkbox(&mut timed_run.objective_data.early_drop, "Early Drop");

          if timed_run.objective_data.early_drop { timed_run.objective_data.glitched = true; }

          
          if ui.button("Save Run").clicked() {
            for_removal.push(id);
          };

          if ui.button("Remove Run").clicked() {
            for_removal.push(id);
          }
          
        });
      }

      for id in for_removal.iter().rev() {
        self.timed_runs.remove(*id);
      }
    });
  }
}
