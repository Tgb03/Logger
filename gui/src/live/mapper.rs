use core::{
    logs::{
        collectable_mapper::CollectableMapper, live_parser::LiveParser, location::Location,
        token_parser::TokenParserT,
    },
    run::traits::Run,
    save_manager::SaveManager,
};
use std::{collections::HashMap, fs, u64};

use egui::{Color32, Ui};
use ron::de::SpannedError;

use crate::{
    render::{BufferedRender, Render},
    windows::settings_window::SettingsWindow,
};

use super::mapper_view::{LevelView, LookUpColor, OptimizedLevelView};

pub enum MapperColorError {
    SpannedError(SpannedError),
    FileNotFound,
}

struct KeyLocationRender {
    location_text: String,
    color: Option<Color32>,
}

impl Render for KeyLocationRender {
    type Response = ();

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        ui.horizontal(|ui| {
            match self.color {
                Some(color) => ui.colored_label(color, &self.location_text),
                None => ui.label(&self.location_text),
            };
        });
    }
}

struct ObjectiveLocationRender {
    name_color: Option<Color32>,
    name_text: String,

    ids: Vec<(u64, Option<Color32>)>,
}

impl Render for ObjectiveLocationRender {
    type Response = ();

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        ui.horizontal(|ui| {
            match self.name_color {
                Some(color) => ui.colored_label(color, &self.name_text),
                None => ui.label(&self.name_text),
            };

            for (id, color) in &self.ids {
                match color {
                    Some(color) => ui.colored_label(*color, id.to_string()),
                    None => ui.label(id.to_string()),
                };
            }
        });
    }
}

pub enum LocationRender {
    #[allow(private_interfaces)]
    Key(KeyLocationRender),
    #[allow(private_interfaces)]
    Collectable(ObjectiveLocationRender),
    #[allow(private_interfaces)]
    Objective(KeyLocationRender),
}

impl Render for LocationRender {
    type Response = ();

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        match self {
            LocationRender::Key(key_location_render) => key_location_render.render(ui),
            LocationRender::Objective(objective_location_render) => {
                objective_location_render.render(ui)
            }
            LocationRender::Collectable(objective_location_render) => {
                objective_location_render.render(ui)
            }
        }
    }
}

impl Render for Vec<LocationRender> {
    type Response = usize;

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        for it in self.iter_mut() {
            it.render(ui)
        }

        ui.separator();

        self.len() * 22 + 6
    }
}

pub struct Mapper<'a> {
    location_colors: HashMap<String, Result<OptimizedLevelView, MapperColorError>>,

    locations: Vec<LocationRender>,
    locations_len: usize,
    run_counter: u64,

    show_objectives: bool,
    level_name: String,
    level_objective: String,
    compare_obj: String,

    collectable_mapper: Option<&'a CollectableMapper>,
}

impl<'a> Mapper<'a> {
    pub fn new(
        collectable_mapper: Option<&'a CollectableMapper>,
        settings_window: &SettingsWindow,
    ) -> Self {
        Self {
            location_colors: Default::default(),
            locations: Default::default(),
            locations_len: 0,
            run_counter: 0,
            show_objectives: settings_window.get_show_objective_items(),
            level_name: "".to_owned(),
            compare_obj: "".to_owned(),
            level_objective: "".to_owned(),
            collectable_mapper,
        }
    }

    fn load_level_info(&mut self, level: &str) {
        if self.location_colors.contains_key(level) {
            return;
        }

        self.force_load_level_info(level);
    }

    fn force_load_level_info(&mut self, level: &str) {
        let mut path = SaveManager::get_config_directory().map(|v| v.join("levels").join(level));
        path = path.map(|mut m| {
            m.set_extension("ron");
            m
        });

        println!("Attempting load with: {:?}", path);

        if let Some(data) = path.map(|p| fs::read_to_string(p).ok()).flatten() {
            match ron::from_str::<LevelView>(&data) {
                Ok(level_view) => {
                    self.location_colors
                        .insert(level.to_owned(), Ok(level_view.into()));
                    println!("Loaded correctly: {}", level);
                }
                Err(e) => {
                    self.location_colors
                        .insert(level.to_owned(), Err(MapperColorError::SpannedError(e)));
                    println!("Spanned Error: {}", level);
                }
            }
        } else {
            self.location_colors
                .insert(level.to_owned(), Err(MapperColorError::FileNotFound));
            println!("File not found: {}", level);
        }
    }

    fn add_location(&mut self, location: &Location) {
        let level_view = self
            .location_colors
            .get(&self.level_objective)
            .map(|v| v.as_ref().ok())
            .flatten();

        if level_view.is_some_and(|v| !v.is_valid_zone(&location.get_zone())) {
            return;
        }

        match location {
            Location::ColoredKey(_, _, _) | Location::BulkheadKey(_, _, _) => {
                self.locations.push(LocationRender::Key(KeyLocationRender {
                    location_text: location.to_string(),
                    color: level_view.lookup(self.locations_len, location),
                }))
            }
            Location::BigObjective(_, _, _) | Location::BigCollectable(_, _) => {
                if self.show_objectives == false {
                    return;
                }

                self.locations
                    .push(LocationRender::Objective(KeyLocationRender {
                        location_text: location.to_string(),
                        color: level_view.lookup(0, location),
                    }));
            }
            Location::Gatherable(item_identifier, zone, id) => {
                if self.show_objectives == false {
                    return;
                }

                let id = match self
                    .collectable_mapper
                    .map(|v| v.get_id(&self.level_name, *zone, *id))
                    .flatten()
                {
                    Some(new_id) => new_id,
                    None => *id,
                };

                let name_text = format!("{}: ZONE {} at", item_identifier.to_string(), zone);
                if let Some(LocationRender::Collectable(last_loc)) = self.locations.last_mut() {
                    if last_loc.name_text == name_text {
                        last_loc.ids.push((
                            id,
                            level_view
                                .lookup(0, &Location::Gatherable(*item_identifier, *zone, id)),
                        ));

                        return;
                    }
                }

                self.locations
                    .push(LocationRender::Collectable(ObjectiveLocationRender {
                        name_color: None,
                        name_text,
                        ids: vec![(
                            id,
                            level_view
                                .lookup(0, &Location::Gatherable(*item_identifier, *zone, id)),
                        )],
                    }));
            }
        }
    }
}

impl<'a> BufferedRender for Mapper<'a> {
    type Response = usize;
    type UpdateData = LiveParser;
    type Render = Vec<LocationRender>;

    fn update(&mut self, update_data: &LiveParser) {
        let locations = update_data
            .get_generation_parser()
            .map(|gp| gp.into_result())
            .unwrap_or(update_data.into_result().get_locations());

        update_data
            .get_run_parser()
            .map(|rp| rp.into_result().get_objective_str())
            .or(update_data
                .into_result()
                .get_runs()
                .last()
                .map(|r| r.get_objective_str()))
            .map(|v| {
                if &self.compare_obj != v {
                    let mut s = v.trim_end_matches(".save").to_owned();
                    s.push_str(".ron");
                    self.load_level_info(&s);
                    self.compare_obj = v.clone();
                    self.level_objective = s;
                }
            });

        if &self.level_name != update_data.into_result().get_level_name() {
            self.level_name = update_data.into_result().get_level_name().clone();
        }

        if update_data.into_result().get_counter() != self.run_counter {
            self.reset();
            self.run_counter = update_data.into_result().get_counter();
        }

        for location in locations.iter().skip(self.locations_len) {
            self.add_location(location);
            self.locations_len += 1;
        }
    }

    fn get_renderer(&mut self) -> &mut Self::Render {
        &mut self.locations
    }

    fn reset(&mut self) {
        self.locations.clear();
        self.locations_len = 0;
    }
}
