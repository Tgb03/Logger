use egui::Label;

use crate::{dll::{callback::Code, exported_data::OutputSeedIndexer, parse_continously::ContinousParser}, render::Render};


pub struct SeedIndexer {

    data_found: Vec<OutputSeedIndexer>,
    continous_parser: ContinousParser<OutputSeedIndexer>

}

impl SeedIndexer {
    pub fn new() -> Self {
        Self {
            data_found: Vec::new(),
            continous_parser: ContinousParser::new(Code::SeedIndexer as u8)
        }
    }
}

impl Render for OutputSeedIndexer {
    type Response = ();

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        match self {
            OutputSeedIndexer::Key(name, zone, id) => { 
                ui.add(Label::new(format!("{} in ZONE_{} at {}", name, zone, id))); 
            },
            OutputSeedIndexer::ConsumableFound(id, found) => {
                ui.add(Label::new(format!("Container {}: {}",
                    id,
                    match found {
                        true => "FOUND",
                        false => "NONE",
                    }
                )));
            },
            _ => {},
        }
    }
}

impl Render for SeedIndexer {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        while let Some(res) = self.continous_parser.try_recv() {
            match res {
                OutputSeedIndexer::GenerationStart => { self.data_found.clear(); },
                OutputSeedIndexer::GenerationEnd | OutputSeedIndexer::Seed(_) | 
                OutputSeedIndexer::ZoneGenEnded(_) | OutputSeedIndexer::ResourcePack(_, _) => {},
                v => { 
                    self.data_found.push(v); 
                }
            }
        }

        for o in self.data_found.iter_mut() {
            o.render(ui);
        }

        ui.separator();

        6 + self.data_found.len() * 22
    }
}
