
use std::{collections::HashMap, fs::File, io::Read, path::Path};

use egui::{Color32, Rect, RichText, Ui, WidgetText};

pub struct SettingsWindow {

  live_rectangle: Rect,
  automatic_loading: bool,

  text_inputs: [String; 4],

}

impl Default for SettingsWindow {
  fn default() -> Self {

    let path = Path::new(env!("HOME")).join("Appdata\\Locallow\\Tgb03\\GTFO Logger\\app.properties");
    let file_str: String = match File::open(path) {
      Ok(mut file) => {
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer);
        buffer
      },
      Err(_) => String::new(),
    };

    let props: HashMap<String, String> = match serde_yaml::from_str(&file_str) {
      Ok(map) => map,
      Err(_) => HashMap::new(),
    };

    let x_pos: f32 = match props.get("x_pos") {
      Some(s) => s.parse::<f32>().unwrap_or(0.0),
      None => 0.0,
    };
    let y_pos: f32 = match props.get("y_pos") {
      Some(s) => s.parse::<f32>().unwrap_or(100.0),
      None => 100.0,
    };
    let x_size: f32 = match props.get("x_size") {
      Some(s) => s.parse::<f32>().unwrap_or(180.0),
      None => 180.0,
    };
    let y_size: f32 = match props.get("y_size") {
      Some(s) => s.parse::<f32>().unwrap_or(100.0),
      None => 100.0,
    };
    let automatic_loading = match props.get("automatic_loading") {
      Some(s) => s.parse::<bool>().unwrap_or(false),
      None => false,
    };
    
    let live_rectangle = Rect { 
      min: [x_pos, y_pos].into(), 
      max: [x_pos + x_size, y_pos + y_size].into() 
    };

    Self { 
      live_rectangle,
      automatic_loading,
      
      text_inputs: [
        x_pos.to_string(),
        y_pos.to_string(),
        x_size.to_string(),
        y_size.to_string(),
      ],
    }
  }
}

impl SettingsWindow {

  pub fn get_live_rectangle(&self) -> Rect {
    self.live_rectangle
  }

  pub fn get_automatic_loading(&self) -> bool {
    self.automatic_loading
  }

  pub fn show(&mut self, ui: &mut Ui) {

    ui.add_space(10.0);

    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(RichText::from("X position").color(Color32::WHITE));
      if ui.add(egui::TextEdit::singleline(&mut self.text_inputs[0])
        .desired_width(100.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          if let Ok(x) = self.text_inputs[0].parse::<f32>() {
            self.live_rectangle.set_left(x);
          }
        };
    });
    
    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(RichText::from("Y position").color(Color32::WHITE));
      if ui.add(egui::TextEdit::singleline(&mut self.text_inputs[1])
        .desired_width(100.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          if let Ok(y) = self.text_inputs[1].parse::<f32>() {
            self.live_rectangle.set_top(y);
          }
        };
    });
    
    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(RichText::from("X size    ").color(Color32::WHITE));
      if ui.add(egui::TextEdit::singleline(&mut self.text_inputs[2])
        .desired_width(100.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          if let Ok(x) = self.text_inputs[2].parse::<f32>() {
            self.live_rectangle.set_right(x + self.live_rectangle.left());
          }
        };
    });

    ui.horizontal(|ui| { 
      ui.add_space(5.0);
      ui.checkbox(&mut self.automatic_loading, WidgetText::from("Automatic Loading of Runs")
        .color(Color32::WHITE)
      );
    });
  
  }

  pub fn save_settings(&self) {

    let mut s = String::new();

    s.push_str(&format!("x_pos: {}\n", self.live_rectangle.left()));
    s.push_str(&format!("y_pos: {}\n", self.live_rectangle.bottom()));
    s.push_str(&format!("x_size: {}\n", self.live_rectangle.right() - self.live_rectangle.left()));
    s.push_str(&format!("y_size: {}\n", self.live_rectangle.top() - self.live_rectangle.bottom()));
    s.push_str(&format!("automatic_loading: {}\n", self.automatic_loading));

    let path = Path::new(env!("HOME")).join("Appdata\\Locallow\\Tgb03\\GTFO Logger\\app.properties");
    
    let _ = std::fs::write(path, s);

  }

}
