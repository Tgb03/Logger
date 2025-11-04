use core::run::traits::Run;

use egui::Ui;
use glr_core::{split::Split, time::Time};

fn get_total_times<T: Split>(timed_runs: &Vec<T>) -> Time {
    let mut total: Time = Time::new();

    for timed_run in timed_runs {
        total += timed_run.get_time();
    }

    total
}

pub fn render_buttons<T: Run>(timed_runs: &mut Vec<T>, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label(format!(
            "Total times added: {}",
            get_total_times(timed_runs).to_string()
        ));

        if ui.button("Sort by Win").clicked() {
            timed_runs.sort_by(|d, e| d.is_win().cmp(&e.is_win()).reverse());
        }

        if ui.button("Sort by objective").clicked() {
            timed_runs.sort_by(|d, e| d.get_objective().cmp(&e.get_objective()));
        }

        if ui.button("Sort by time").clicked() {
            timed_runs.sort_by(|d, e| d.get_time().get_stamp().cmp(&e.get_time().get_stamp()));
        }

        // if ui.button(super::create_text("Sort by Players")).clicked() {
        //   timed_runs.sort_by(|d, e| d.get_objective().get_player_count().cmp(&e.get_objective().get_player_count()));
        // }

        if ui.button("Sort by Stamps").clicked() {
            timed_runs.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
        }
    });
}

pub trait OptionalVisualSorterButtons<T: Run> {
    fn get_vec(&mut self) -> Option<&mut Vec<T>>;

    fn render_buttons(&mut self, ui: &mut Ui) {
        let timed_runs = self.get_vec();

        if let Some(timed_runs) = timed_runs {
            render_buttons(timed_runs, ui);
        }
    }
}

pub trait VisualSorterButtons<T: Run> {
    fn get_vec(&mut self) -> &mut Vec<T>;
}

impl<T, R> OptionalVisualSorterButtons<R> for T
where
    T: VisualSorterButtons<R>,
    R: Run,
{
    fn get_vec(&mut self) -> Option<&mut Vec<R>> {
        Some(self.get_vec())
    }
}
