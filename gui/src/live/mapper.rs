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

#[derive(Clone)]
pub enum MapperColorError {
    SpannedError(SpannedError),
    FileNotFound,
}

impl Render for MapperColorError {
    type Response = ();

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        match self {
            MapperColorError::SpannedError(spanned_error) => {
                ui.colored_label(Color32::RED, format!("{:?}", spanned_error));
            },
            MapperColorError::FileNotFound => {},
        }
    }
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
    #[allow(private_interfaces)]
    Error(MapperColorError),
}

impl PartialEq for LocationRender {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Key(_), Self::Key(_)) => true,
            (Self::Collectable(_), Self::Collectable(_)) => true,
            (Self::Objective(_), Self::Objective(_)) => true,
            _ => false,
        }
    }
}

impl Eq for LocationRender {}

impl PartialOrd for LocationRender {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LocationRender {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (LocationRender::Key(_), LocationRender::Key(_)) => std::cmp::Ordering::Equal,
            (LocationRender::Key(_), _) => std::cmp::Ordering::Greater,
            (LocationRender::Collectable(_), LocationRender::Key(_)) => std::cmp::Ordering::Less,
            (LocationRender::Collectable(_), LocationRender::Collectable(_)) => std::cmp::Ordering::Equal,
            (LocationRender::Collectable(_), LocationRender::Objective(_)) => std::cmp::Ordering::Greater,
            (LocationRender::Objective(_), LocationRender::Objective(_)) => std::cmp::Ordering::Equal,
            (LocationRender::Objective(_), _) => std::cmp::Ordering::Less,
            (LocationRender::Error(_), LocationRender::Error(_)) => std::cmp::Ordering::Equal,
            (LocationRender::Error(_), _) => std::cmp::Ordering::Greater,
            (_, LocationRender::Error(_)) => std::cmp::Ordering::Less,
        }
    }
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
            LocationRender::Error(mapper_color_error) => {
                mapper_color_error.render(ui)
            },
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
    key_len: usize,
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
            key_len: 0,
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
            .map(|v| {
                if let Err(e) = v {
                    if !self.locations.iter().any(|v| {
                        match (v, e) {
                            (LocationRender::Error(_), MapperColorError::SpannedError(_)) => true,
                            _ => false
                        }
                    }) {
                        self.locations.push(LocationRender::Error(e.clone()));
                    }
                }

                v.as_ref().ok()
            })
            .flatten();

        if level_view.is_some_and(|v| !v.is_valid_zone(&location.get_zone())) {
            return;
        }

        match location {
            Location::ColoredKey(_, _, _) | Location::BulkheadKey(_, _, _) => {
                self.locations.push(LocationRender::Key(KeyLocationRender {
                    location_text: location.to_string(),
                    color: level_view.lookup(self.key_len, location),
                }));

                self.locations.sort_by(|a, b| b.cmp(a));
                self.key_len += 1;
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

                self.locations.sort_by(|a, b| b.cmp(a));
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
                if let Some(LocationRender::Collectable(last_loc)) = self.locations.iter_mut()
                    .find(|v| {
                        if let LocationRender::Collectable(t) = v {
                            return t.name_text == name_text;
                        }

                        false
                    }) {
                    if last_loc.name_text == name_text {
                        last_loc.ids.push((
                            id,
                            level_view
                                .lookup(0, &Location::Gatherable(*item_identifier, *zone, id)),
                        ));

                        last_loc.ids.sort_by_key(|(a, _)| { *a });

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

                self.locations.sort_by(|a, b| b.cmp(a));
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
            .or(Some(update_data.into_result().get_objective_str()))
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
        self.key_len = 0;
    }
}
