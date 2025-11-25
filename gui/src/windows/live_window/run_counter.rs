use std::collections::HashSet;

use glr_core::token::Token;
use glr_lib::dll_exports::enums::SubscribeCode;

use crate::{dll::parse_continously::ContinousParser, render::Render};

pub struct RunCounter {
    run_counter: usize,
    seed_counter: usize,
    unique_counter: usize,

    seeds: HashSet<i32>,
    continous_parser: ContinousParser<Token>,
}

impl Default for RunCounter {
    fn default() -> Self {
        Self {
            run_counter: 0,
            seed_counter: 0,
            unique_counter: 0,
            seeds: HashSet::new(),
            continous_parser: ContinousParser::new(SubscribeCode::Tokenizer),
        }
    }
}

impl Render for RunCounter {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        while let Some(r) = self.continous_parser.try_recv() {
            match r {
                Token::SelectExpedition(_, seed) => {
                    self.seeds.insert(seed);

                    self.seed_counter += 1;
                    self.unique_counter = self.seeds.len();
                },
                Token::SessionSeed(seed) => {
                    self.seeds.insert(seed as i32);
                    
                    self.run_counter += 1;
                    self.unique_counter = self.seeds.len();
                }
                _ => {}
            }
        }

        ui.label(format!(
            "Run counter: {} Seeds: {} Unique: {}",
            self.run_counter, self.seed_counter, self.unique_counter
        ));

        ui.separator();

        28
    }
}
