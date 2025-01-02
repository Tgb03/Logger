use std::{fs::File, time::Duration};

use egui::{Color32, Frame, Vec2};

use crate::{graphics::{log_parser_window::LogParserWindow, run_manager_window::RunManagerWindow}, parse_files::file_parse::parse_all_files_async, save_run::SaveManager};

use super::live_window::LiveWindow;

#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
enum AppState {

  None,
  LogParserWindow,
  ManagingRuns,
  LiveWindow,

}

pub struct BaseApp {

  app_state: AppState,

  log_parser_window: LogParserWindow,
  run_manager_window: RunManagerWindow,
  live_window: LiveWindow,

  save_manager: SaveManager,

}

impl Default for BaseApp {
  fn default() -> Self {

    Self {
      app_state: AppState::None,

      log_parser_window: LogParserWindow::default(),
      run_manager_window: RunManagerWindow::default(),
      live_window: LiveWindow::default(),
      save_manager: SaveManager::default(),
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
      fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 50),
      stroke: egui::Stroke::new(1.0, Color32::TRANSPARENT),
    };

    egui::TopBottomPanel::top("TopPanel").frame(frame).show(ctx, |ui| {
      
      ui.horizontal_top(|ui| {
        if self.app_state == AppState::LiveWindow {
          if ui.button("Stop Splitter").clicked() {
            self.app_state = AppState::None;

            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 {x: 1024.0, y: 512.0 }));
          }

          return;
        }

        if ui.button("Live Splitter").clicked() {
          self.app_state = AppState::LiveWindow;
          self.live_window.load_file();
          ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
          ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
        }
        
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

            // let parse_result = parse_all_files(&files);
            let parse_result = parse_all_files_async(files);
            self.log_parser_window.set_times(parse_result.into());
            self.app_state = AppState::LogParserWindow;
          }
        }

        if ui.button("Check Saved Runs").clicked() {
          self.app_state = AppState::ManagingRuns;
        }
      })
    });
    
    egui::CentralPanel::default()
      .frame(Frame::default().fill(Color32::TRANSPARENT))
      .show(ctx, |ui| {

      match self.app_state {
        AppState::None => {},
        AppState::LogParserWindow => self.log_parser_window.show(ui, &mut self.save_manager),
        AppState::ManagingRuns => self.run_manager_window.show(ui, &mut self.save_manager),
        AppState::LiveWindow => self.live_window.show(ui, &mut self.save_manager, ctx),
      }
      
    });
    

    // if let Some(path) = self.file_dialog.update(ctx).selected() {
    //   println!("Selected file: {:?}", path);
    // }
  }
}

