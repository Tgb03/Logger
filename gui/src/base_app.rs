use core::{ 
    run::timed_run::LevelRun, save_manager::SaveManager, version::{
        get_latest_version, 
        is_there_new_version
    }
};
use std::{
    collections::BTreeMap, path::PathBuf, time::Duration
};

use might_sleep::prelude::CpuLimiter;

use eframe::CreationContext;
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Frame, Vec2, WidgetText};
use opener::open;

use crate::{
    render::Render, windows::{await_parse_files::AwaitParseFiles, live_window::live_window::LiveWindow, log_parser_window::LogParserWindow, run_manager_window::RunManagerWindow, settings_window::SettingsWindow, stats_window::StatsWindow},
};

use crate::egui::TextStyle::{Body, Button, Heading, Monospace, Small};

enum AppState {
    None,
    SettingsWindow,
    AwaitParseLogWindow(Option<AwaitParseFiles>),
    AwaitParseStatWindow(Option<AwaitParseFiles>),
    LogParserWindow(LogParserWindow),
    ManagingRuns(RunManagerWindow),
    StatsWindow(StatsWindow),
    LiveWindow(LiveWindow),
}

pub struct BaseApp {
    app_state: AppState,
    live_window_size: Option<usize>,

    settings_window: SettingsWindow,
    save_manager: SaveManager,
    limiter: CpuLimiter,

    latest_version: Option<String>,
    new_version_warning: bool,
}

impl BaseApp {
    pub fn new(cc: &CreationContext) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "jetbrains_mono".to_owned(),
            std::sync::Arc::new(FontData::from_static(include_bytes!(
                "../../JetBrainsMono-Regular.ttf"
            ))),
        );

        let mut newfam = BTreeMap::new();
        newfam.insert(
            FontFamily::Name("jetbrains_mono".into()),
            vec!["jetbrains_mono".to_owned()],
        );
        fonts.families.append(&mut newfam);

        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_theme(egui::Theme::Dark);

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = <BTreeMap<egui::TextStyle, FontId>>::from([
            (
                Heading,
                FontId::new(12.0, FontFamily::Name("jetbrains_mono".into())),
            ),
            (
                Body,
                FontId::new(12.0, FontFamily::Name("jetbrains_mono".into())),
            ),
            (
                Monospace,
                FontId::new(12.0, FontFamily::Name("jetbrains_mono".into())),
            ),
            (
                Button,
                FontId::new(12.0, FontFamily::Name("jetbrains_mono".into())),
            ),
            (
                Small,
                FontId::new(12.0, FontFamily::Name("jetbrains_mono".into())),
            ),
        ]);
        cc.egui_ctx.set_style(style);

        let settings_window = SettingsWindow::default();
        let mut save_manager = SaveManager::default();
        save_manager.set_automatic_saving(settings_window.get_def("automatic_saving"));

        if settings_window.get_def("automatic_loading") {
            save_manager.load_all_runs();
        }

        let limiter = CpuLimiter::new(Duration::from_micros(16667));

        let latest_version = get_latest_version();
        let new_version_warning = match &latest_version {
            Some(ver) => is_there_new_version(ver)
                .unwrap_or(false),
            None => false,
        };

        Self {
            limiter,
            live_window_size: None,
            app_state: AppState::None,

            save_manager,
            settings_window,
            latest_version,
            new_version_warning,
        }
    }
}

impl eframe::App for BaseApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        match self.app_state {
            AppState::LiveWindow(_) => [0.0, 0.0, 0.0, self.settings_window.get_def("window_transparency")],
            _ => [0.0, 0.0, 0.0, 1.0],
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(50));

        let frame = Frame::none()
            .fill(Color32::TRANSPARENT);

        egui::TopBottomPanel::top("TopPanel")
            .frame(frame)
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    if let AppState::LiveWindow(_) = self.app_state {
                        if ui.button("Stop Splitter").clicked() {
                            self.app_state = AppState::None;

                            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                                egui::WindowLevel::Normal,
                            ));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
                            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 {
                                x: 1024.0,
                                y: 512.0,
                            }));
                            self.live_window_size = None;
                        }

                        return;
                    }

                    if let AppState::SettingsWindow = self.app_state {
                        if ui.button("Save Settings").clicked() {
                            self.app_state = AppState::None;

                            self.settings_window.save_settings();
                            self.save_manager.set_automatic_saving(self.settings_window.get_def("automatic_saving"));
                        }

                        return;
                    }

                    if ui.button("Live Splitter").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                            egui::WindowLevel::AlwaysOnTop,
                        ));
                        let x = self.settings_window.get_def("x_position");
                        let y = self.settings_window.get_def("y_position");
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                            egui::Pos2 { x, y }
                        ));
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                            egui::Vec2 {
                                x: self.settings_window.get_def("x_size"),
                                y: 80f32,
                            }
                        ));
                        self.live_window_size = Some(80);
                        self.app_state = AppState::LiveWindow(LiveWindow::new(
                            &self.settings_window,
                        ));
                    }

                    if ui.button("Input Speedrun Logs...").clicked() {
                        if let Some(paths) = rfd::FileDialog::new().pick_files() {
                            self.app_state = AppState::AwaitParseLogWindow(Some(AwaitParseFiles::new(paths)));
                        }
                    }

                    if ui.button("Grab stats from Logs...").clicked() {
                        if let Some(paths) = rfd::FileDialog::new().pick_files() {
                            self.app_state = AppState::AwaitParseStatWindow(Some(AwaitParseFiles::new(paths)));
                        }
                    }

                    if ui.button("Check Saved Runs").clicked() {
                        self.app_state = AppState::ManagingRuns(RunManagerWindow::new());
                    }

                    if ui.button("Settings").clicked() {
                        self.app_state = AppState::SettingsWindow;
                    }

                    if self.new_version_warning {
                        if let Some(version) = &self.latest_version {
                            if ui.button(
                                WidgetText::from("NEW VERSION DETECTED")
                                    .color(Color32::ORANGE)
                            ).clicked() {
                                let mut path: PathBuf = "https://github.com/Tgb03/Logger/releases/tag/"
                                    .into();
                                path.push(version);
                                let _ = open(path);
                            }
                        } else {
                            ui.colored_label(Color32::ORANGE, "NEW VERSION DETECTED");
                        }
                    }
                })
            });

        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| match &mut self.app_state {
                AppState::None => {}
                AppState::LogParserWindow(log_parser_window) => {
                    log_parser_window.render(ui, &mut self.save_manager)
                }
                AppState::ManagingRuns(run_manager_window) => {
                    run_manager_window.render(ui, &mut self.save_manager)
                }
                AppState::SettingsWindow => self.settings_window.render(ui),
                AppState::LiveWindow(live_window) => {
                    let size = live_window.render(ui, &self.save_manager, &self.settings_window);

                    if self.live_window_size.is_none_or(|v| v != size) {
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 {
                            x: self.settings_window.get_def("x_size"),
                            y: size as f32,
                        }));
                        self.live_window_size = Some(size);
                    }
                }
                AppState::StatsWindow(stats_window) => {
                    stats_window.render(ui);
                }
                AppState::AwaitParseLogWindow(awaiter) => {
                    if awaiter.render(ui).is_some_and(|v| v == true) {
                        let awaiter = awaiter.take().unwrap();
                        self.app_state = AppState::LogParserWindow(LogParserWindow::new(awaiter.into()));
                    }
                }
                AppState::AwaitParseStatWindow(awaiter) => {
                    if awaiter.render(ui).is_some_and(|v| v == true) {
                        let awaiter = awaiter.take().unwrap();
                        let runs: Vec<LevelRun> = awaiter.into();
                        self.app_state = AppState::StatsWindow(StatsWindow::new(runs));
                    }
                }
            });

        // if let Some(path) = self.file_dialog.update(ctx).selected() {
        //   println!("Selected file: {:?}", path);
        // }

        self.limiter.might_sleep();
    }
}
