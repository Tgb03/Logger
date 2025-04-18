use core::{logs::parser::ParserResult, parse_files::file_parse::AwaitParseFiles};

use egui::ProgressBar;

use crate::render::Render;


impl<T> Render for AwaitParseFiles<T>
where
    T: From<ParserResult> {
    
    type Response = bool;
    
    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        self.update();

        ui.vertical_centered(|ui| {
            ui.label(format!("Files left to parse: {} out of {}", self.get_left(), self.get_len()));
            ui.label(format!("Files/frame: {:.2}", (self.get_len() - self.get_left()) as f64 / self.get_frames() as f64));
            ui.label(format!("Percentage Done: {:.2}%", (self.get_len() - self.get_left()) as f64 * 100.0 / self.get_len() as f64));

            ui.add(ProgressBar::new((self.get_len() - self.get_left()) as f32 / self.get_len() as f32));
                
        });
        self.is_done()
    }
}
