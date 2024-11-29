#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

pub mod logs;

pub mod splitter_app;
pub mod base_app;
pub mod time;
pub mod timed_run;
pub mod objective_data;
pub mod parse_files;
pub mod save_run;
pub mod objective_data_req;
pub mod sorter_window;

pub mod log_parser_window;
pub mod run_manager_window;

fn main() -> eframe::Result {

  // for (id, val) in std::env::vars() {
  //   println!("{}: {}", id, val);
  // }
  
  let options_base_app = eframe::NativeOptions {
    viewport: egui::ViewportBuilder {
      decorations: Some(true),
      resizable: Some(true),
      //position: Some([0.0, 128.0].into()),
      inner_size: Some([932.0, 512.0].into()),
      //window_level: Some(egui::WindowLevel::AlwaysOnTop),
      //transparent: Some(true),
      ..Default::default()
    },
    hardware_acceleration: eframe::HardwareAcceleration::Preferred,
    ..Default::default()
  };

  eframe::run_native(
    "GTFO Logger",
    options_base_app,
    Box::new(|_| {
      Ok(Box::<base_app::BaseApp>::default())
    }),
  )

}
