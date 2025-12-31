use core::save_manager::{SaveManager, SaveType};
use std::collections::HashMap;

use egui::Color32;
use glr_core::time::Time;

use crate::{run::RenderRun, sorter_buttons::render_buttons, windows::settings_window::SettingsWindow};

pub struct RunManagerWindow {
    objective: String,
    show_split_times: bool,
    compare_all: bool,

    bottom_range: usize,
    merge_splits_string: String,

    compare_first: Option<usize>,
    compare_second: Vec<bool>,
    save_type: SaveType,
}

impl RunManagerWindow {
    fn sum_run_splits(run: impl Iterator<Item = Time>) -> Time {
        let mut result = Time::new();

        for time in run {
            result += time;
        }

        result
    }

    pub fn new(settings: &SettingsWindow) -> Self {
        Self {
            objective: "".to_owned(),
            show_split_times: false,
            bottom_range: 0,
            merge_splits_string: "".to_owned(),
            compare_first: None,
            compare_second: Vec::new(),
            compare_all: false,
            save_type: settings.get_save_type(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, save_manager: &mut SaveManager) {
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Select loaded objective")
                .selected_text(self.objective.to_string())
                .height(500.0)
                .show_ui(ui, |ui| {
                    for key in save_manager.get_all_objectives() {
                        if ui
                            .selectable_value(&mut self.objective, key.clone(), key)
                            .clicked()
                        {
                            self.bottom_range = 0;
                            self.compare_first = None;
                            self.compare_second = vec![
                                false; 
                                save_manager.get_runs(&self.objective)
                                    .map(|v| v.len())
                                    .unwrap_or_default()
                            ];
                            self.merge_splits_string = save_manager
                                .get_level_merge_split_str(&self.objective)
                                .unwrap_or_default();
                        };
                    }
                });

            if ui.button("Remove useless runs").clicked() {
                save_manager.optimize_obj(&self.objective);
            }

            if let Some(best_splits) = save_manager.get_best_splits(&self.objective) {
                ui.colored_label(Color32::GOLD, "Theoretical:");
                ui.colored_label(
                    Color32::GOLD,
                    Self::sum_run_splits(
                        save_manager
                            .get_split_names(&self.objective)
                            .unwrap_or(&Vec::new())
                            .iter()
                            .map(|name| best_splits.get(name).cloned().unwrap_or_default()),
                    )
                    .to_string(),
                );
            }

            ui.checkbox(&mut self.show_split_times, "Show Split Times");

            if ui.checkbox(&mut self.compare_all, "Compare ALL").clicked() {
                if self.compare_all {
                    for it in &mut self.compare_second {
                        *it = true;
                    }
                    self.compare_second.get_mut(0).map(|v| *v = false);
                    self.compare_first = match save_manager.get_runs(&self.objective)
                        .map(|v| v.len())
                        .unwrap_or_default() > 0 {
                        true => Some(0),
                        false => None,
                    };
                } else {
                    self.compare_first = None;
                    for it in &mut self.compare_second {
                        *it = false;
                    }
                }
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Save run to PC").clicked() {
                save_manager.save_to_file(self.save_type, &self.objective);
            }

            if ui.button("Save ALL runs to PC").clicked() {
                save_manager.save_to_files();
            }

            if ui.button("Load runs for this objective").clicked() {
                save_manager.load_advanced(&self.objective);
                self.compare_second = vec![
                    false; 
                    save_manager.get_runs(&self.objective)
                        .map(|v| v.len())
                        .unwrap_or_default()
                ];
            }

            if ui.button("Load ALL runs").clicked() {
                save_manager.load_all_runs();
                self.compare_second = vec![
                    false; 
                    save_manager.get_runs(&self.objective)
                        .map(|v| v.len())
                        .unwrap_or_default()
                ];
            }
        });

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.monospace("Merge splits: ");
            if ui
                .add(
                    egui::TextEdit::singleline(&mut self.merge_splits_string)
                        .desired_width(512.0)
                        .background_color(Color32::from_rgb(32, 32, 32))
                        .text_color(Color32::WHITE),
                )
                .changed()
            {
                save_manager.set_merge_splits(&self.objective, &self.merge_splits_string);
            };
        });

        ui.separator();

        if let Some(runs) = save_manager.get_runs_mut(&self.objective) {
            // handles all sorters
            render_buttons(runs, ui);
        }

        let binding = Vec::new();
        let split_names = save_manager
            .get_split_names(&self.objective)
            .unwrap_or(&binding);
        let mut min_size = vec![12; split_names.len().saturating_sub(self.bottom_range) + 1];
        let binding = HashMap::default();
        let best_splits = save_manager
            .get_best_splits(&self.objective)
            .unwrap_or(&binding);

        ui.horizontal(|ui| {
            ui.label("Name of splits:           ");
            if ui.button(" < ").clicked() {
                self.bottom_range = self.bottom_range.saturating_sub(1);
            }
            if ui.button(" > ").clicked() {
                self.bottom_range = (self.bottom_range + 1).min(split_names.len() - 1);
            }
            if !self.show_split_times {
                ui.label("  CMP ");
            }

            for (id, name) in split_names.iter().skip(self.bottom_range).enumerate() {
                ui.label(format!("{: ^12}", name));

                min_size[id] = min_size[id].max(name.len());
            }
        });

        ui.horizontal(|ui| {
            ui.label("Best split for each part:           ");
            if !self.show_split_times {
                ui.label("SPLITS");
            }
            for (id, name) in split_names.iter().skip(self.bottom_range).enumerate() {
                ui.label(format!(
                    "{: ^fill$}",
                    best_splits
                        .get(name)
                        .unwrap_or(&Time::default())
                        .to_string(),
                    fill = min_size[id]
                ));
            }
        });

        let timed_runs = match save_manager.get_runs(&self.objective) {
            Some(run) => run,
            None => return,
        };

        let mut has_deleted = false;
        let mut for_deletion = Vec::new();

        egui::ScrollArea::vertical().show_rows(
            ui,
            ui.text_style_height(&egui::TextStyle::Body),
            timed_runs.len(),
            |ui, row_range| {
                for row in row_range {
                    let timed_run = &timed_runs[row];

                    let result = timed_run.show(
                        &min_size,
                        self.bottom_range..split_names.len(),
                        &save_manager,
                        ui,
                        self.show_split_times,
                        match self.compare_second.get(row).cloned().unwrap_or_default() {
                            true => self.compare_first
                                .map(|v| timed_runs.get(v))
                                .flatten(),
                            false => None
                        },
                        self.compare_first.is_some_and(|v| v == row),
                        self.compare_second.get(row).cloned().unwrap_or_default(),
                    );

                    if result.delete {
                        for_deletion.push(row);
                        has_deleted = true;
                    }

                    if let Some(val) = result.compare_first {
                        let copy = self.compare_first;
                        self.compare_first = match val {
                            true => Some(row),
                            false => None,
                        };
                        if self.compare_second[row] {
                            copy.map(|v| { self.compare_second[v] = true; });
                        }
                        self.compare_second[row] = false;
                    }

                    if let Some(val) = result.compare_second {
                        self.compare_second[row] = val;
                    }
                }
            },
        );

        let timed_runs = match save_manager.get_runs_mut(&self.objective) {
            Some(run) => run,
            None => return,
        };
        for it in for_deletion.iter().rev() {
            timed_runs.remove(*it);
        }

        if has_deleted {
            save_manager.calculate_best_splits(&self.objective.to_string());
        }
    }
}
