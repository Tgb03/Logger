use core::{logs::{live_parser::LiveParser, token_parser::TokenParserT}, seed_gen::{consumers::{base_consumer::Consumer, OutputSeedIndexer}, load_seed_consumers::LoadSeedConsumers, unity_random::UnityRandom}};


use crate::render::{BufferedRender, Render};

#[derive(Default)]
pub struct SeedIndexer {

    output: Vec<OutputSeedIndexer>,
    last_seed: i32,

}

impl Render for OutputSeedIndexer {
    type Response = ();

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        ui.label(format!("{:?}", self));
    }
}

impl Render for Vec<OutputSeedIndexer> {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        for data in self.iter_mut() {
            data.render(ui);
        }

        self.len() * 22
    }
}

impl BufferedRender for SeedIndexer {
    type Response = usize;
    type UpdateData = LiveParser;
    type Render = Vec<OutputSeedIndexer>;

    fn update(&mut self, update_data: &Self::UpdateData) {
        
        if self.last_seed != update_data.into_result().get_last_seed() {
            self.last_seed = update_data.into_result().get_last_seed();

            let mut ur = UnityRandom::from(self.last_seed);
            let json = LoadSeedConsumers::load_all();
            let level_name = &update_data.get_parser().get_base_objective().level_name;

            if let Some(mut p) = json
                .as_ref()
                .map(|h| h.get(level_name))
                .flatten()
                .cloned() {
                
                self.output.clear();
                p.take_multiple(&mut ur, &mut self.output);
            }
        }

    }

    fn reset(&mut self) {
        self.output.clear();
    }

    fn get_renderer(&mut self) -> &mut Self::Render {
        &mut self.output
    }
}
