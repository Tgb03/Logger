#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use graphics::base_app::BaseApp;

pub mod logs;
pub mod game_runs;

pub mod time;
pub mod timed_run;
pub mod objective_data;
pub mod parse_files;
pub mod save_run;
pub mod graphics;
pub mod key_guess;

fn main() -> eframe::Result {

  // for (id, val) in std::env::vars() {
  //   println!("{}: {}", id, val);
  // }

  
  let options_base_app = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([932.0, 512.0])
      .with_transparent(true),
    hardware_acceleration: eframe::HardwareAcceleration::Preferred,
    persist_window: true,
    ..Default::default()
  };

  eframe::run_native(
    "GTFO Logger",
    options_base_app,
    Box::new(|cc| {
      Ok(Box::new(BaseApp::new(cc)))
    }),
  )

}
