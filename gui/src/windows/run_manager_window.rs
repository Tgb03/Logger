use core::{save_manager::SaveManager, time::Time};
use std::collections::HashMap;

use egui::{Color32, Ui};

use crate::{run::RenderRun, sorter_buttons::render_buttons};

#[derive(Default)]
pub struct RunManagerWindow {
    objective: String,
    show_split_times: bool,

    bottom_range: usize,
    merge_splits_string: String,
}

impl RunManagerWindow {
    fn sum_run_splits(run: impl Iterator<Item = Time>) -> Time {
        let mut result = Time::new();

        for time in run {
            result += time;
        }

        result
    }

    pub fn render(&mut self, ui: &mut Ui, save_manager: &mut SaveManager) {
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
                            self.merge_splits_string = save_manager.get_level_merge_split_str(&self.objective)
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
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Save run to PC").clicked() {
                save_manager.save_to_file(&self.objective);
            }

            if ui.button("Save ALL runs to PC").clicked() {
                save_manager.save_to_files();
            }

            if ui.button("Load runs for this objective").clicked() {
                save_manager.load(&self.objective);
            }

            if ui.button("Load ALL runs").clicked() {
                save_manager.load_all_runs();
            }
        });

        ui.horizontal(|ui| {
            ui.add_space(5.0);
            ui.monospace("Merge splits: ");
            if ui.add(
                egui::TextEdit::singleline(&mut self.merge_splits_string)
                    .desired_width(512.0)
                    .background_color(Color32::from_rgb(32, 32, 32))
                    .text_color(Color32::WHITE),
            ).changed() {
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
            for (id, name) in split_names.iter().skip(self.bottom_range).enumerate() {
                ui.label(format!("{: ^12}", name));

                min_size[id] = min_size[id].max(name.len());
            }
        });

        ui.horizontal(|ui| {
            ui.label("Best split for each part:           ");
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
                    );

                    if result.delete {
                        for_deletion.push(row);
                        has_deleted = true;
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
