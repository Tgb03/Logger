use std::time::{Instant, SystemTime};

use egui::{Color32, RichText};
use chrono::{DateTime, Utc};
use glr_core::run_gen_result::RunGeneratorResult;
use glr_lib::dll_exports::enums::SubscribeCode;

use crate::{dll::parse_continously::ContinousParser, render::Render, windows::settings_window::SettingsWindow};

pub struct Timer {
    start: Instant,
    active: bool,
    show_milliseconds: bool,
    run_gen_parser: ContinousParser<RunGeneratorResult>,
}

impl Timer {
    pub fn new(settings: &SettingsWindow) -> Self {
        Self {
            start: Instant::now(),
            active: false,
            show_milliseconds: settings.get("show_timer_milliseconds").unwrap_or_default(),
            run_gen_parser: ContinousParser::new(SubscribeCode::RunInfo),
        }
    }
}

impl Render for Timer {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        while let Some(r) = self.run_gen_parser.try_recv() {
            match r {
                RunGeneratorResult::GameStarted(_, _, utc) => {
                    self.active = true;
                    self.start = instant_from_datetime(utc);
                }
                RunGeneratorResult::LevelRun(_) => {
                    self.active = false;
                }
                _ => {}
            }
        }

        if self.active == false {
            return 0;
        }

        let total_millis = self.start.elapsed().as_millis();
        
        let hours = total_millis / 3_600_000;
        let minutes = (total_millis / 60_000) % 60;
        let seconds = (total_millis / 1_000) % 60;
        let millis = total_millis % 1_000;
        
        let text = match (self.show_milliseconds, hours > 0) {
            (true, true) => format!(" {}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis),
            (true, false) => format!(" {:02}:{:02}.{:03}", minutes, seconds, millis),
            (false, true) => format!(" {}:{:02}:{:02}", hours, minutes, seconds),
            (false, false) => format!(" {:02}:{:02}", minutes, seconds),
        };

        ui.colored_label(
            Color32::GREEN,
            Into::<RichText>::into(text)
                .size(32.0),
        );

        ui.separator();

        54
    }
}

fn instant_from_datetime(dt: DateTime<Utc>) -> Instant {
    let now_instant = Instant::now();
    let now_system = SystemTime::now();

    let dt_system: SystemTime = dt.into();

    match now_system.duration_since(dt_system) {
        Ok(diff) => now_instant - diff,
        Err(e) => now_instant + e.duration(), // dt is in the future
    }
}