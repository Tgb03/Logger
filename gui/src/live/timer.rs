use core::logs::live_parser::LiveParser;
use std::time::{Duration, Instant};

use egui::{Color32, RichText};

use crate::render::{BufferedRender, Render};

pub struct Timer {

    start: Instant,
    duration: Duration,
    stop_updating: bool,

}

#[derive(Default)]
pub struct BufferedTimer {

    timer: Timer,

}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            duration: Duration::default(),
            stop_updating: false,
        }
    }
}

impl Render for Timer {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {

        if self.stop_updating {
            return 0;
        }

        self.duration = self.start.elapsed();
        
        let total_secs = self.duration.as_secs();
        
        ui.colored_label(
            Color32::GREEN, Into::<RichText>::into(
                format!(" {:02}:{:02}:{:02}", 
                    total_secs / 3600, 
                    (total_secs % 3600) / 60, 
                    total_secs % 60
                )
            ).size(32.0)
        );

        return 48

    }
}

impl BufferedRender for BufferedTimer {
    type Response = usize;
    type UpdateData = LiveParser;
    type Render = Timer;

    fn update(&mut self, live_parser: &Self::UpdateData) {

        match live_parser.get_run_parser().is_some_and(|v| v.run_started()) {
            true => { 
                if self.timer.stop_updating {
                    self.timer = Timer::default();
                }
             },
            false => { self.timer.stop_updating = true }
        }

    }

    fn reset(&mut self) {}

    fn get_renderer(&mut self) -> &mut Self::Render {
        &mut self.timer
    }
}
