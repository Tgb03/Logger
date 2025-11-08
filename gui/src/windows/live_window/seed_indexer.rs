use core::save_manager::SaveManager;
use std::{collections::HashMap, fs};

use egui::{Color32, Label, RichText};
use glr_core::seed_indexer_result::OutputSeedIndexer;
use glr_lib::dll_exports::enums::SubscribeCode;

use crate::{
    dll::parse_continously::ContinousParser,
    render::Render,
    windows::{
        live_window::foresight_view::{
            AddToConditions, ForesightView, LookUpForesight, OptimizedForesightView
        },
        settings_window::SettingsWindow,
    },
};
use indexmap::IndexMap;

pub struct SeedIndexer {
    data_found: Vec<OutputSeedIndexer>,
    end_shown: IndexMap<(i32, String), Vec<(i32, Option<Color32>)>>,
    continous_parser: ContinousParser<OutputSeedIndexer>,

    views: HashMap<String, OptimizedForesightView>,
    objective: String,

    show_gather_small_items: bool,
    show_fog_turbine: bool,
    show_cell: bool,
    show_colored_key: bool,
    show_bulkhead_key: bool,
    show_terminal_uplink: bool,
    show_special_terminal_command: bool,
    show_retrieve_big_items: bool,
    show_hsu: bool,
    show_power_cell_distribution: bool,

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
            views: HashMap::new(),
            objective: Default::default(),

            show_gather_small_items: settings
                .get("seed_indexer_show_gather_small_items")
                .unwrap_or(true),
            show_fog_turbine: settings
                .get("seed_indexer_show_fog_turbine")
                .unwrap_or(true),
            show_cell: settings.get("seed_indexer_show_cell").unwrap_or(true),
            show_colored_key: settings
                .get("seed_indexer_show_colored_key")
                .unwrap_or(true),
            show_bulkhead_key: settings
                .get("seed_indexer_show_bulkhead_key")
                .unwrap_or(true),
            show_terminal_uplink: settings
                .get("seed_indexer_show_terminal_uplink")
                .unwrap_or(true),
            show_retrieve_big_items: settings
                .get("seed_indexer_show_retrieve_big_items")
                .unwrap_or(true),
            show_special_terminal_command: settings
                .get("seed_indexer_show_special_terminal_command")
                .unwrap_or(true),
            show_hsu: settings.get("seed_indexer_show_hsu").unwrap_or(true),
            show_power_cell_distribution: settings
                .get("seed_indexer_show_power_cell_distribution")
                .unwrap_or(true),

            show_resources: settings.get("seed_indexer_show_resources").unwrap_or(true),
            show_consumables: settings
                .get("seed_indexer_show_consumables")
                .unwrap_or(true),
            show_artifacts: settings.get_def("seed_indexer_show_artifacts"),
            number_of_items: settings.get("seed_indexer_length").unwrap_or(10) as usize,
        }
    }

    pub fn update_view(&mut self) {
        let mut path =
            SaveManager::get_config_directory().map(|v| v.join("foresight").join(self.objective.clone()));
        path = path.map(|mut m| {
            m.set_extension("ron");
            m
        });
            
        if let Some(data) = path.as_ref().map(|v| fs::read_to_string(v).ok()).flatten() {
            match ron::from_str::<ForesightView>(&data) {
                Ok(f_view) => {
                    println!("Foresight loaded view: {:?}", self.objective);
                    self.views.insert(self.objective.clone(), f_view.into());
                }
                Err(err) => println!("ForesightView loading error: {err}"),
            }
        } else {
            println!("Foresight couldn't read {path:?}");
        }
    }
}

impl Render for SeedIndexer {
    type Response = usize;

    fn render(&mut self, ui: &mut egui::Ui) -> Self::Response {
        while let Some(res) = self.continous_parser.try_recv() {
            match res {
                OutputSeedIndexer::GenerationStart(name) => {
                    self.data_found.clear();
                    self.end_shown.clear();
                    self.objective = name;
                    self.views.get_mut(&self.objective).reset();
                    
                    self.update_view();
                }
                OutputSeedIndexer::GenerationEnd => {
                    self.end_shown.clear();

                    for data in &self.data_found {
                        match data {
                            OutputSeedIndexer::Key(name, zone, id) => {
                                if self.views.get(&self.objective).is_ignored(name, zone, id) { continue; }

                                if self.show_artifacts == false && name.contains("rtifact") {
                                    continue;
                                }
                                if self.show_consumables == false && name.contains("onsumable") {
                                    continue;
                                }
                                if self.show_gather_small_items == false
                                    && name.as_str() == "GatherSmallItems"
                                {
                                    continue;
                                }
                                if self.show_fog_turbine == false && name.as_str() == "FOG_TURBINE"
                                {
                                    continue;
                                }
                                if self.show_cell == false && name.as_str() == "Cell" {
                                    continue;
                                }
                                if self.show_bulkhead_key == false && name.as_str() == "BulkKey" {
                                    continue;
                                }
                                if self.show_colored_key == false
                                    && name.get(0..3).is_some_and(|v| v == "Key")
                                {
                                    continue;
                                }
                                if self.show_terminal_uplink == false
                                    && name.as_str() == "TerminalUplink"
                                {
                                    continue;
                                }
                                if self.show_retrieve_big_items == false
                                    && name.as_str() == "RetrieveBigItems"
                                {
                                    continue;
                                }
                                if self.show_special_terminal_command == false
                                    && name.as_str() == "SpecialTerminalCommand"
                                {
                                    continue;
                                }
                                if self.show_hsu == false && name.as_str() == "HSU_FindTakeSample" {
                                    continue;
                                }
                                if self.show_power_cell_distribution == false
                                    && name.as_str() == "PowerCellDistribution"
                                {
                                    continue;
                                }

                                let color =
                                    self.views.get(&self.objective).lookup(name, zone, id);

                                let new_name = self.views.get(&self.objective)
                                    .rename(&name)
                                    .unwrap_or_else(|| name.clone());

                                self.end_shown
                                    .entry((*zone, new_name))
                                    .or_default()
                                    .push((*id, color));

                                self.end_shown.sort_by(|(zone1, name1), _, (zone2, name2), _| {
                                    let zone_cmp1 = self.views.get(&self.objective)
                                        .get_order(name1, zone1)
                                        .unwrap_or_else(|| *zone1 as usize);
                                    let zone_cmp2 = self.views.get(&self.objective)
                                        .get_order(name2, zone2)
                                        .unwrap_or_else(|| *zone2 as usize);

                                    zone_cmp1.cmp(&zone_cmp2)
                                });
                            }
                            OutputSeedIndexer::ResourcePack(t, zone, id, _) => {
                                if self.show_resources == false {
                                    continue;
                                }
                                
                                let name = format!("{:?}", t);

                                let new_name = self.views.get(&self.objective)
                                    .rename(&name)
                                    .unwrap_or_else(|| name.clone());
                                
                                if self.views.get(&self.objective).is_ignored(&name, zone, id) { continue; }
                                let color =
                                    self.views.get(&self.objective).lookup(&name, zone, id);
                                
                                self.end_shown
                                    .entry((*zone, new_name))
                                    .or_default()
                                    .push((*id, color));

                                self.end_shown.sort_by_key(|(v, _), _| *v);
                            }
                            _ => {}
                        }
                    }
                }
                OutputSeedIndexer::Seed(_) | OutputSeedIndexer::ZoneGenEnded(_) => {}
                v => {
                    match &v {
                        OutputSeedIndexer::Key(name, zone, id) => {
                            self.views.get_mut(&self.objective)
                                .add_found(name, zone, id);
                        },
                        OutputSeedIndexer::ResourcePack(resource_type, zone, id, _) => {
                            let name = format!("{resource_type:?}");
                            self.views.get_mut(&self.objective)
                                .add_found(&name, zone, id);
                        },
                        _ => {}
                    }
                    self.data_found.push(v);
                }
            }
        }

        let mut count_separators = 0;
        let row_height = ui.spacing().interact_size.y;
        egui::ScrollArea::vertical()
            .max_height(row_height * self.number_of_items as f32)
            .show_rows(ui, row_height, self.end_shown.len(), |ui, row_range| {
                let mut last_grouped = -1;
                let mut spit_separator = false;

                for row in row_range {
                    let ((zone, name), ids) = self.end_shown.get_index(row).unwrap();

                    if last_grouped != *zone && spit_separator {
                        if !self.views.get(&self.objective).is_grouped(zone) {
                            let size = egui::vec2(150.0, 1.0);
                            let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());
                            count_separators += 1;

                            let painter = ui.painter();
                            let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
                            painter.line_segment([rect.left_center(), rect.right_center()], stroke);
                        }
                        
                        spit_separator = false;
                    }

                    if last_grouped != *zone && self.views.get(&self.objective).is_grouped(zone) {
                        last_grouped = *zone;
                        
                        let size = egui::vec2(150.0, 1.0);
                        let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());

                        let painter = ui.painter();
                        let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
                        painter.line_segment([rect.left_center(), rect.right_center()], stroke);
                            count_separators += 1;

                        ui.add(
                            Label::new(RichText::new(format!("   ZONE_{}   ", zone))
                                //.underline()
                                .strong()
                                .heading()
                        ));
                        spit_separator = true;
                    }

                    ui.horizontal(|ui| {
                        if last_grouped != *zone {
                            ui.add(Label::new(format!("{} in ZONE_{}:", name, zone)));
                        } else {
                            ui.add(Label::new(format!("{}:", name)));
                        }

                        for (id, color) in ids {
                            match color {
                                Some(color) => ui.colored_label(*color, format!("{id}")),
                                None => ui.label(format!("{id}")),
                            };
                        }
                    });
                }
            });

        ui.separator();

        60 + (row_height * self.number_of_items.min(self.end_shown.len()) as f32) as usize + count_separators * (row_height as usize + 4)
    }
}
