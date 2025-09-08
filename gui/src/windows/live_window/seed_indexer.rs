use egui::Label;
use glr_core::seed_indexer_result::OutputSeedIndexer;
use glr_lib::dll_exports::enums::SubscribeCode;

use crate::{dll::parse_continously::ContinousParser, render::Render, windows::settings_window::SettingsWindow};
use indexmap::IndexMap;

pub struct SeedIndexer {

    data_found: Vec<OutputSeedIndexer>,
    end_shown: IndexMap<(i32, String), Vec<i32>>,
    continous_parser: ContinousParser<OutputSeedIndexer>,

    show_resources: bool,
    show_consumables: bool,
    show_artifacts: bool,

    number_of_items: usize,

}

impl SeedIndexer {
    pub fn new(settings: &SettingsWindow) -> Self {
        Self {
            end_shown: IndexMap::new(),
            data_found: Vec::new(),
            continous_parser: ContinousParser::new(SubscribeCode::SeedIndexer),
            show_resources: settings.get("seed_indexer_show_resources").unwrap_or(true),
            show_consumables: settings.get("seed_indexer_show_consumables").unwrap_or(true),
            show_artifacts: settings.get_def("seed_indexer_show_artifacts"),
            number_of_items: settings.get("seed_indexer_length").unwrap_or(10) as usize,
        }
    }
}

impl Render for SeedIndexer {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        while let Some(res) = self.continous_parser.try_recv() {
            match res {
                OutputSeedIndexer::GenerationStart => { 
                    self.data_found.clear(); 
                    self.end_shown.clear(); 
                },
                OutputSeedIndexer::GenerationEnd => {
                    self.end_shown.clear();

                    for data in &self.data_found {
                        match data {
                            OutputSeedIndexer::Key(name, zone, id) => {
                                if self.show_artifacts == false && name.contains("rtifact") { continue; }
                                if self.show_consumables == false && name.contains("onsumable") { continue; }

                                self.end_shown
                                    .entry((*zone, name.clone()))
                                    .or_default()
                                    .push(*id);
                            }
                            OutputSeedIndexer::ResourcePack(t, zone, id, _) => {
                                if self.show_resources == false { continue; }

                                self.end_shown
                                    .entry((*zone, format!("{:?}", t)))
                                    .or_default()
                                    .push(*id);
                            }
                            _ => {}
                        }
                    }
                }
                OutputSeedIndexer::Seed(_) | OutputSeedIndexer::ZoneGenEnded(_) => {},
                v => { 
                    self.data_found.push(v); 
                }
            }
        }

        let row_height = ui.spacing().interact_size.y;
        egui::ScrollArea::vertical()
            .max_height(row_height * self.number_of_items as f32)
            .show_rows(
            ui,
            row_height,
            self.end_shown.len(),
            |ui, row_range| {
                for row in row_range {
                    let (zone_name_pair, ids) = self.end_shown.get_index(row).unwrap();

                    ui.horizontal(|ui| {
                        let ids_str = if ids.len() == 1 {
                            ids[0].to_string()
                        } else {
                            format!("{:?}", ids)
                        };
                        ui.add(Label::new(format!(
                            "{} in ZONE_{}: {}",
                            zone_name_pair.1, zone_name_pair.0, ids_str
                        )));
                    });
                }

                // ui.horizontal(|ui| ui.label(""));
            }
        );

        ui.separator();

        60 + (row_height * self.number_of_items.min(self.end_shown.len()) as f32) as usize
    }
}
