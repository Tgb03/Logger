use std::{collections::HashMap, fs, u64};

use egui::{Color32, RichText, Ui, WidgetText};
use itertools::Itertools;
use ron::de::SpannedError;

use crate::{graphics::create_text, logs::location::{Location, LocationType}, save_run::SaveManager};

use super::mapper_view::{LevelView, OptimizedLevelView};

pub trait LookUpColor {

  fn lookup(&self, location_vec_id: usize, location: &Location) -> Option<Color32>;
  fn is_valid_zone(&self, zone: &u64) -> bool;

}

pub enum MapperColorError {
  
  SpannedError(SpannedError),
  FileNotFound,

}

#[derive(Default)]
pub struct Mapper {

  location_colors: HashMap<String, Result<OptimizedLevelView, MapperColorError>>,

}

impl Mapper {

  fn show_colored<T>(ui: &mut Ui, text: T, color: Option<Color32>)
  where T: Into<RichText> + Into<WidgetText> {
    match color {
      Some(color) => ui.colored_label(color, text),
      None => ui.label(text),
    };
  }

  pub fn render_type(
    ui: &mut Ui, 
    locations: &Vec<Location>, 
    show_objectives: bool,
    level_view: Option<&OptimizedLevelView>
  ) -> usize {

    let mut len = 0;

    for (id, key_location) in locations
      .iter()
      .filter(|v| 
        !v.has_type(&LocationType::Objective) &&
        v.get_zone().is_none_or(|z| level_view.is_valid_zone(&z))
      )
      .enumerate() {
      
      let color = level_view.lookup(id, key_location);
      Self::show_colored(ui, create_text(format!("{key_location}")), color);
      len += 1;

    }

    if show_objectives {
      for ((name, zone), group) in locations
        .iter()
        .filter(|v| 
          v.has_type(&LocationType::Objective) &&
          v.get_zone().is_none_or(|z| level_view.is_valid_zone(&z))
        )
        .chunk_by(|v| (v.get_name(), v.get_zone()))
        .into_iter() {

          ui.horizontal(|ui| {
            ui.label(create_text(format!("{}: ZONE {} at",
              name.map(|v| v.as_str()).unwrap_or("No name"),
              zone.map(|v| v.to_string()).unwrap_or("None".to_owned()),
            )));

            for it in group {
              let color = level_view.lookup(0, it);
              Self::show_colored(
                ui, 
                create_text(
                  it.get_id().map(|v| v.to_string()).unwrap_or("No ID".to_owned())
                ), 
                color
              );
            }
          });

          len += 1;

        }
    }

    len * 22
  }

  pub fn render_error(&mut self, ui: &mut Ui, level_name: &String) -> usize {
    if let Some(Err(MapperColorError::SpannedError(error))) = self.location_colors.get(level_name) {
      ui.colored_label(Color32::RED, create_text(format!("{:?}", error)));

      if ui.button(create_text("Reload file")).clicked() {
        self.force_load_level_info(level_name);
      }

      return 12;
    }

    0
  }

  pub fn get_color_info(&self, level_name: &String) -> Option<&OptimizedLevelView> {
    match self.location_colors.get(level_name) {
      Some(result) => result.as_ref().ok(),
      None => None,
    }
  }

  pub fn load_level_info(&mut self, level: &String) {
    if self.location_colors.contains_key(level) { return; }

    self.force_load_level_info(level);
  }

  fn force_load_level_info(&mut self, level: &String) {

    let mut path = SaveManager::get_config_directory().map(
      |v| v.join("levels").join(level)
    );
    path = path.map(|mut m| { m.set_extension("ron"); m });

    if let Some(data) = path.map(|p| fs::read_to_string(p).ok()).flatten() {
      match ron::from_str::<LevelView>(&data) {
        Ok(level_view) => {
          self.location_colors.insert(level.clone(), Ok(level_view.into()));
        }
        Err(e) => {
          self.location_colors.insert(level.clone(), Err(MapperColorError::SpannedError(e)));
        },
      }
    } else {
      self.location_colors.insert(
        level.clone(), 
        Err(MapperColorError::FileNotFound)
      );
    }

  }

}
