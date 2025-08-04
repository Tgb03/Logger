use std::time::Instant;

use egui::{Color32, RichText};
use glr_core::run_gen_result::RunGeneratorResult;
use glr_lib::dll_exports::enums::SubscribeCode;

use crate::{dll::parse_continously::ContinousParser, render::Render};


pub struct Timer {

    start: Instant,
    active: bool,
    continous_parser: ContinousParser<RunGeneratorResult>,

}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            active: false,
            continous_parser: ContinousParser::new(SubscribeCode::RunInfo)
        }
    }
}

impl Render for Timer {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        while let Some(r) = self.continous_parser.try_recv() {
            match r {
                RunGeneratorResult::GameStarted(_, _) => {
                    self.active = true;
                    self.start = Instant::now();
                },
                RunGeneratorResult::LevelRun(_) => {
                    self.active = false;
                },
                _ => {}
            }
        }

        if self.active == false {
            return 0;
        }
        
        let total_secs = self.start.elapsed().as_secs();
        
        ui.colored_label(
            Color32::GREEN, Into::<RichText>::into(
                format!(" {:02}:{:02}:{:02}", 
                    total_secs / 3600, 
                    (total_secs % 3600) / 60, 
                    total_secs % 60
                )
            ).size(32.0)
        );

        ui.separator();

        54
    }
}
