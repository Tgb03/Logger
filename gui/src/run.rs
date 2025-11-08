use core::{
    run::{objectives::Objective, timed_run::RunEnum, traits::Run},
    save_manager::SaveManager,
};
use std::ops::Range;

use egui::{Color32, Ui};
use glr_core::time::Time;

#[derive(Default)]
pub struct RenderResult {
    pub delete: bool,
    pub save: bool,
    pub compare_first: Option<bool>,
    pub compare_second: Option<bool>,
}

pub trait RenderRun {
    fn show(
        &self,
        min_sizes: &Vec<usize>,
        range: Range<usize>,
        save_manager: &SaveManager,
        ui: &mut Ui,
        show_split_times: bool,
        compare_run: Option<&RunEnum>,
        compare_first: bool,
        compare_second: bool,
    ) -> RenderResult;
}

impl<T> RenderRun for T
where
    T: Run,
{
    fn show(
        &self,
        min_sizes: &Vec<usize>,
        range: Range<usize>,
        save_manager: &SaveManager,
        ui: &mut egui::Ui,
        show_split_times: bool,
        compare_run: Option<&RunEnum>,
        mut compare_first: bool,
        mut compare_second: bool,
    ) -> RenderResult {
        let mut result = RenderResult::default();
        let empty_vec = Vec::new();
        let objective = self.get_objective();
        let objective_str = objective.to_string();
        let split_names = save_manager
            .get_split_names(&objective_str)
            .unwrap_or(&empty_vec);

        ui.horizontal(|ui| {
            let time = self.get_time();

            let color = match self.is_win() {
                true => Color32::GREEN,
                false => Color32::RED,
            };

            ui.label("RUN:");
            ui.colored_label(color, time.to_string());

            ui.label(objective.get_player_count().to_string());

            if ui.button(format!("DELETE RUN")).clicked() {
                result.delete = true;
            }
            ui.label(format!("{:03}", self.len()));

            if !show_split_times {
                if ui.checkbox(&mut compare_first, "")
                    .clicked() {
                    
                    result.compare_first = Some(compare_first);
                }

                if ui.checkbox(&mut compare_second, "")
                    .clicked() {
                    
                    result.compare_second = Some(compare_second);
                }
            }

            let mut running_total = Time::default();
            let mut cmp_total = Time::new();
            for id in 0..range.start.min(split_names.len()) {
                running_total += grab_time(self, &objective_str, &split_names[id], save_manager)
                    .unwrap_or_default();
                if let Some(cmp_run) = compare_run {
                    cmp_total += grab_time(cmp_run, &objective_str, &split_names[id], save_manager)
                        .unwrap_or_default();
                }
            }

            let first = range.start.min(split_names.len());

            for id in range {
                if let Some(time) = grab_time(self, &objective_str, &split_names[id], save_manager)
                {
                    if show_split_times {
                        let color = match save_manager
                            .get_best_split(&objective_str, &split_names[id])
                            .is_some_and(|v| *v == time)
                        {
                            true => Color32::GREEN,
                            false => Color32::GRAY,
                        };

                        ui.colored_label(
                            color,
                            format!("{: ^fill$}", time.to_string(), fill = min_sizes[id - first]),
                        );

                        continue
                    } 

                    if let Some(cmp_run) = compare_run {
                        running_total += time;
                        cmp_total += grab_time(cmp_run, &objective_str, &split_names[id], save_manager)
                            .unwrap_or_default();

                        let (color, sign, t_show) = match running_total.cmp(&cmp_total) {
                            std::cmp::Ordering::Less => (Color32::GREEN, '-', cmp_total - running_total),
                            std::cmp::Ordering::Equal => (Color32::GRAY, ' ', Time::new()),
                            std::cmp::Ordering::Greater => (Color32::RED, '+', running_total - cmp_total),
                        };

                        ui.colored_label(
                            color, 
                            format!(
                                "{:^fill$}",
                                format!("{}{}", sign, t_show.to_string_no_hours()),
                                fill = min_sizes[id - first]
                            )
                        );

                        continue
                    }    
                    
                    running_total += time;
                    ui.colored_label(
                        Color32::GRAY,
                        format!(
                            "{: ^fill$}",
                            running_total.to_string(),
                            fill = min_sizes[id - first]
                        ),
                    );
                    
                } else {
                    ui.label("            ");
                }
            }
        });

        result
    }
}

fn grab_time<R: Run>(
    run: &R,
    objective_str: &String,
    split_name: &String,
    save_manager: &SaveManager,
) -> Option<Time> {
    let splits_vec = save_manager.get_splits_req(objective_str, split_name);
    match splits_vec {
        Some(splits_vec) => {
            let mut result = Time::default();
            for it in splits_vec.iter().map(|v| run.get_time_for_split(v)) {
                if let Some(it) = it {
                    result += it;
                } else {
                    return None;
                }
            }
            return Some(result);
        }
        None => {
            if let Some(time) = run.get_time_for_split(split_name) {
                return Some(time);
            }
        }
    };
    None
}
