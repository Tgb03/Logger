use egui::Ui;

use crate::{graphics::create_text, logs::{location::LocationType, parser::ParserResult}};

#[derive(Default)]
pub struct Mapper;

impl Mapper {

  pub fn render_type(ui: &mut Ui, parser_result: &ParserResult, location_type: Option<LocationType>) -> usize {

    let mut len = 0;

    for location in parser_result.get_locations() {
      if location_type.as_ref().is_none_or(|lt| location.has_type(&lt)) {
        ui.label(create_text(format!("{}", location)));
        len += 1;
      }
    }

    len * 22
  }



}
