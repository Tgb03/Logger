#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod windows;

pub mod render;
pub mod sorter_buttons;

pub mod base_app;
pub mod run;

use base_app::BaseApp;
use eframe::egui;
use egui::{Color32, Visuals};

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

    eframe::run_native(
        "GTFO Logger",
        options_base_app,
        Box::new(|cc| {
            let mut visuals = Visuals::dark();
            visuals.override_text_color = Some(Color32::from_rgb(225, 225, 225));
            cc.egui_ctx.set_visuals(visuals);
            Ok(Box::new(BaseApp::new(cc)))
        }),
    )
}
