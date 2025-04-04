use egui::{FontId, RichText};

pub mod base_app;
pub mod sorter_window;
pub mod log_parser_window;
pub mod run_manager_window;
pub mod settings_window;

pub mod traits;
pub mod live_parser;

pub mod implementations;
pub mod live;

pub mod folder_watcher;

pub fn create_text<T>(text: T) -> egui::RichText
where T: Into<RichText> {
  Into::<RichText>::into(text)
    .color(egui::Color32::from_rgb(200, 200, 200))
    .font(FontId::new(12.0, egui::FontFamily::Name("jetbrains_mono".into())))
}
