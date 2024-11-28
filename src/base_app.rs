use std::{cmp::Ordering, fs::File};

use egui::{Color32, Vec2};

use crate::{parse_files::file_parse::parse_all_files, time::Time, timed_run::TimedRun};


pub struct BaseApp {
  
  level_name: String,
  secondary_included: bool,
  overload_included: bool,
  glitched: bool,
  early_drop: bool,

  timed_runs: Vec<TimedRun>,

  set_all_secondary: bool,
  set_all_overload: bool,
  set_all_glitched: bool,
  set_all_early_drop: bool,

}

impl BaseApp {

  fn get_total_times(&self) -> Time {
    let mut total: Time = Time::new();
    
    for timed_run in &self.timed_runs {
      total = total.add(&timed_run.get_time());
    }

    total
  }

}

impl Default for BaseApp {
  fn default() -> Self {

    Self { 
      level_name: String::new(),
      secondary_included: false,
      overload_included: false,
      glitched: false,
      early_drop: false,

      timed_runs: Vec::new(),
      set_all_secondary: false,
      set_all_overload: false,
      set_all_glitched: false,
      set_all_early_drop: false,
    }
  }
}

impl eframe::App for BaseApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    let frame = egui::containers::Frame {
      inner_margin: egui::Margin { left: 1., right: 1., top: 1., bottom: 1.},
      outer_margin: egui::Margin { left: 5., right: 1., top: 1., bottom: 1.},
      rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0},
      shadow: egui::Shadow { offset: Vec2::ZERO, blur: 0.0, spread: 0.0, color: Color32::TRANSPARENT },
      fill: Color32::BLACK,
      stroke: egui::Stroke::new(1.0, Color32::TRANSPARENT),
    };
    egui::TopBottomPanel::top("TopPanel").frame(frame).show(ctx, |ui| {
      ui.horizontal_top(|ui| {
        //ui.button("Start AutoSplitter");
        if ui.button("Input Speedrun Logs...").clicked() {
          if let Some(paths) = rfd::FileDialog::new().pick_files() {
            let files: Vec<File> = paths.iter()
              .filter_map(|p| {
                match File::open(p) {
                  Ok(file) => Some(file),
                  Err(_) => {println!("Failed to parse {:?}", p); None},
                }
              })
              .collect();

            self.timed_runs = parse_all_files(files);
          }
        }
      })
    });
    egui::SidePanel::left("LeftPanel").frame(frame).show(ctx, |ui| {
      ui.horizontal( |ui| {
        ui.label("Level Name: ");
        ui.text_edit_singleline(&mut self.level_name)
      });
      ui.horizontal( |ui| {
        ui.checkbox(&mut self.secondary_included, "secondary");
        ui.checkbox(&mut self.overload_included, "overload");
      });
      ui.horizontal( |ui| {
        ui.checkbox(&mut self.glitched, "glitch");
        ui.checkbox(&mut self.early_drop, "early_drop");
      });
      let mut level_id: String = self.level_name.to_string().to_uppercase();
      if self.secondary_included { level_id += "_sec"; }
      if self.overload_included { level_id += "_ovrl"; }
      if self.glitched { level_id += "_glitch"; }
      if self.early_drop { level_id += "_edrop"; }
      ui.label(format!("ID: {}", level_id));
    });
    
    egui::CentralPanel::default().show(ctx, |ui| {

      // handles all the set all buttons.
      ui.horizontal(|ui| {
        ui.label(format!("Total times added: {}", self.get_total_times().to_string()));
        
        if ui.button("Sort by name").clicked() {
            self.timed_runs.sort_by(|d, e| d.level_name.cmp(&e.level_name));
        }
        
        if ui.button("Sort by time").clicked() {
          self.timed_runs.sort_by(|d, e| d.get_time().get_stamp().cmp(&e.get_time().get_stamp()));
        }
      });

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
            ui.colored_label(Color32::WHITE, &timed_run.level_name);

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

              let serialized = bincode::serialize(&timed_run).unwrap();
              println!("Serialized: {:?}", serialized);

              let deserialized: TimedRun = bincode::deserialize(&serialized).unwrap();
              println!("Deserialized: {:?}", deserialized);
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
      
    });
    

    // if let Some(path) = self.file_dialog.update(ctx).selected() {
    //   println!("Selected file: {:?}", path);
    // }
  }
}

