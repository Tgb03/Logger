use core::{
    logs::collectable_mapper::CollectableMapper, parse_files::file_parse::parse_all_files_async,
    run::timed_run::LevelRun, save_manager::SaveManager,
};
use std::{
    collections::{BTreeMap, HashSet}, time::Duration
};

use eframe::CreationContext;
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Frame, Vec2};

use crate::{
    render::{BufferedRender, Render},
    windows::{
        live_window::LiveWindow, log_parser_window::LogParserWindow,
        run_manager_window::RunManagerWindow, settings_window::SettingsWindow,
    },
};

use crate::egui::TextStyle::{Body, Button, Heading, Monospace, Small};

enum AppState<'a> {
    None,
    LogParserWindow(LogParserWindow),
    ManagingRuns(RunManagerWindow),
    LiveWindow(LiveWindow<'a>),
    SettingsWindow,
}

pub struct BaseApp<'a> {
    app_state: AppState<'a>,

    settings_window: SettingsWindow,
    save_manager: SaveManager,
    collectable_mapper: Option<&'a CollectableMapper>,
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

        if settings_window.get_automatic_loading() {
            save_manager.load_all_runs();
        }

        Self {
            app_state: AppState::None,

            save_manager,
            settings_window,
            collectable_mapper,
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

        let frame = Frame::none().fill(Color32::TRANSPARENT);

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

                            // let parse_result = parse_all_files(&files);
                            let parse_result = parse_all_files_async(paths);
                            let hash: HashSet<LevelRun> =
                                HashSet::from_iter(Into::<Vec<LevelRun>>::into(parse_result));
                            let runs = hash.into_iter().collect();
                            self.app_state = AppState::LogParserWindow(LogParserWindow::new(runs));
                        }
                    }

                    if ui.button("Check Saved Runs").clicked() {
                        self.app_state = AppState::ManagingRuns(RunManagerWindow::default());
                    }

                    if ui.button("Settings").clicked() {
                        self.app_state = AppState::SettingsWindow;
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
            });

        // if let Some(path) = self.file_dialog.update(ctx).selected() {
        //   println!("Selected file: {:?}", path);
        // }
    }
}
