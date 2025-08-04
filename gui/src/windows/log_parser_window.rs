use core::{export::Export, run::{objectives::{objective_enum::ObjectiveEnum, Objective}, timed_run::{LevelRun, RunEnum}, traits::Run}, save_manager::SaveManager};
use std::fs::File;

use egui::Color32;

use glr_core::split::Split;
use crate::{run::RenderResult, sorter_buttons::{OptionalVisualSorterButtons, VisualSorterButtons}};


pub struct LogParserWindow {
    
    timed_runs: Vec<LevelRun>,

    set_all_secondary: bool,
    set_all_overload: bool,
    set_all_glitched: bool,
    set_all_early_drop: bool,

    // game_obj: GameObjective,
    // player_count_input: String,

}

impl LogParserWindow {

    pub fn new(runs: Vec<LevelRun>) -> Self {
        Self {
            timed_runs: runs,
            set_all_secondary: false,
            set_all_overload: false,
            set_all_glitched: false,
            set_all_early_drop: false,
            // game_obj: GameObjective::default(),
            // player_count_input: "0".to_string(),
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, save_manager: &mut SaveManager) {
        self.render_buttons(ui);

        // handles all the set all buttons.
        ui.horizontal(|ui| {
            let secondary_checkbox = ui.checkbox(&mut self.set_all_secondary, "Set ALL secondary");
            let overload_checkbox = ui.checkbox(&mut self.set_all_overload, "Set ALL overload");
            let glitched_checkbox = ui.checkbox(&mut self.set_all_glitched, "Set ALL glitched");
            let early_drop_checkbox =
                ui.checkbox(&mut self.set_all_early_drop, "Set ALL early drop");

            if secondary_checkbox.clicked() {
                for timed_run in &mut self.timed_runs {
                    timed_run.set_objective({
                        let ObjectiveEnum::Run(obj) = timed_run
                            .get_objective()
                            .clone() else { unreachable!() };
                        ObjectiveEnum::Run(obj.with_secondary(self.set_all_secondary))
                    });
                }
            }

            if overload_checkbox.clicked() {
                for timed_run in &mut self.timed_runs {
                    timed_run.set_objective({
                        let ObjectiveEnum::Run(obj) = timed_run
                            .get_objective()
                            .clone() else { unreachable!() };
                        ObjectiveEnum::Run(obj.with_overload(self.set_all_overload))
                    });
                }
            }

            if glitched_checkbox.clicked() {
                for timed_run in &mut self.timed_runs {
                    timed_run.set_objective({
                        let ObjectiveEnum::Run(obj) = timed_run
                            .get_objective()
                            .clone() else { unreachable!() };
                        ObjectiveEnum::Run(obj.with_glitched(self.set_all_glitched))
                    });
                }
            }

            if early_drop_checkbox.clicked() {
                for timed_run in &mut self.timed_runs {
                    timed_run.set_objective({
                        let ObjectiveEnum::Run(obj) = timed_run
                            .get_objective()
                            .clone() else { unreachable!() };
                        ObjectiveEnum::Run(obj.with_early_drop(self.set_all_early_drop))
                    });
                }
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Save ALL runs").clicked() {
                save_manager.save_multiple(
                    self.timed_runs
                        .iter()
                        .map(|f| RunEnum::Level(f.clone()))
                        .collect(),
                );
                self.timed_runs.clear();
            }
            /* 
            if ui.button("Save ALL as FULL GAME RUN").clicked() {
                let mut game_run = GameRun::new(self.game_obj.clone());

                self.timed_runs
                    .sort_by(|a, b| a.get_objective_str().cmp(b.get_objective_str()));
                for run in self.timed_runs.drain(0..self.timed_runs.len()) {
                    game_run.add_split(run);
                }

                game_run.validate();

                self.save_manager.save(RunEnum::Game(game_run));
            }
            */
            if ui.button("Export to CSV").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_title("Export to file")
                    .set_file_name("new_file.csv")
                    .save_file()
                {
                    match File::create(path) {
                        Ok(file) => {
                            Export::export_times(self.timed_runs.iter(), file);
                        }
                        Err(_) => {}
                    }
                }
            }
        });

        egui::ScrollArea::vertical().show_rows(
            ui,
            ui.text_style_height(&egui::TextStyle::Body),
            self.timed_runs.len(),
            |ui, row_range| {
                let mut for_removal = Vec::new();
                let mut for_saving = Vec::new();

                for row in row_range {
                    let timed_run = &mut self.timed_runs[row];
                    let mut result = RenderResult::default();

                    let time = timed_run.get_time();
                    let color = match timed_run.is_win() {
                        true => Color32::GREEN,
                        false => Color32::RED,
                    };
                    let mut objective = timed_run
                        .get_objective()
                        .as_level_run()
                        .unwrap()
                        .clone();

                    ui.horizontal(|ui| {
                        ui.label(&objective.level_name);
                        ui.colored_label(
                            Color32::WHITE,
                            format!("{}p", objective.get_player_count().to_string()),
                        );

                        ui.colored_label(color, time.to_string());

                        ui.colored_label(Color32::WHITE, format!("{:03} stamps", timed_run.len()));

                        ui.checkbox(&mut objective.secondary, "Secondary");
                        ui.checkbox(&mut objective.overload, "Overload");
                        ui.checkbox(&mut objective.glitched, "Glitched");
                        ui.checkbox(&mut objective.early_drop, "Early Drop");

                        timed_run.set_objective(ObjectiveEnum::Run(objective.clone()));

                        if ui.button("SAVE RUN").clicked() {
                            result.save = true;
                        }

                        if ui.button("DELETE").clicked() {
                            result.delete = true;
                        }
                    });

                    if result.delete {
                        for_removal.push(row);
                    }
                    if result.save {
                        for_saving.push(row);
                    }
                }

                for id in for_removal.iter().rev() {
                    self.timed_runs.remove(*id);
                }

                for id in for_saving.iter().rev() {
                    let run = self.timed_runs.remove(*id);
                    save_manager.save(RunEnum::Level(run));
                }
            },
        );
    }


}

impl VisualSorterButtons<LevelRun> for LogParserWindow {
    fn get_vec(&mut self) -> &mut Vec<LevelRun> {
        &mut self.timed_runs
    }
}

