use egui::{Color32, Vec2};



pub struct SplitterApp {
  
  level_name: String,
  secondary_included: bool,
  overload_included: bool,
  glitched: bool,
  early_drop: bool,

}

impl Default for SplitterApp {
  fn default() -> SplitterApp {
    SplitterApp {
      level_name: String::new(),
      secondary_included: false,
      overload_included: false,
      glitched: false,
      early_drop: false,
    }
  }
}

impl eframe::App for SplitterApp {

  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    egui::Rgba::TRANSPARENT.to_array()
  }

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let frame = egui::containers::Frame {
        inner_margin: egui::Margin { left: 1., right: 1., top: 1., bottom: 1.},
        outer_margin: egui::Margin { left: 1., right: 1., top: 1., bottom: 1.},
        rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0},
        shadow: egui::Shadow { offset: Vec2::ZERO, blur: 0.0, spread: 0.0, color: Color32::TRANSPARENT },
        fill: Color32::BLACK,
        stroke: egui::Stroke::new(1.0, Color32::TRANSPARENT),
    };
    egui::TopBottomPanel::top("TopPanel").frame(frame).show(ctx, |ui| {
      ui.horizontal( |ui| {
        ui.label("Level Name: ");
        ui.text_edit_singleline(&mut self.level_name)
      });
      ui.horizontal( |ui| {
        ui.checkbox(&mut self.secondary_included, "secondary");
        ui.checkbox(&mut self.overload_included, "overload");
      });
      ui.horizontal( |ui| {
        ui.checkbox(&mut self.glitched, "glitch");
        ui.checkbox(&mut self.early_drop, "early_drop");
      });
      let mut level_id: String = self.level_name.to_string().to_uppercase();
      if self.secondary_included { level_id += "_sec"; }
      if self.overload_included { level_id += "_ovrl"; }
      if self.glitched { level_id += "_glitch"; }
      if self.early_drop { level_id += "_edrop"; }
      ui.label(format!("ID: {}", level_id));
    });
  }
}