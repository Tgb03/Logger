use core::logs::parser::ParserResult;

use crate::render::{BufferedRender, Render};

#[derive(Default)]
pub struct RunCounter {
    run_counter: usize,
    seed_counter: usize,
}

#[derive(Default)]
pub struct RunCounterBuffer {
    run_counter: RunCounter,
}

impl Render for RunCounter {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        ui.horizontal(|ui| {
            ui.label(format!(
                "Run counter: {}  Unique Seeds: {}",
                self.run_counter, self.seed_counter
            ));
        });

        ui.separator();

        28
    }
}

impl BufferedRender for RunCounterBuffer {
    type Response = usize;
    type UpdateData = ParserResult;
    type Render = RunCounter;

    fn update(&mut self, update_data: &Self::UpdateData) {
        self.run_counter.run_counter = update_data.get_counter() as usize;
        self.run_counter.seed_counter = update_data.get_set().len();
    }

    fn reset(&mut self) {
        self.run_counter.run_counter = 0;
        self.run_counter.seed_counter = 0;
    }

    fn get_renderer(&mut self) -> &mut Self::Render {
        &mut self.run_counter
    }
}
