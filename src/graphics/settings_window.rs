
use std::{collections::HashMap, fs::File, io::Read, path::Path};

use egui::{Color32, Rect, Ui};

pub struct SettingsWindow {

  live_rectangle: Rect,
  automatic_loading: bool,
  compare_to_record: bool,
  compare_to_theoretical: bool,

  show_warden_mapper: bool,
  show_objective_items: bool,

  show_code_guess: bool,
  code_guess_line_count: usize,
  code_guess_line_width: usize,

  text_inputs: [String; 6],

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
    let automatic_loading = match props.get("automatic_loading") {
      Some(s) => s.parse::<bool>().unwrap_or(false),
      None => false,
    };
    let compare_to_record = match props.get("compare_to_record") {
      Some(s) => s.parse::<bool>().unwrap_or(false),
      None => true,
    };
    let compare_to_theoretical = match props.get("compare_to_theoretical") {
      Some(s) => s.parse::<bool>().unwrap_or(false),
      None => false,
    };
    let show_warden_mapper = match props.get("show_warden_mapper") {
      Some(s) => s.parse::<bool>().unwrap_or(false),
      None => false,
    };
    let show_objective_items = match props.get("show_objective_items") {
      Some(s) => s.parse::<bool>().unwrap_or(false),
      None => false,
    };
    let show_code_guess = match props.get("show_code_guess") {
      Some(s) => s.parse::<bool>().unwrap_or(false),
      None => false,
    };
    let code_guess_line_count = match props.get("code_guess_line_count") {
      Some(s) => s.parse::<usize>().unwrap_or(1),
      None => 1,
    };
    let code_guess_line_width = match props.get("code_guess_line_width") {
      Some(s) => s.parse::<usize>().unwrap_or(6),
      None => 6,
    };
    
    let live_rectangle = Rect { 
      min: [x_pos, y_pos].into(), 
      max: [x_pos + x_size, y_pos + 80.0].into() 
    };

    Self { 
      live_rectangle,
      automatic_loading,
      compare_to_record,
      compare_to_theoretical,
      show_warden_mapper,
      show_objective_items,
      show_code_guess,
      code_guess_line_count,
      code_guess_line_width,

      text_inputs: [
        x_pos.to_string(),
        y_pos.to_string(),
        x_size.to_string(),
        80.to_string(),
        code_guess_line_count.to_string(),
        code_guess_line_width.to_string(),
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

  pub fn get_compare_to_record(&self) -> bool {
    self.compare_to_record
  }

  pub fn get_compare_to_theoretical(&self) -> bool {
    self.compare_to_theoretical
  }

  pub fn get_show_warden_mapper(&self) -> bool {
    self.show_warden_mapper
  }

  pub fn get_show_code_guess(&self) -> bool {
    self.show_code_guess
  }

  pub fn get_code_guess_line_count(&self) -> usize {
    self.code_guess_line_count
  }

  pub fn get_show_objective_items(&self) -> bool {
    self.show_objective_items
  }

  pub fn get_code_guess_line_width(&self) -> usize {
    self.code_guess_line_width
  }

  pub fn show(&mut self, ui: &mut Ui) {

    ui.add(egui::Label::new(super::create_text("LiveSplitter settings: ")
      .size(14.0)));

    ui.add_space(10.0);

    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(super::create_text("X position"));
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
      ui.monospace(super::create_text("Y position"));
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
      ui.monospace(super::create_text("X size    "));
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
      ui.checkbox(&mut self.compare_to_record, super::create_text("Compare to saved record."));
    });

    ui.horizontal(|ui| { 
      ui.add_space(5.0);
      ui.checkbox(&mut self.compare_to_theoretical, super::create_text("Compare to best splits"));
    });

    ui.separator();

    ui.add(egui::Label::new(super::create_text("Mapper settings: ")
      .size(14.0)));
    ui.add_space(10.0);

    ui.horizontal(|ui| { 
      ui.add_space(5.0);
      ui.checkbox(&mut self.show_warden_mapper, super::create_text("Show Mapper in live splitter"));
    });

    ui.horizontal(|ui| { 
      ui.add_space(5.0);
      ui.checkbox(&mut self.show_objective_items, super::create_text("Show objective items in live splitter"));
    });

    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.checkbox(&mut self.show_code_guess, super::create_text("Show code guess"));
    });
    
    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(super::create_text("Code guess number of lines: "));
      if ui.add(egui::TextEdit::singleline(&mut self.text_inputs[4])
        .desired_width(100.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          if let Ok(x) = self.text_inputs[4].parse::<usize>() {
            self.code_guess_line_count = x;
          }
        };
    });
    
    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(super::create_text("Code guess number of words per line: "));
      if ui.add(egui::TextEdit::singleline(&mut self.text_inputs[5])
        .desired_width(100.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          if let Ok(x) = self.text_inputs[5].parse::<usize>() {
            self.code_guess_line_width = x;
          }
        };
    });

    ui.separator();
    ui.add_space(10.0);

    ui.horizontal(|ui| { 
      ui.add_space(5.0);
      ui.checkbox(&mut self.automatic_loading, super::create_text("Automatic Loading of Runs"));
    });
  
  }

  pub fn save_settings(&self) {

    let mut s = String::new();

    s.push_str(&format!("x_pos: {}\n", self.live_rectangle.left()));
    s.push_str(&format!("y_pos: {}\n", self.live_rectangle.top()));
    s.push_str(&format!("x_size: {}\n", self.live_rectangle.right() - self.live_rectangle.left()));
    s.push_str(&format!("automatic_loading: {}\n", self.automatic_loading));
    s.push_str(&format!("compare_to_record: {}\n", self.compare_to_record));
    s.push_str(&format!("compare_to_theoretical: {}\n", self.compare_to_theoretical));
    s.push_str(&format!("show_warden_mapper: {}\n", self.show_warden_mapper));
    s.push_str(&format!("show_objective_items: {}\n", self.show_objective_items));
    s.push_str(&format!("show_code_guess: {}\n", self.show_code_guess));
    s.push_str(&format!("code_guess_line_count: {}\n", self.code_guess_line_count));
    s.push_str(&format!("code_guess_line_width: {}\n", self.code_guess_line_width));

    let path = Path::new(env!("HOME")).join("Appdata\\Locallow\\Tgb03\\GTFO Logger\\app.properties");
    
    let _ = std::fs::write(path, s);

  }

}
