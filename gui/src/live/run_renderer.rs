#[allow(unused_imports)]
use core::run::traits::Timed;
use core::{
    run::{objectives::Objective, traits::Run},
    save_manager::SaveManager,
    time::Time,
};
use std::{collections::VecDeque, marker::PhantomData};

use egui::Color32;

use crate::{render::Render, windows::settings_window::SettingsWindow};

use super::objective_reader::ObjectiveReader;

pub struct RunRender {
    split_labels: VecDeque<String>,
    compared_wr: Option<VecDeque<(String, Color32)>>,
    compared_best: Option<VecDeque<(String, Color32)>>,

    objective_str: String,
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

        ui.label(&self.objective_str);

        ui.separator();

        (len + 1) * 22 + 6
    }
}

pub struct RunRendererBuffer<R, O>
where
    R: Run,
    O: Objective,
{
    render_obj: RunRender,
    current_run_total: Time,
    wr_total: Time,

    max_length: usize,
    current_run_counter: Option<u64>,

    splits_added_count: usize,

    phantom_data1: PhantomData<O>,
    phantom_data2: PhantomData<R>,
}

impl<R, O> Render for RunRendererBuffer<R, O>
where
    R: Run,
    O: Objective,
{
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        self.get_renderer().render(ui)
    }
}

impl<R, O> RunRendererBuffer<R, O>
where
    R: Run,
    O: Objective,
{
    pub fn new(objective: String, settings: &SettingsWindow) -> Self {
        Self {
            render_obj: RunRender {
                split_labels: VecDeque::new(),
                compared_wr: match settings.get_compare_to_record() {
                    true => Some(VecDeque::new()),
                    false => None,
                },
                compared_best: match settings.get_compare_to_theoretical() {
                    true => Some(VecDeque::new()),
                    false => None,
                },
                objective_str: format!("Rendering: {objective}"),
            },
            current_run_total: Time::new(),
            wr_total: Time::new(),
            current_run_counter: None,
            max_length: settings.get_splitter_length(),
            splits_added_count: 0,
            phantom_data1: PhantomData,
            phantom_data2: PhantomData,
        }
    }

    pub fn update<OR>(
        &mut self,
        run_counter: u64,
        current_run: &R,
        save_manager: &SaveManager,
        objective_reader: &Option<OR>,
    ) where
        OR: ObjectiveReader<Objective = O>,
    {
        if self.current_run_counter.is_none_or(|rc| rc != run_counter) {
            self.reset();
            self.current_run_counter = Some(run_counter);
        }

        let objective = current_run.get_objective::<OR::Objective>().unwrap();

        let objective = match objective_reader.as_ref() {
            Some(or) => or.override_obj(objective).to_string(),
            None => objective.to_string(),
        };

        self.render_obj.objective_str = format!("Rendering: {objective}");

        let wr_run = match self.render_obj.compared_wr {
            Some(_) => save_manager.get_best_run(&objective),
            None => None,
        };

        for split in current_run.get_splits().skip(self.splits_added_count) {
            let time = split.get_time();
            let name = split.get_name();

            if name == "LOSS" {
                self.splits_added_count += 1;
                continue;
            }

            self.current_run_total += time;
            self.render_obj
                .split_labels
                .push_back(self.current_run_total.to_string());

            match (wr_run, &mut self.render_obj.compared_wr) {
                (Some(run), Some(wr_buffer)) => {
                    self.wr_total += run.get_time_for_split(name).unwrap_or_default();
                    wr_buffer.push_back(match self.current_run_total.cmp(&self.wr_total) {
                        std::cmp::Ordering::Less => (
                            (self.wr_total - self.current_run_total).to_string_no_hours(),
                            Color32::GREEN,
                        ),
                        std::cmp::Ordering::Equal => ("00:00.000".to_string(), Color32::WHITE),
                        std::cmp::Ordering::Greater => (
                            (self.current_run_total - self.wr_total).to_string_no_hours(),
                            Color32::RED,
                        ),
                    });
                }
                (None, Some(wr_buffer)) => {
                    wr_buffer.push_back(("         ".to_string(), Color32::WHITE));
                }
                _ => {}
            }

            let best_split = save_manager.get_best_split(
                &objective, 
                save_manager.get_split_merge(&objective, name)
                    .unwrap_or(name)
            ).cloned();
            let req_splits = save_manager.get_split_merge(&objective, name)
                .map(|v| save_manager.get_splits_req(&objective, v))
                .flatten();
            let split_time_total = req_splits
                .map(|v| {
                    v
                        .iter()
                        .map(|s| current_run.get_time_for_split(s))
                        .fold(Some(Time::default()), |a, b| {
                            match (a, b) {
                                (Some(t1), Some(t2)) => Some(t1 + t2),
                                _ => None
                            }
                        })
                })
                .unwrap_or(Some(time));
            println!("OBJ: {}", objective);
            println!("BRUK: {:?} vs {:?} and {:?}", split_time_total, best_split, req_splits);
            match (best_split, &mut self.render_obj.compared_best) {
                (Some(best_split), Some(buffer)) => {
                    if let Some(time) = split_time_total {
                        buffer.push_back(match time.cmp(&best_split) {
                            std::cmp::Ordering::Less => {
                                ((best_split - time).to_string_no_hours(), Color32::GREEN)
                            }
                            std::cmp::Ordering::Equal => ("00:00.000".to_string(), Color32::WHITE),
                            std::cmp::Ordering::Greater => {
                                ((time - best_split).to_string_no_hours(), Color32::RED)
                            }
                        });
                    } else {
                        buffer.push_back(("         ".to_string(), Color32::WHITE));
                    }
                }
                (None, Some(buffer)) => {
                    buffer.push_back(("         ".to_string(), Color32::WHITE));
                }
                _ => {}
            }

            self.splits_added_count += 1;
        }

        while self.render_obj.split_labels.len() > self.max_length {
            self.render_obj.split_labels.pop_front();
            self.render_obj
                .compared_wr
                .as_mut()
                .map(|cw| cw.pop_front());
            self.render_obj
                .compared_best
                .as_mut()
                .map(|cw| cw.pop_front());
        }
    }

    pub fn reset(&mut self) {
        self.render_obj.objective_str = "".to_string();
        self.render_obj.split_labels = VecDeque::new();
        self.render_obj.compared_wr.as_mut().map(|v| v.clear());
        self.render_obj.compared_best.as_mut().map(|v| v.clear());

        self.current_run_total = Time::new();
        self.wr_total = Time::new();
        self.splits_added_count = 0;
    }

    pub fn get_renderer(&mut self) -> &mut impl Render<Response = usize> {
        &mut self.render_obj
    }
}
