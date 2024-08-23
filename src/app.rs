use crate::time_manager::TimeManager;

pub struct MyApp {
  
  time_manager: TimeManager,
  level_name: String,
  
  secondary_included: bool,
  overload_included: bool,
  glitched: bool,

}

impl Default for MyApp {
  fn default() -> MyApp {
    MyApp {
      time_manager: TimeManager::new(),
      level_name: String::new(),
      secondary_included: false,
      overload_included: false,
      glitched: false,
    }
  }
}

impl eframe::App for MyApp {

  fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    egui::Rgba::TRANSPARENT.to_array()
}

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default()/*.frame(egui::Frame::none()) */.show(ctx, |ui| {
      ui.horizontal( |ui| {
        ui.label("Level Name: ");
        ui.text_edit_singleline(&mut self.level_name)
      });
      ui.horizontal( |ui| {
        ui.checkbox(&mut self.secondary_included, "Sec");
        ui.checkbox(&mut self.overload_included, "Ovrld");
        ui.checkbox(&mut self.glitched, "glitch");
      });
      let mut level_id: String = self.level_name.to_string();
      if self.secondary_included { level_id += "_sec"; }
      if self.overload_included { level_id += "_ovrl"; }
      if self.glitched { level_id += "_glitch"; }
      ui.label(format!("ID: {}", level_id));
    });
  }
}