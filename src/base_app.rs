use std::{fs::File, time::Duration};

use egui::{Color32, Vec2};

use crate::{log_parser_window::LogParserWindow, parse_files::file_parse::parse_all_files, run_manager_window::RunManagerWindow, save_run::SaveManager};

enum AppState {

  None,
  LogParserWindow,
  ManagingRuns,

}

pub struct BaseApp {
  
  level_name: String,
  secondary_included: bool,
  overload_included: bool,
  glitched: bool,
  early_drop: bool,

  app_state: AppState,

  log_parser_window: LogParserWindow,
  run_manager_window: RunManagerWindow,
  save_manager: SaveManager,

}

impl Default for BaseApp {
  fn default() -> Self {
    Self { 
      level_name: String::new(),
      secondary_included: false,
      overload_included: false,
      glitched: false,
      early_drop: false,

      app_state: AppState::None,

      log_parser_window: LogParserWindow::new(),
      run_manager_window: RunManagerWindow::new(),
      save_manager: SaveManager::new(),
    }
  }
}

impl eframe::App for BaseApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    ctx.request_repaint_after(Duration::from_millis(25));
    
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

            let parse_result = parse_all_files(files);
            self.log_parser_window.set_times(parse_result.get_timed_runs());
            self.app_state = AppState::LogParserWindow;
          }
        }

        if ui.button("Check Saved Runs").clicked() {
          self.app_state = AppState::ManagingRuns;
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

      match self.app_state {
        AppState::None => {},
        AppState::LogParserWindow => self.log_parser_window.show(ui, &mut self.save_manager),
        AppState::ManagingRuns => self.run_manager_window.show(ui, &mut self.save_manager),
      }
      
    });
    

    // if let Some(path) = self.file_dialog.update(ctx).selected() {
    //   println!("Selected file: {:?}", path);
    // }
  }
}

