use core::{run::objectives::run_objective::RunObjective, save_manager::SaveManager};
use std::{collections::{HashMap, VecDeque}, fs, u64};

use egui::{Color32, Ui};
use glr_core::location::Location;
use glr_lib::dll_exports::enums::SubscribeCode;
use ron::de::SpannedError;

use crate::{
    dll::{parse_continously::ContinousParser}, render::Render, windows::{live_window::objective_reader::{ObjectiveReader, UpdateObjective}, settings_window::SettingsWindow}
};

use super::mapper_view::{LevelView, LookUpColor, OptimizedLevelView};

#[derive(Clone, Debug)]
pub enum MapperColorError {
    SpannedError(SpannedError),
    FileNotFound,
}

impl Render for MapperColorError {
    type Response = ();

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        match self {
            MapperColorError::SpannedError(spanned_error) => {
                spanned_error.render(ui)
            },
            MapperColorError::FileNotFound => {},
        }
    }
}

impl Render for SpannedError {
    type Response = ();

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        ui.colored_label(Color32::RED, format!("{:?}", self));
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum LocationRender {
    #[allow(private_interfaces)]
    Key(KeyLocationRender),
    #[allow(private_interfaces)]
    Collectable(ObjectiveLocationRender),
    #[allow(private_interfaces)]
    Objective(KeyLocationRender),
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
        }
    }
}

#[derive(Default)]
pub struct LocationRenderVec {

    vec: Vec<LocationRender>,
    error_found: Option<MapperColorError>,

}

impl Render for LocationRenderVec {
    type Response = usize;

    fn render(&mut self, ui: &mut Ui) -> Self::Response {
        for it in self.vec.iter_mut() {
            it.render(ui)
        }
        if let Some(MapperColorError::SpannedError(error)) = &mut self.error_found {
            error.render(ui);

            return 1000;
        }

        ui.separator();

        self.vec.len() * 22 + 6
    }
}

pub struct Mapper {
    location_colors: HashMap<String, Result<OptimizedLevelView, MapperColorError>>,
    level_objective: String,
    key_len: usize,
    show_objectives: bool,

    continous_parser: ContinousParser<Location>,
    locations_copy: VecDeque<Location>,

    locations: LocationRenderVec,
}

impl Mapper {
    pub fn new(
        settings_window: &SettingsWindow,
        objective: String,
    ) -> Self {
        Self {
            continous_parser: ContinousParser::new(SubscribeCode::Mapper),
            level_objective: objective,
            location_colors: Default::default(),
            locations: Default::default(),
            locations_copy: VecDeque::new(),
            show_objectives: settings_window.get_def("show_objectives"),
            key_len: 0,
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

        println!("<MapperView> Attempting load with: {:?}", path);

        if let Some(data) = path.map(|p| fs::read_to_string(p).ok()).flatten() {
            match ron::from_str::<LevelView>(&data) {
                Ok(level_view) => {
                    self.location_colors
                        .insert(level.to_owned(), Ok(level_view.into()));
                    println!("<MapperView> Loaded correctly: {}", level);
                }
                Err(e) => {
                    self.location_colors
                        .insert(level.to_owned(), Err(MapperColorError::SpannedError(e)));
                    println!("<MapperView> Spanned Error: {}", level);
                }
            }
        } else {
            self.location_colors
                .insert(level.to_owned(), Err(MapperColorError::FileNotFound));
            println!("<MapperView> File not found: {}", level);
        }
    }

    fn add_location(&mut self, location: &Location) {
        self.locations_copy.push_back(location.clone());

        let level_view = self
            .location_colors
            .get(&self.level_objective)
            .map(|v| {
                if let Err(e) = v {
                    self.locations.error_found = Some(e.clone());
                }

                v.as_ref().ok()
            })
            .flatten();

        if level_view.is_some_and(|v| !v.is_valid_zone(&location.get_zone())) {
            return;
        }

        match location {
            Location::ColoredKey(_, _, _) | Location::BulkheadKey(_, _, _) => {
                self.locations.vec.push(LocationRender::Key(KeyLocationRender {
                    location_text: location.to_string(),
                    color: level_view.lookup(self.key_len, location),
                }));

                self.locations.vec.sort_by(|a, b| b.cmp(a));
                self.key_len += 1;
            }
            Location::BigObjective(_, _, _) | Location::BigCollectable(_, _) => {
                if self.show_objectives == false {
                    return;
                }

                self.locations
                    .vec
                    .push(LocationRender::Objective(KeyLocationRender {
                        location_text: location.to_string(),
                        color: level_view.lookup(0, location),
                    }));
                
                self.locations.vec.sort_by(|a, b| b.cmp(a));
            }
            Location::Gatherable(item_identifier, zone, id) => {
                if self.show_objectives == false {
                    return;
                }

                let name_text = format!("{}: ZONE {} at", item_identifier.to_string(), zone);
                if let Some(LocationRender::Collectable(last_loc)) = self.locations.vec.iter_mut()
                    .find(|v| {
                        if let LocationRender::Collectable(t) = v {
                            return t.name_text == name_text;
                        }

                        false
                    }) {
                    if last_loc.name_text == name_text {
                        last_loc.ids.push((
                            *id,
                            level_view
                                .lookup(0, &Location::Gatherable(*item_identifier, *zone, *id)),
                        ));

                        last_loc.ids.sort_by_key(|(a, _)| { *a });
                        return;
                    }
                }

                self.locations
                    .vec
                    .push(LocationRender::Collectable(ObjectiveLocationRender {
                        name_color: None,
                        name_text,
                        ids: vec![(
                            *id,
                            level_view
                                .lookup(0, &Location::Gatherable(*item_identifier, *zone, *id)),
                        )],
                    }));

                self.locations.vec.sort_by(|a, b| b.cmp(a));
            }
            Location::GenerationStarted(_) => {},
        }
    }

    pub fn render(&mut self, reader: &impl ObjectiveReader<Objective = RunObjective>, ui: &mut Ui) -> usize {
        while let Some(location) = self.continous_parser.try_recv() {
            match location {
                Location::GenerationStarted(level) => {
                    self.key_len = 0;
                    self.locations.vec.clear();
                    self.locations_copy.clear();
                    if let Some(ro) = format!("{}.save", level).as_str().try_into().ok() {
                        let level = reader.override_obj(ro).to_string();
                        self.load_level_info(&level);
                        self.level_objective = level;
                    }
                },
                v => self.add_location(&v),
            }
        }

        self.locations.render(ui)
    }
}

impl UpdateObjective for Mapper {
    type Objective = RunObjective;
    
    fn update(&mut self, reader: &impl ObjectiveReader<Objective = Self::Objective>) {
        if let Ok(obj) = TryInto::<RunObjective>::try_into(self.level_objective.as_str()) {
            let obj = reader.override_obj(obj);
            self.level_objective = obj.to_string();
            
            self.key_len = 0;
            self.locations.vec.clear();
            self.load_level_info(&self.level_objective.clone());
            let mut clone = self.locations_copy.clone();
            self.locations_copy.clear();

            while let Some(l) = clone.pop_front() {
                self.add_location(&l);
            }
        }
    }
}
