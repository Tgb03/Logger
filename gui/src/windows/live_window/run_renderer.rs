use core::{
    run::{
        objectives::run_objective::RunObjective,
        timed_run::{LevelRun, RunEnum},
        traits::Run,
    },
    save_manager::SaveManager,
};
use std::collections::VecDeque;

use egui::{Color32, Ui};
use glr_core::{
    run_gen_result::RunGeneratorResult,
    split::{NamedSplit, Split},
    time::Time,
};
use glr_lib::dll_exports::enums::SubscribeCode;

use crate::{
    dll::parse_continously::ContinousParser,
    render::Render,
    windows::{
        live_window::objective_reader::{ObjectiveReader, UpdateObjective},
        settings_window::SettingsWindow,
    },
};

struct RunRender {
    split_labels: VecDeque<String>,
    compared_wr: Option<VecDeque<(String, Color32)>>,
    compared_best: Option<VecDeque<(String, Color32)>>,

    objective_str: String,

    total_time: Time,
    wr_compared_time: Time,
    run_buffer: Vec<NamedSplit>,
}

impl Render for RunRender {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        let len = self.split_labels.len();

        for it in 0..self.split_labels.len() {
            ui.horizontal(|ui| {
                ui.label(&self.split_labels[it]);

                if let Some(compared_wr) = &self.compared_wr {
                    match compared_wr.get(it) {
                        Some((text, color)) => ui.colored_label(*color, text),
                        None => ui.label("         "),
                    };
                }

                if let Some(compared_best) = &self.compared_best {
                    match compared_best.get(it) {
                        Some((text, color)) => ui.colored_label(*color, text),
                        None => ui.label("         "),
                    };
                }
            });
        }

        ui.horizontal(|ui| {
            ui.label("Rendering: ");
            ui.label(&self.objective_str);
        });

        ui.separator();

        (len + 1) * 22 + 6
    }
}

impl RunRender {
    pub fn new(objective_str: String, settings: &SettingsWindow) -> Self {
        Self {
            split_labels: VecDeque::new(),
            compared_wr: match settings.get_def("compare_record") {
                true => Some(VecDeque::new()),
                false => None,
            },
            compared_best: match settings.get_def("compare_best_splits") {
                true => Some(VecDeque::new()),
                false => None,
            },
            objective_str,
            total_time: Time::new(),
            wr_compared_time: Time::new(),
            run_buffer: Vec::new(),
        }
    }

    pub fn add_split<S: Split>(&mut self, split: &S, save_manager: &SaveManager) {
        let total = self.get_time(split);
        let wr_time = self.get_time_wr(split, save_manager);
        let split_time = self.get_time_split(split, save_manager);
        let split_compare = self.get_time_split_comparison(split, save_manager);

        self.split_labels.push_back(total.to_string());
        match (&mut self.compared_wr, wr_time) {
            (Some(vd), Some(c_time)) => {
                vd.push_back(match total.cmp(&c_time) {
                    std::cmp::Ordering::Less => {
                        ((c_time - total).to_string_no_hours(), Color32::GREEN)
                    }
                    std::cmp::Ordering::Equal => ("00:00.000".to_string(), Color32::WHITE),
                    std::cmp::Ordering::Greater => {
                        ((total - c_time).to_string_no_hours(), Color32::RED)
                    }
                });
            }
            (Some(vd), None) => {
                vd.push_back(("         ".to_string(), Color32::WHITE));
            }
            _ => {}
        }
        match (&mut self.compared_best, split_time, split_compare) {
            (Some(vd), Some(time), Some(c_time)) => {
                vd.push_back(match time.cmp(&c_time) {
                    std::cmp::Ordering::Less => {
                        ((c_time - time).to_string_no_hours(), Color32::GREEN)
                    }
                    std::cmp::Ordering::Equal => ("00:00.000".to_string(), Color32::WHITE),
                    std::cmp::Ordering::Greater => {
                        ((time - c_time).to_string_no_hours(), Color32::RED)
                    }
                });
            }
            (Some(vd), _, _) => {
                vd.push_back(("         ".to_string(), Color32::WHITE));
            }
            _ => {}
        }
    }

    fn get_time<S: Split>(&mut self, split: &S) -> Time {
        self.total_time += split.get_time();
        let split = NamedSplit::new(split.get_time(), split.get_name().to_owned());
        self.run_buffer.push(split);

        self.total_time
    }

    fn get_time_wr(&mut self, split: &impl Split, save_manager: &SaveManager) -> Option<Time> {
        self.wr_compared_time += save_manager
            .get_best_run(&self.objective_str)?
            .get_time_for_split(split.get_name())?;

        Some(self.wr_compared_time)
    }

    fn get_time_split(&mut self, split: &impl Split, save_manager: &SaveManager) -> Option<Time> {
        save_manager
            .get_splits_req(&self.objective_str, split.get_name())
            .unwrap_or(&vec![split.get_name().to_owned()])
            .iter()
            .map(|v| {
                self.run_buffer
                    .iter()
                    .find(|ns| ns.get_name() == v)
                    .map(|v| v.get_time())
            })
            .fold(Some(Time::new()), |a, b| match (a, b) {
                (Some(a), Some(b)) => Some(a + b),
                _ => None,
            })
    }

    fn get_time_split_comparison(
        &mut self,
        split: &impl Split,
        save_manager: &SaveManager,
    ) -> Option<Time> {
        save_manager
            .get_best_split_with_merge(&self.objective_str, split.get_name())
            .cloned()
    }
}

pub struct LevelRunRenderer {
    run_render: RunRender,
    run_buffer: Option<Vec<NamedSplit>>,

    continous_parser: ContinousParser<RunGeneratorResult>,
    no_save_for_frames: usize,
}

impl LevelRunRenderer {
    pub fn new(settings: &SettingsWindow) -> Self {
        Self {
            run_render: RunRender::new("".to_owned(), settings),
            continous_parser: ContinousParser::new(SubscribeCode::RunInfo),
            run_buffer: None,
            no_save_for_frames: 5,
        }
    }

    pub fn render(
        &mut self,
        save_manager: &mut SaveManager,
        settings: &SettingsWindow,
        reader: &impl ObjectiveReader<Objective = RunObjective>,
        ui: &mut Ui,
    ) -> usize {
        self.no_save_for_frames = self.no_save_for_frames.saturating_sub(1);
        if let Some(buffer) = self.run_buffer.take() {
            self.run_render = RunRender::new(self.run_render.objective_str.clone(), settings);

            for it in buffer {
                self.run_render.add_split(&it, save_manager);
            }
        }

        while let Some(r) = self.continous_parser.try_recv() {
            match r {
                RunGeneratorResult::GameStarted(level_descriptor, player_count) => {
                    let obj = RunObjective::from_name(level_descriptor.to_string())
                        .with_player_count(player_count);
                    self.run_render =
                        RunRender::new(reader.override_obj(obj).to_string(), settings);
                }
                RunGeneratorResult::SplitAdded(named_split) => {
                    self.run_render.add_split(&named_split, save_manager);
                }
                RunGeneratorResult::LevelRun(timed_run) => {
                    let level_run: LevelRun = timed_run.into();

                    if let Some(split) = level_run.get_split_by_name("WIN") {
                        let split = NamedSplit::new(split.get_time(), split.get_name().to_owned());
                        self.run_render.add_split(&split, save_manager);
                        self.run_render.objective_str = level_run.get_objective().to_string();
                    }

                    if self.no_save_for_frames == 0 {
                        save_manager.save(RunEnum::Level(level_run.clone()));
                    }
                }
                _ => {}
            }
        }

        self.run_render.render(ui)
    }
}

impl UpdateObjective for LevelRunRenderer {
    type Objective = RunObjective;

    fn update(&mut self, reader: &impl ObjectiveReader<Objective = Self::Objective>) {
        if let Ok(objective) = self.run_render.objective_str.as_str().try_into() {
            self.run_buffer = Some(self.run_render.run_buffer.clone());
            self.run_render.objective_str = reader.override_obj(objective).to_string();
        }
    }
}
