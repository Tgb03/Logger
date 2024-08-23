#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

pub mod app;
pub mod time;
pub mod time_manager;
pub mod log_handler;

fn main() -> eframe::Result {
  
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder {
      decorations: Some(false),
      resizable: Some(true),
      position: Some([0.0, 128.0].into()),
      inner_size: Some([196.0, 256.0].into()),
      window_level: Some(egui::WindowLevel::AlwaysOnTop),
      transparent: Some(true),
      ..Default::default()
    },
    ..Default::default()
  };

  eframe::run_native(
    "My egui App",
    options,
    Box::new(|cc| {
      Ok(Box::<app::MyApp>::default())
    }),
  )

}
