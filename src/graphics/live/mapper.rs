use std::{collections::HashMap, fs, u64};

use egui::{Color32, Ui};

use crate::{graphics::create_text, logs::location::{Location, LocationType}, save_run::SaveManager};

use super::mapper_view::{LevelView, OptimizedLevelView};

pub trait LookUpColor {

  fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32>;
  fn is_valid_zone(&self, zone: &u64) -> bool;

}

#[derive(Default)]
pub struct Mapper {

  location_colors: HashMap<String, Option<OptimizedLevelView>>,

}

impl Mapper {

  pub fn render_type(
    ui: &mut Ui, 
    locations: &Vec<Location>, 
    show_objectives: bool,
    level_view: Option<&OptimizedLevelView>,
  ) -> usize {

    let mut len = 0;

    for (id, location) in locations.iter().enumerate() {
      if location.get_zone().is_some_and(|z| !level_view.is_valid_zone(&z)) { continue; }

      if show_objectives || !location.has_type(&LocationType::Objective) {
        let color = level_view.lookup(id, location);

        match color {
          Some(color) => {
            ui.colored_label(
              color,
              create_text(format!("{}", location))
            );
          },
          None => {
            ui.label(create_text(format!("{}", location)));
          },
        }
        len += 1;
      }
    }

    len * 22
  }

  pub fn get_color_info(&self, level_name: &String) -> Option<&OptimizedLevelView> {
    self.location_colors.get(level_name)?.as_ref()
  }

  pub fn load_level_info(&mut self, level: &String) {
    if self.location_colors.contains_key(level) { return; }

    let mut path = SaveManager::get_directory().map(
      |v| v.join("config").join("levels").join(level)
    );
    path = path.map(|mut m| { m.set_extension("ron"); m });

    if let Some(data) = path.map(|p| fs::read_to_string(p).ok()).flatten() {
      match ron::from_str::<LevelView>(&data) {
        Ok(level_view) => {
          println!("Loaded: {:?}", level_view);
          self.location_colors.insert(level.clone(), Some(level_view.into()));
        }
        Err(e) => {
          println!("{:?}", e);
          self.location_colors.insert(level.clone(), None);
        },
      }
    } else {
      self.location_colors.insert(level.clone(), None);
    }
  }

}
