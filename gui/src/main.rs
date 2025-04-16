#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod render;
pub mod sorter_buttons;

pub mod live;
pub mod windows;

pub mod base_app;
pub mod run;

use core::logs::collectable_mapper::CollectableMapper;

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

    let collectable_mapper =
        CollectableMapper::load_from_web().or(CollectableMapper::load_from_file());

    eframe::run_native(
        "GTFO Logger",
        options_base_app,
        Box::new(|cc| {
            let mut visuals = Visuals::dark();
            visuals.override_text_color = Some(Color32::from_rgb(225, 225, 225));
            cc.egui_ctx.set_visuals(visuals);
            Ok(Box::new(BaseApp::new(cc, collectable_mapper.as_ref())))
        }),
    )
}
