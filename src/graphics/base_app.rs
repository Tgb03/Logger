use std::{collections::BTreeMap, fs::File, time::Duration};

use eframe::CreationContext;
use egui::{Color32, FontData, FontDefinitions, FontFamily, Frame, Vec2};

use crate::{graphics::{log_parser_window::LogParserWindow, run_manager_window::RunManagerWindow}, parse_files::file_parse::parse_all_files_async, run::timed_run::LevelRun, save_run::SaveManager};

use super::{live_window::LiveWindow, settings_window::SettingsWindow};

#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
enum AppState {

  None,
  LogParserWindow,
  ManagingRuns,
  LiveWindow,
  SettingsWindow,

}

pub struct BaseApp<'a> {

  app_state: AppState,

  log_parser_window: LogParserWindow,
  run_manager_window: RunManagerWindow,
  live_window: LiveWindow<'a>,
  settings_window: SettingsWindow,

  save_manager: SaveManager,

}

impl<'a> Default for BaseApp<'a> {
  fn default() -> Self {
    let settings_window = SettingsWindow::default();
    let mut save_manager = SaveManager::default();

    if settings_window.get_automatic_loading() {
      save_manager.load_all_runs();
    }

    Self {
      app_state: AppState::None,

      log_parser_window: LogParserWindow::default(),
      run_manager_window: RunManagerWindow::default(),
      live_window: LiveWindow::default(),
      save_manager,
      settings_window,
    }
  }
}

impl<'a> BaseApp<'a> {

  pub fn new(cc: &CreationContext) -> Self {
    let mut fonts = FontDefinitions::default();
    
    fonts.font_data.insert("jetbrains_mono".to_owned(), 
      std::sync::Arc::new(
        FontData::from_static(include_bytes!("../../JetBrainsMono-Regular.ttf"))
      )
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(
      FontFamily::Name("jetbrains_mono".into()), 
      vec!["jetbrains_mono".to_owned()]
    );
    fonts.families.append(&mut newfam);

    cc.egui_ctx.set_fonts(fonts);

    Self::default()
  } 

}

impl<'a> eframe::App for BaseApp<'a> {
  
  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    match self.app_state {
      AppState::LiveWindow => [0.0, 0.0, 0.0, 0.5],
      _ => [0.0, 0.0, 0.0, 1.0]
    }
  }

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    ctx.request_repaint_after(Duration::from_millis(25));

    let frame = Frame::none()
      .fill(Color32::TRANSPARENT);

    egui::TopBottomPanel::top("TopPanel").frame(frame).show(ctx, |ui| {
      
      ui.horizontal_top(|ui| {
        if self.app_state == AppState::LiveWindow {
          if ui.button(super::create_text("Stop Splitter")).clicked() {
            self.app_state = AppState::None;

            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 {x: 1024.0, y: 512.0 }));
          }

          return;
        }

        if self.app_state == AppState::SettingsWindow {
          if ui.button(super::create_text("Save Settings")).clicked() {
            self.app_state = AppState::None;

            self.settings_window.save_settings();
          }

          return;
        }

        if ui.button(super::create_text("Live Splitter")).clicked() {
          self.app_state = AppState::LiveWindow;
          self.live_window.load_file(&self.settings_window);
          ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
          ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(self.settings_window.get_live_rectangle().min));
          ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(self.settings_window.get_live_rectangle().size()));
        }
        
        if ui.button(super::create_text("Input Speedrun Logs...")).clicked() {
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
            self.log_parser_window.set_times(
              Into::<Vec<LevelRun>>::into(parse_result)
            );
            self.app_state = AppState::LogParserWindow;
          }
        }

        if ui.button(super::create_text("Check Saved Runs")).clicked() {
          self.app_state = AppState::ManagingRuns;
        }

        if ui.button(super::create_text("Settings")).clicked() {
          self.app_state = AppState::SettingsWindow;
        }
      })
    });
    
    egui::CentralPanel::default()
      .frame(frame)
      .show(ctx, |ui| {

      match self.app_state {
        AppState::None => {},
        AppState::LogParserWindow => self.log_parser_window.show(ui, &mut self.save_manager),
        AppState::ManagingRuns => self.run_manager_window.show(ui, &mut self.save_manager),
        AppState::LiveWindow => self.live_window.show(ui, &mut self.save_manager, &self.settings_window, ctx),
        AppState::SettingsWindow => self.settings_window.show(ui),
      }
      
    });
    
    // if let Some(path) = self.file_dialog.update(ctx).selected() {
    //   println!("Selected file: {:?}", path);
    // }
  }
}

