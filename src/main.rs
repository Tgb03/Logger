#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use graphics::base_app;

pub mod logs;

pub mod time;
pub mod timed_run;
pub mod timed_run_parser;
pub mod objective_data;
pub mod parse_files;
pub mod save_run;
pub mod graphics;

fn main() -> eframe::Result {

  // for (id, val) in std::env::vars() {
  //   println!("{}: {}", id, val);
  // }

  
  
  let options_base_app = eframe::NativeOptions {
    viewport: egui::ViewportBuilder {
      //position: Some([0.0, 128.0].into()),
      inner_size: Some([932.0, 512.0].into()),
      //window_level: Some(egui::WindowLevel::AlwaysOnTop),
      transparent: Some(true),
      ..Default::default()
    },
    hardware_acceleration: eframe::HardwareAcceleration::Preferred,
    persist_window: true,
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
