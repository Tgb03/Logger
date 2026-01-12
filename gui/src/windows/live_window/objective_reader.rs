use core::run::objectives::{Objective, run_objective::RunObjective};
use std::ops::DerefMut;

use crate::render::Render;

pub trait ObjectiveReader {
    type Objective: Objective;

    fn override_obj(&self, objective: Self::Objective) -> Self::Objective;
}

impl<T> ObjectiveReader for Option<T>
where
    T: ObjectiveReader,
{
    type Objective = T::Objective;

    fn override_obj(&self, objective: Self::Objective) -> Self::Objective {
        match self {
            Some(s) => s.override_obj(objective),
            None => objective,
        }
    }
}

pub trait UpdateObjective {
    type Objective: Objective;

    fn update(&mut self, reader: &impl ObjectiveReader<Objective = Self::Objective>);
}

impl<T: UpdateObjective> UpdateObjective for Option<T> {
    type Objective = T::Objective;

    fn update(&mut self, reader: &impl ObjectiveReader<Objective = Self::Objective>) {
        match self {
            Some(s) => s.update(reader),
            None => {}
        }
    }
}

impl<T: UpdateObjective> UpdateObjective for Box<T> {
    type Objective = T::Objective;

    fn update(&mut self, reader: &impl ObjectiveReader<Objective = Self::Objective>) {
        self.deref_mut().update(reader);
    }
}

#[derive(Default, Clone)]
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
    type Response = (usize, bool);

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        let mut changed = false;

        ui.horizontal(|ui| {
            changed = changed.max(
                ui.checkbox(&mut self.objective.secondary, "Sec   ")
                    .changed(),
            );
            changed = changed.max(ui.checkbox(&mut self.objective.overload, "Ovrl").changed());
        });

        ui.horizontal(|ui| {
            changed = changed.max(
                ui.checkbox(&mut self.objective.glitched, "Glitch")
                    .changed(),
            );
            changed = changed.max(
                ui.checkbox(&mut self.objective.early_drop, "E-Drop")
                    .changed(),
            );
        });

        ui.separator();

        (50, changed)
    }
}
