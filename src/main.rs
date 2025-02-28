#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::Visuals;
use graphics::base_app::BaseApp;

pub mod logs;
pub mod run;

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
    vsync: true,
    renderer: eframe::Renderer::Glow,
    persist_window: true,
    ..Default::default()
  };

  logs::collectable_mapper::CollectableMapper::init();
 
  eframe::run_native(
    "GTFO Logger",
    options_base_app,
    Box::new(|cc| {
      cc.egui_ctx.set_visuals(Visuals::dark());
      Ok(Box::new(BaseApp::new(cc)))
    }),
  )

}
