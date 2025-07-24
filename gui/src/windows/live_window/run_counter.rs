use core::logs::token::Token;
use std::collections::HashSet;

use crate::{dll::{callback::Code, parse_continously::ContinousParser}, render::Render};


pub struct RunCounter {
    run_counter: usize,
    seed_counter: usize,

    seeds: HashSet<u64>,
    continous_parser: ContinousParser<Token>,
}

impl Default for RunCounter {
    fn default() -> Self {
        Self {
            run_counter: 0,
            seed_counter: 0,
            seeds: HashSet::new(),
            continous_parser: ContinousParser::new(Code::Tokenizer as u8),
        }
    }
}

impl Render for RunCounter {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        while let Some(r) = self.continous_parser.try_recv() {
            match r {
                Token::SessionSeed(seed) => {
                    self.seeds.insert(seed);

                    self.run_counter += 1;
                    self.seed_counter = self.seeds.len();
                },
                _ => {}
            }
        }

        ui.label(format!("Run counter: {} Unique: {}", self.run_counter, self.seed_counter));

        ui.separator();

        28
    }
}

