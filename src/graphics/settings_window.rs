
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use directories::ProjectDirs;
use egui::{Color32, Rect, Ui};

pub struct SettingsWindow {

  show_splitter: bool,
  show_game_splitter: bool,
  splitter_length: usize,
  game_splitter_length: usize,

  live_rectangle: Rect,
  automatic_loading: bool,
  compare_to_record: bool,
  compare_to_theoretical: bool,

  show_warden_mapper: bool,
  show_objective_items: bool,

  show_code_guess: bool,
  code_guess_line_count: usize,
  code_guess_line_width: usize,
  logs_folder: PathBuf,

  text_inputs: [String; 9],

}

impl Default for SettingsWindow {

  fn default() -> Self {
    let path = Self::config_path();
    let file_str: Option<String> = path.map(|path| {
      match File::open(path.join("app.properties")) {
        Ok(mut file) => {
          let mut buffer = String::new();
          let _ = file.read_to_string(&mut buffer);
          buffer
        },
        Err(_) => String::new(),
      }
    });

    let props: HashMap<String, String> = file_str.map_or(HashMap::default(), |file_str| {
      match serde_yaml::from_str(&file_str) {
        Ok(map) => map,
        Err(_) => HashMap::new(),
      }
    });

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
    let splitter_length = match props.get("splitter_length") {
      Some(s) => s.parse::<usize>().unwrap_or(100),
      None => 100,
    };
    let show_game_splitter = match props.get("show_game_splitter") {
      Some(s) => s.parse::<bool>().unwrap_or(false),
      None => false,
    };
    let game_splitter_length = match props.get("game_splitter_length") {
      Some(s) => s.parse::<usize>().unwrap_or(5),
      None => 5,
    };
    let logs_folder = match props.get("logs_folder") {
      Some(s) => PathBuf::from(s),
      None => {
        match Self::logs_path() {
          Some(path) => path,
          None => panic!("There is no home folder."),
        }
      }
    };
    let show_splitter = match props.get("show_splitter") {
      Some(s) => s.parse::<bool>().unwrap_or(true),
      None => true,
    };
    
    let live_rectangle = Rect { 
      min: [x_pos, y_pos].into(), 
      max: [x_pos + x_size, y_pos + 80.0].into() 
    };

    Self { 
      show_splitter,
      show_game_splitter,
      splitter_length,
      live_rectangle,
      automatic_loading,
      compare_to_record,
      compare_to_theoretical,
      show_warden_mapper,
      show_objective_items,
      show_code_guess,
      code_guess_line_count,
      code_guess_line_width,
      logs_folder: logs_folder.clone(),
      game_splitter_length,

      text_inputs: [
        x_pos.to_string(),
        y_pos.to_string(),
        x_size.to_string(),
        80.to_string(),
        code_guess_line_count.to_string(),
        code_guess_line_width.to_string(),
        logs_folder.to_str().map_or(String::new(), |s| s.to_owned()),
        splitter_length.to_string(),
        game_splitter_length.to_string(),
      ],
    }
  }
}

impl SettingsWindow {

  fn config_path() -> Option<PathBuf> {
    
    if let Some(proj_dirs) = ProjectDirs::from("com", "Tgb03", "GTFO Logger") {
      return Some(proj_dirs.config_dir().to_path_buf());
    }

    None
  }

  fn logs_path() -> Option<PathBuf> {
    if let Some(dirs) = directories::UserDirs::new() {
      return Some(dirs.home_dir().to_path_buf().join("AppData\\LocalLow\\10 Chambers Collective\\GTFO"));
    }

    None
  }

  pub fn get_show_game_splitter(&self) -> bool {
    self.show_game_splitter
  }

  pub fn get_show_splitter(&self) -> bool {
    self.show_splitter
  }

  pub fn get_splitter_length(&self) -> usize {
    self.splitter_length
  }

  pub fn get_game_splitter_length(&self) -> usize {
    self.game_splitter_length
  }

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

  pub fn get_logs_folder(&self) -> &PathBuf {
    &self.logs_folder
  }

  pub fn show(&mut self, ui: &mut Ui) {

    ui.add(egui::Label::new(super::create_text("LiveSplitter settings: ")
      .size(14.0)));

    ui.add_space(10.0);

    ui.horizontal(|ui| { 
      ui.add_space(5.0);
      ui.checkbox(&mut self.show_splitter, super::create_text("Show Actual Splits"));
      ui.add_space(5.0);
      ui.label(super::create_text("Warning: this disables completely the splits part."));
    });

    ui.horizontal(|ui| { 
      ui.add_space(5.0);
      ui.checkbox(&mut self.show_game_splitter, super::create_text("Show Game Splitter"));
      ui.add_space(5.0);
    });
    
    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(super::create_text("Path to logs folder: "));
      ui.add(egui::TextEdit::singleline(&mut self.text_inputs[6])
        .desired_width(512.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE));
    });

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

    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(super::create_text("Splitter max length"));
      if ui.add(egui::TextEdit::singleline(&mut self.text_inputs[7])
        .desired_width(100.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          if let Ok(x) = self.text_inputs[7].parse::<usize>() {
            self.splitter_length = x;
          }
        };
    });

    ui.horizontal(|ui| {
      ui.add_space(5.0);
      ui.monospace(super::create_text("Game splitter max length"));
      if ui.add(egui::TextEdit::singleline(&mut self.text_inputs[8])
        .desired_width(100.0)
        .background_color(Color32::from_rgb(32, 32, 32))
        .text_color(Color32::WHITE))
        .changed() {
          if let Ok(x) = self.text_inputs[8].parse::<usize>() {
            self.game_splitter_length = x;
          }
        };
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

    ui.separator();
    ui.add_space(10.0);
    
    ui.label(super::create_text(format!("App version: {}", env!("CARGO_PKG_VERSION"))));
    ui.label(super::create_text(format!("Made by Tgb03")));

  }

  pub fn save_settings(&self) {

    let mut s = String::new();

    s.push_str(&format!("x_pos: {}\n", self.live_rectangle.left()));
    s.push_str(&format!("y_pos: {}\n", self.live_rectangle.top()));
    s.push_str(&format!("x_size: {}\n", self.live_rectangle.width()));
    s.push_str(&format!("automatic_loading: {}\n", self.automatic_loading));
    s.push_str(&format!("compare_to_record: {}\n", self.compare_to_record));
    s.push_str(&format!("compare_to_theoretical: {}\n", self.compare_to_theoretical));
    s.push_str(&format!("show_warden_mapper: {}\n", self.show_warden_mapper));
    s.push_str(&format!("show_objective_items: {}\n", self.show_objective_items));
    s.push_str(&format!("show_code_guess: {}\n", self.show_code_guess));
    s.push_str(&format!("code_guess_line_count: {}\n", self.code_guess_line_count));
    s.push_str(&format!("code_guess_line_width: {}\n", self.code_guess_line_width));
    s.push_str(&format!("logs_folder: {}\n", self.logs_folder.to_str().unwrap_or_default()));
    s.push_str(&format!("show_splitter: {}\n", self.show_splitter));
    s.push_str(&format!("splitter_length: {}\n", self.splitter_length));
    s.push_str(&format!("show_game_splitter: {}\n", self.show_game_splitter));

    if let Some(path) = Self::config_path() {
      
      if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
      }

      if let Err(e) = std::fs::write(path.join("app.properties"), s) {
        eprintln!("{}", e);
      }
    
    } else {

      eprintln!("Failed to save file since config path is not set.")

    }

  }

}
