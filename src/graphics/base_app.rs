use std::{fs::File, time::Duration};

use egui::{Color32, Vec2};

use crate::{graphics::{log_parser_window::LogParserWindow, run_manager_window::RunManagerWindow}, parse_files::file_parse::parse_all_files_async, save_run::SaveManager};

enum AppState {

  None,
  LogParserWindow,
  ManagingRuns,

}

pub struct BaseApp {

  app_state: AppState,

  log_parser_window: LogParserWindow,
  run_manager_window: RunManagerWindow,
  save_manager: SaveManager,

}

impl Default for BaseApp {
  fn default() -> Self {
    Self {
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

            let parse_result = parse_all_files_async(files);
            self.log_parser_window.set_times(parse_result.get_timed_runs());
            self.app_state = AppState::LogParserWindow;
          }
        }

        if ui.button("Check Saved Runs").clicked() {
          self.app_state = AppState::ManagingRuns;
        }
      })
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

