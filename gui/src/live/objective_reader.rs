use core::{logs::{live_parser::LiveParser, parser::{Parser, ParserResult}, token_parser::TokenParserT}, run::{objectives::{run_objective::RunObjective, Objective}, traits::Run}};

use crate::render::Render;

pub trait ObjectiveReader {
    type Objective: Objective;

    fn override_obj(&self, objective: Self::Objective) -> Self::Objective;
}

pub trait ObjectiveUpdate {
    type Reader: ObjectiveReader;

    fn override_obj(&mut self, reader: &Self::Reader);
}

#[derive(Default)]
pub struct LevelObjectiveReader {
    objective: RunObjective,
}

impl ObjectiveReader for LevelObjectiveReader {
    type Objective = RunObjective;

    fn override_obj(&self, mut objective: Self::Objective) -> Self::Objective {
        objective.secondary = self.objective.secondary;
        objective.overload = self.objective.overload;
        objective.glitched = self.objective.glitched;
        objective.early_drop = self.objective.early_drop;

        objective
    }
}

impl ObjectiveUpdate for ParserResult {
    type Reader = LevelObjectiveReader;

    fn override_obj(&mut self, reader: &Self::Reader) {
        if let Ok(objective) = TryInto::<RunObjective>::try_into(self.get_objective_str().as_str()) {
            self.set_objective_str((&reader.override_obj(objective)).into());
        }
    }
}

impl ObjectiveUpdate for Parser {
    type Reader = LevelObjectiveReader;

    fn override_obj(&mut self, reader: &Self::Reader) {
        self.into_result_mut()
            .override_obj(reader);
        self.get_run_parser_mut()
            .map(|rp| rp.into_result_mut())
            .map(|run| {
                if let Some(objective) = run.get_objective::<RunObjective>() {
                    run.set_objective(
                        &reader.override_obj(objective)
                    )
                }
            });
    }
}

impl ObjectiveUpdate for LiveParser {
    type Reader = LevelObjectiveReader;

    fn override_obj(&mut self, reader: &Self::Reader) {
        self.get_parser_mut()
            .override_obj(reader);    
    }
}

impl Render for LevelObjectiveReader {
    type Response = (usize, bool);

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        let mut modified = false;

        ui.horizontal(|ui| {
            modified = modified.max(ui.checkbox(&mut self.objective.secondary, "Sec   ").changed());
            modified = modified.max(ui.checkbox(&mut self.objective.overload, "Ovrl").changed());
        });

        ui.horizontal(|ui| {
            modified = modified.max(ui.checkbox(&mut self.objective.glitched, "Glitch").changed());
            modified = modified.max(ui.checkbox(&mut self.objective.early_drop, "E-Drop").changed());
        });

        ui.separator();

        (50, modified)
    }
}
