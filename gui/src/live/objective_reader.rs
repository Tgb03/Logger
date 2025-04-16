use core::run::objectives::{Objective, run_objective::RunObjective};

use crate::render::Render;

pub trait ObjectiveReader {
    type Objective: Objective;

    fn override_obj(&self, objective: Self::Objective) -> Self::Objective;
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

impl Render for LevelObjectiveReader {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.objective.secondary, "Sec   ");
            ui.checkbox(&mut self.objective.overload, "Ovrl");
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.objective.glitched, "Glitch");
            ui.checkbox(&mut self.objective.early_drop, "E-Drop");
        });

        ui.separator();

        50
    }
}
