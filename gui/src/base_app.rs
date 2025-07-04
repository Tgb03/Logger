use core::{
    logs::{
        collectable_mapper::CollectableMapper, 
        parser::ParserResult
    }, 
    parse_files::file_parse::AwaitParseFiles, 
    save_manager::SaveManager, 
    version::{
        get_latest_version, 
        is_there_new_version
    },
};
use std::{
    collections::BTreeMap, path::PathBuf, time::Duration
};

use might_sleep::prelude::CpuLimiter;

use eframe::CreationContext;
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Frame, Vec2, WidgetText};
use opener::open;

use crate::{
    render::{BufferedRender, Render},
    windows::{
        live_window::LiveWindow, log_parser_window::LogParserWindow,
        run_manager_window::RunManagerWindow, settings_window::SettingsWindow,
        stats_window::StatsWindow,
    },
};

use crate::egui::TextStyle::{Body, Button, Heading, Monospace, Small};

enum AppState<'a> {
    None,
    AwaitParseLogWindow(Option<AwaitParseFiles<LogParserWindow>>),
    AwaitParseStatWindow(Option<AwaitParseFiles<StatsWindow>>),
    LogParserWindow(LogParserWindow),
    ManagingRuns(RunManagerWindow),
    LiveWindow(LiveWindow<'a>),
    StatsWindow(StatsWindow),
    SettingsWindow,
}

pub struct BaseApp<'a> {
    app_state: AppState<'a>,

    settings_window: SettingsWindow,
    save_manager: SaveManager,
    collectable_mapper: Option<&'a CollectableMapper>,
    limiter: CpuLimiter,

    latest_version: Option<String>,
    new_version_warning: bool,
}

impl<'a> BaseApp<'a> {
    pub fn new(cc: &CreationContext, collectable_mapper: Option<&'a CollectableMapper>) -> Self {
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
        save_manager.set_automatic_saving(settings_window.get_automatic_saving());

        if settings_window.get_automatic_loading() {
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
            app_state: AppState::None,

            save_manager,
            settings_window,
            collectable_mapper,
            latest_version,
            new_version_warning,
        }
    }
}

impl<'a> eframe::App for BaseApp<'a> {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        match self.app_state {
            AppState::LiveWindow(_) => [0.0, 0.0, 0.0, self.settings_window.get_transparency()],
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
                        }

                        return;
                    }

                    if let AppState::SettingsWindow = self.app_state {
                        if ui.button("Save Settings").clicked() {
                            self.app_state = AppState::None;

                            self.settings_window.save_settings();
                            self.save_manager.set_automatic_saving(self.settings_window.get_automatic_saving());
                        }

                        return;
                    }

                    if ui.button("Live Splitter").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                            egui::WindowLevel::AlwaysOnTop,
                        ));
                        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                            self.settings_window.get_live_rectangle().min,
                        ));
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                            self.settings_window.get_live_rectangle().size(),
                        ));
                        self.app_state = AppState::LiveWindow(LiveWindow::new(
                            self.collectable_mapper,
                            &self.settings_window,
                        ));
                    }

                    if ui.button("Input Speedrun Logs...").clicked() {
                        if let Some(paths) = rfd::FileDialog::new().pick_files() {
                            self.app_state = AppState::AwaitParseLogWindow(Some(AwaitParseFiles::new(paths, false)));
                        }
                    }

                    if ui.button("Grab stats from Logs...").clicked() {
                        if let Some(paths) = rfd::FileDialog::new().pick_files() {
                            self.app_state = AppState::AwaitParseStatWindow(Some(AwaitParseFiles::new(paths, true)));
                        }
                    }

                    if ui.button("Check Saved Runs").clicked() {
                        self.app_state = AppState::ManagingRuns(RunManagerWindow::default());
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
                AppState::SettingsWindow => self.settings_window.show(ui),
                AppState::LiveWindow(live_window) => {
                    live_window.update(&self.save_manager);
                    let (size, resize) = live_window.render(ui);

                    if resize {
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2 {
                            x: self.settings_window.get_live_rectangle().width(),
                            y: size as f32,
                        }));
                    }

                    while let Some(run) = live_window.get_vec_list().pop() {
                        self.save_manager.save(run);
                    }
                }
                AppState::StatsWindow(stats_window) => {
                    stats_window.render(ui);
                }
                AppState::AwaitParseLogWindow(awaiter) => {
                    if awaiter.render(ui).is_some_and(|v| v == true) {
                        let awaiter = awaiter.take().unwrap();
                        let r: ParserResult = awaiter.into();
                        self.app_state = AppState::LogParserWindow(LogParserWindow::new(r.into()));
                    }
                }
                AppState::AwaitParseStatWindow(awaiter) => {
                    if awaiter.render(ui).is_some_and(|v| v == true) {
                        let awaiter = awaiter.take().unwrap();
                        let r: ParserResult = awaiter.into();
                        self.app_state = AppState::StatsWindow(StatsWindow::new(r.into()));
                    }
                }
            });

        // if let Some(path) = self.file_dialog.update(ctx).selected() {
        //   println!("Selected file: {:?}", path);
        // }

        self.limiter.might_sleep();
    }
}
